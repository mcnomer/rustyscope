use crate::metadata::Metadata;
use regex::regex;

#[derive(Debug)]
pub struct Channel {
    // name: String,
    pub metadata: Metadata,
}

impl Channel {
    pub fn get_byte_offset(&self) -> Result<usize, String> {
        self.metadata
            .get_int("data offset", None)
            .map(|x| x as usize)
    }

    pub fn get_byte_length(&self) -> Result<usize, String> {
        self.metadata
            .get_int("data length", None)
            .map(|x| x as usize)
    }

    pub fn get_v_per_lsb(&self, key: &str) -> Result<f64, String> {
        self.metadata
            .get_float(key, Some(regex!(r"\(([-+]?(?:\d*\.?\d+)) V\/LSB")))
    }
    pub fn get_lsb_scale(&self) -> Result<f64, String> {
        self.metadata.get_float("z lsb scale", None)
    }
}
