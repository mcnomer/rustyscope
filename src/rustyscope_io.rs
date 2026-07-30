use crate::metadata::Metadata;
use crate::scandata::Channel;
use regex::regex;
use std::cmp::min;
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

#[derive(Debug)]
enum HeaderSection {
    FileMetadata,
    ScannerMetadata,
    Channels(usize),
}

#[derive(Debug)]
pub struct Header {
    pub file_metadata: Metadata,
    pub scanner_metadata: Metadata,
    pub channels: Vec<Channel>,
}

#[derive(Debug)]
pub struct NanoscopeFile {
    buffer: Vec<u8>,
    pub header: Header,
}

impl NanoscopeFile {
    pub fn load(file_path: &str) -> std::io::Result<NanoscopeFile> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let header = parse_header(&buffer)?;

        // let data: Vec<i16> = buffer[40960..40960 + 39898]

        Ok(NanoscopeFile { buffer, header })
    }

    pub fn get_channel_data(&self, channel_idx: usize) -> Result<Vec<i16>, String> {
        let channel: &Channel = self.header.channels.get(channel_idx).ok_or(format!(
            "Rustyscope Error: couldn't get channel {}.",
            channel_idx
        ))?;
        let offset = channel.get_byte_offset()?;
        let length = channel.get_byte_length()?;
        if length % 2 != 0 {
            return Err(format!(
                "Rustyscope Error: data length ({}) was not a multiple of 2.",
                length
            ));
        }
        Ok(self.buffer[offset..offset + length]
            .chunks_exact(2)
            .map(|x| i16::from_le_bytes([x[0], x[1]]))
            .collect())
    }

    pub fn get_scan_lines(&self) -> Result<Vec<(Vec<f64>, Vec<f64>)>, String> {
        let height_data = self
            .get_channel_data(0)
            .map_err(|err| err + "\n - Couldn't read Z height channel data.")?;
        let x_data = self
            .get_channel_data(1)
            .map_err(|err| err + "\n - Couldn't read Y scan channel data.")?;

        let height_channel = self
            .header
            .channels
            .get(0)
            .ok_or("Rustyscope Error: couldn't get channel 0 (Z height).")?;
        let x_channel = self
            .header
            .channels
            .get(1)
            .ok_or("Rustyscope Error: couldn't get channel 1 (Y scan).")?;

        let height_nm_per_v = self.get_axis_nm_per_v(H_SENS_KEY)?;
        let height_v_per_lsb = height_channel.get_v_per_lsb(H_SCALE_KEY)?;
        let height_lsb_scale = height_channel.get_lsb_scale()?;

        let x_nm_per_v = self.get_axis_nm_per_v(X_SENS_KEY)?;
        let x_v_per_lsb = x_channel.get_v_per_lsb(X_SCALE_KEY)?;
        let x_lsb_scale = x_channel.get_lsb_scale()?;

        let mut lines: Vec<(Vec<f64>, Vec<f64>)> = vec![];
        let height_scale = height_nm_per_v * height_v_per_lsb / height_lsb_scale;
        let x_scale = x_nm_per_v * x_v_per_lsb / x_lsb_scale;

        let min_length = min(height_data.len(), x_data.len());
        let mut off: usize = 0;
        while off < min_length {
            let line_length = *x_data.get(off).ok_or(format!(
                "Rustyscope Error: failed to read byte {} in line of length {} while parsing scan data. The file may be corrupted.",
                off,
                x_data.len()
            ))?;
            let line_height =
                height_data
                    .get(off + 1..off + line_length as usize)
                    .ok_or(format!(
                        "Rustyscope Error: failed to read bytes {}-{} in line of length {} while parsing scan's Z height data. The file may be corrupted.",
                        off+1, off+line_length as usize,
                        height_data.len()
                    ))?;
            let line_x = x_data
                .get(off + 1..off + line_length as usize)
                .ok_or(format!(
                        "Rustyscope Error: failed to read bytes {}-{} in line of length {} while parsing scan Y data. The file may be corrupted.",
                        off+1, off+line_length as usize,
                        x_data.len()
                    ))?;

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

    fn get_axis_nm_per_v(&self, key: &str) -> Result<f64, String> {
        self.header
            .scanner_metadata
            .get_float(key, Some(regex!(r"([-+]?(?:\d*\.?\d+)) nm\/V")))
    }
}

fn parse_header(buffer: &[u8]) -> std::io::Result<Header> {
    let mut header = Header {
        file_metadata: Metadata::new(),
        scanner_metadata: Metadata::new(),
        channels: vec![],
    };

    let mut current_section: Option<HeaderSection> = None;
    let mut channel_idx = 0;

    if buffer.len() < MIN_FILE_SIZE {
        return Err(Error::new(ErrorKind::Other, "File header too short."));
    };

    let lines = filter_valid_header_lines(split_lines(buffer));

    for line in lines {
        if line.starts_with(HEADER_SECTION_PREFIX) {
            if let Some(HeaderSection::Channels(_)) = current_section {
                channel_idx += 1;
            }
            current_section = match line[1..].to_ascii_lowercase().as_str() {
                "file list" => Some(HeaderSection::FileMetadata),
                "scanner list" => Some(HeaderSection::ScannerMetadata),
                "ciao image list" => Some(HeaderSection::Channels(channel_idx)),
                _ => None,
            };
            if let Some(HeaderSection::Channels(_)) = current_section {
                header.channels.push(Channel {
                    metadata: Metadata::new(),
                });
            }
        } else {
            let metadata = match current_section {
                Some(HeaderSection::FileMetadata) => Some(&mut header.file_metadata),
                Some(HeaderSection::ScannerMetadata) => Some(&mut header.scanner_metadata),
                Some(HeaderSection::Channels(i)) => {
                    header.channels.get_mut(i).map(|x| &mut x.metadata)
                }
                None => None,
            };
            if let Some(meta) = metadata {
                meta.insert_line(line).unwrap();
            }
        }
    }

    Ok(header)
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
