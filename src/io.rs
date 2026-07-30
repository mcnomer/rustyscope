use crate::metadata::Metadata;
use crate::scandata::Channel;
use ordered_hash_map::OrderedHashMap;
use regex::regex;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

const HEADER_LINE_PREFIX: char = '\\';
const HEADER_SECTION_PREFIX: char = '*';
const HEADER_END_STR: &str = "*file list end";
const MIN_FILE_SIZE: usize = 32;

const H_SCALE_KEY: &str = "@2:z scale zsensor";
const H_SENS_KEY: &str = "@sens. zsensor";
const X_SCALE_KEY: &str = "@2:z scale y scan";
const X_SENS_KEY: &str = "@sens. ypiezo";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum HeaderSection {
    FileMetadata,
    ScannerMetadata,
    EquipmentMetadata,
    HSDCMetadata,
    MiscMetadata,
    EngageMetadata,
    SweepMetadata,
    Channels(usize),
    Other(String),
}

impl Display for HeaderSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderSection::FileMetadata => write!(f, "File Metadata"),
            HeaderSection::ScannerMetadata => write!(f, "Scanner Metadata"),
            HeaderSection::EquipmentMetadata => write!(f, "Equipment Metadata"),
            HeaderSection::HSDCMetadata => write!(f, "HSDC Metadata"),
            HeaderSection::MiscMetadata => write!(f, "Misc Metadata"),
            HeaderSection::EngageMetadata => write!(f, "Engage Metadata"),
            HeaderSection::SweepMetadata => write!(f, "Sweep Metadata"),
            HeaderSection::Channels(i) => write!(f, "Channel {i}"),
            HeaderSection::Other(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug)]
struct Header {
    sections: OrderedHashMap<HeaderSection, Metadata>,
}

#[derive(Debug)]
pub struct NanoscopeFile {
    // buffer: Vec<u8>,
    pub file_metadata: Metadata,
    pub scanner_metadata: Metadata,
    pub equipment_metadata: Option<Metadata>,
    pub hdsc_metadata: Option<Metadata>,
    pub misc_metadata: Option<Metadata>,
    pub engage_metadata: Option<Metadata>,
    pub sweep_metadata: Option<Metadata>,
    pub channels: Vec<Channel>,
}

impl Header {
    fn get_section_and_consume(&mut self, section: HeaderSection) -> std::io::Result<Metadata> {
        self.sections.remove(&section).ok_or(Error::new(
            ErrorKind::Other,
            format!(
                "Rustyscope Error: Missing section '{}'",
                section.to_string()
            ),
        ))
    }
}

impl NanoscopeFile {
    pub fn load(file_path: &str) -> std::io::Result<NanoscopeFile> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut header = parse_header(&buffer)?;

        let file_metadata = header.get_section_and_consume(HeaderSection::FileMetadata)?;
        let scanner_metadata = header.get_section_and_consume(HeaderSection::ScannerMetadata)?;
        let mut equipment_metadata = None;
        let mut hdsc_metadata = None;
        let mut misc_metadata = None;
        let mut engage_metadata = None;
        let mut sweep_metadata = None;
        let mut channels = vec![];

        for (key, val) in header.sections.drain() {
            match key {
                HeaderSection::EquipmentMetadata => equipment_metadata = Some(val),
                HeaderSection::HSDCMetadata => hdsc_metadata = Some(val),
                HeaderSection::MiscMetadata => misc_metadata = Some(val),
                HeaderSection::EngageMetadata => engage_metadata = Some(val),
                HeaderSection::SweepMetadata => sweep_metadata = Some(val),
                HeaderSection::Channels(i) => {
                    channels.push(Channel::from_metadata(val, &buffer).map_err(|err| {
                        Error::new(
                            ErrorKind::Other,
                            format!("Rustyscope parsing channel {i}: {err}"),
                        )
                    })?);
                }
                _ => (),
            }
        }

        Ok(NanoscopeFile {
            file_metadata,
            scanner_metadata,
            equipment_metadata,
            hdsc_metadata,
            misc_metadata,
            engage_metadata,
            sweep_metadata,
            channels,
        })
    }

    pub fn get_scan_lines(&self) -> Result<Vec<(Vec<f64>, Vec<f64>)>, String> {
        let height = self.get_height_channel()?;
        let x = self.get_x_channel()?;

        let height_nm_per_v = self.get_axis_nm_per_v(&height.metadata, H_SENS_KEY)?;
        let height_v_per_lsb = self.get_v_per_lsb(&height.metadata, H_SCALE_KEY)?;
        let height_lsb_scale = self.get_lsb_scale(&height.metadata)?;

        let x_nm_per_v = self.get_axis_nm_per_v(&x.metadata, X_SENS_KEY)?;
        let x_v_per_lsb = self.get_v_per_lsb(&x.metadata, X_SCALE_KEY)?;
        let x_lsb_scale = self.get_lsb_scale(&x.metadata)?;

        let mut lines: Vec<(Vec<f64>, Vec<f64>)> = vec![];
        let height_scale = height_nm_per_v * height_v_per_lsb / height_lsb_scale;
        let x_scale = x_nm_per_v * x_v_per_lsb / x_lsb_scale;

        let min_length = min(height.data.len(), x.data.len());
        let mut off: usize = 0;
        while off < min_length {
            let line_length = x.get_data_num(off)?;
            let line_height = height.get_data_range(off + 1..off + line_length as usize)?;
            let line_x = x.get_data_range(off + 1..off + line_length as usize)?;

            let scaled_line_height: Vec<f64> = line_height
                .iter()
                .map(|&x| x as f64 * height_scale)
                .collect();
            let scaled_line_x: Vec<f64> = line_x.iter().map(|&x| x as f64 * x_scale).collect();

            lines.push((scaled_line_x, scaled_line_height));
            off += line_length as usize + 1;
        }
        Ok(lines)
    }

    fn get_height_channel(&self) -> Result<&Channel, String> {
        self.channels
            .get(0)
            .ok_or_else(|| format!("Rustyscope Error: couldn't get channel 0 (Z height)."))
    }

    fn get_x_channel(&self) -> Result<&Channel, String> {
        self.channels
            .get(0)
            .ok_or_else(|| format!("Rustyscope Error: couldn't get channel 1 (Y scan)."))
    }

    fn get_v_per_lsb(&self, metadata: &Metadata, key: &str) -> Result<f64, String> {
        metadata.get_float(key, Some(regex!(r"\(([-+]?(?:\d*\.?\d+)) V\/LSB")))
    }
    fn get_lsb_scale(&self, metadata: &Metadata) -> Result<f64, String> {
        metadata.get_float("z lsb scale", None)
    }

    fn get_axis_nm_per_v(&self, metadata: &Metadata, key: &str) -> Result<f64, String> {
        metadata.get_float(key, Some(regex!(r"([-+]?(?:\d*\.?\d+)) nm\/V")))
    }
}

fn parse_header(buffer: &[u8]) -> std::io::Result<Header> {
    let mut sections: OrderedHashMap<HeaderSection, Metadata> = OrderedHashMap::new();

    let mut current_section: Option<HeaderSection> = None;
    let mut channel_idx = 0;

    if buffer.len() < MIN_FILE_SIZE {
        return Err(Error::new(ErrorKind::Other, "File header too short."));
    };

    let lines = filter_valid_header_lines(split_lines(buffer));

    for line in lines {
        if line.starts_with(HEADER_SECTION_PREFIX) {
            let section_name = line[1..].to_ascii_lowercase();

            if let Some(HeaderSection::Channels(_)) = current_section {
                channel_idx += 1;
            }

            current_section = match section_name.as_str() {
                "file list" => Some(HeaderSection::FileMetadata),
                "scanner list" => Some(HeaderSection::ScannerMetadata),
                "ciao scan list" => Some(HeaderSection::ScannerMetadata),
                "equipment list" => Some(HeaderSection::EquipmentMetadata),
                "hsdc list" => Some(HeaderSection::HSDCMetadata),
                "misc. data list" => Some(HeaderSection::MiscMetadata),
                "engage list" => Some(HeaderSection::EngageMetadata),
                "sweep list" => Some(HeaderSection::SweepMetadata),
                "ciao image list" => Some(HeaderSection::Channels(channel_idx)),
                _ => Some(HeaderSection::Other(section_name.clone())),
            };
        } else {
            if let Some(section) = current_section.clone() {
                let metadata = sections.entry(section).or_insert(Metadata::new());
                metadata.insert_line(line)?;
            }
        }
    }

    Ok(Header { sections })
}

fn split_lines(buffer: &[u8]) -> Vec<&[u8]> {
    let mut lines: Vec<&[u8]> = vec![];
    let mut start_offset: Option<usize> = Some(0);

    for i in 0..buffer.len() {
        if (buffer[i] == '\r' as u8) || (buffer[i] == '\n' as u8) {
            if let Some(off) = start_offset {
                lines.push(&buffer[off..i]);
                start_offset = None;
            }
        } else {
            if start_offset.is_none() {
                start_offset = Some(i);
            }
        }
    }
    return lines;
}

fn filter_valid_header_lines(lines: Vec<&[u8]>) -> Vec<&str> {
    lines
        .iter()
        .filter(|&&line| {
            line.get(0)
                .is_some_and(|&prefix| prefix == HEADER_LINE_PREFIX as u8)
        })
        .filter_map(|&line| line.get(1..line.len()).and_then(|x| str::from_utf8(x).ok()))
        .take_while(|&line| line.to_ascii_lowercase() != HEADER_END_STR)
        .collect()
}
