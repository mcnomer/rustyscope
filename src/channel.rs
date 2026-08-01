use regex::regex;

use crate::metadata::Metadata;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub metadata: Metadata,
    pub data: Vec<i16>,
}

impl Channel {
    pub fn from_metadata(metadata: Metadata, buffer: &[u8]) -> Result<Channel, String> {
        let data = get_channel_data(&metadata, buffer)?;
        let name = metadata.get_string("@2:image data", Some(regex!(r####"\"(.+)\""####)))?;
        Ok(Channel {
            name,
            metadata,
            data,
        })
    }

    pub fn get_data_range(&self, idx_range: Range<usize>) -> Result<&[i16], String> {
        let start = idx_range.start;
        let end = idx_range.end;
        self.data.get(idx_range)
            .ok_or_else(|| format!(
                "Rustyscope Error: failed to read bytes {}-{} in channel buffer of length {} while parsing scan's {} data. The file may be corrupted.",
                start, end, self.data.len(), self.name
            ))
    }

    pub fn get_data_num(&self, idx: usize) -> Result<i16, String> {
        self.data.get(idx)
        .ok_or_else(|| 
            format!("Rustyscope Error: failed to read byte {} in line of length {} while parsing scan data. The file may be corrupted.",
            idx, self.data.len()
        )).copied()
    }
}

fn get_channel_data(metadata: &Metadata, buffer: &[u8]) -> Result<Vec<i16>, String> {
    let offset: usize = metadata.get_int("data offset", None)?;
    let length: usize = metadata.get_int("data length", None)?;
    if length % 2 != 0 {
        return Err(format!(
            "Rustyscope Error: data length ({}) was not a multiple of 2.",
            length
        ));
    }
    let data = buffer.get(offset..offset + length).ok_or_else(|| {
        format!(
            "Rustyscope error getting data in range {}-{}",
            offset,
            offset + length
        )
    })?;
    Ok(data
        .chunks_exact(2)
        .map(|x| i16::from_le_bytes([x[0], x[1]]))
        .collect())
}
