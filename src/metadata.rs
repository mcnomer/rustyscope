use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use regex::Regex;

#[derive(Debug)]
pub enum MetadataValue {
    Integer(i64),
    Float(f64),
    // IntegerWithUnits(i64, String),
    // FloatWithUnits(f64, String),
    String(String),
}

impl MetadataValue {
    pub fn to_string(&self) -> String {
        match self {
            MetadataValue::Integer(i) => i.to_string(),
            MetadataValue::Float(x) => x.to_string(),
            MetadataValue::String(s) => s.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub data: HashMap<String, MetadataValue>,
}

fn parse_value(s: &str) -> MetadataValue {
    if let Ok(x) = s.parse::<i64>() {
        return MetadataValue::Integer(x);
    };
    if let Ok(x) = s.parse::<f64>() {
        return MetadataValue::Float(x);
    };
    // if let Ok(x) = s.parse::<i64>() {
    //     return MetadataValue::Integer(x);
    // };
    // if let Ok(x) = s.parse::<i64>() {
    //     return MetadataValue::Integer(x);
    // };
    MetadataValue::String(s.to_owned())
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            data: HashMap::new(),
        }
    }

    pub fn insert_line(&mut self, line: &str) -> std::io::Result<()> {
        let (k, v) = line.split_once(": ").ok_or(Error::new(
            ErrorKind::Other,
            "error inserting line - can't split",
        ))?;
        let parsed_value = parse_value(v);
        self.data.insert(k.to_lowercase().to_owned(), parsed_value);
        Ok(())
    }

    pub fn get_int(&self, key: &str, regex_option: Option<&Regex>) -> Result<i64, String> {
        let value = self
            .data
            .get(key)
            .ok_or(format!("Rustyscope Error couldn't find '{}' entry", key))?;
        match value {
            MetadataValue::Integer(x) => Ok(*x),
            MetadataValue::Float(x) => Err(format!(
                "Rustyscope error parsing '{}': '{}' should be an integer not a float.",
                key,
                x.to_string()
            )),
            MetadataValue::String(s) => {
                let err_str = format!(
                    "Rustyscope error parsing '{}': '{}' was in an unexpected format.",
                    key, s
                );
                if let Some(re) = regex_option {
                    let caps = re.captures(s).ok_or(&err_str)?;
                    let x_str = caps.get(1).ok_or(&err_str)?.as_str();
                    x_str.parse::<i64>().map_err(|err| err.to_string())
                } else {
                    s.parse::<i64>().map_err(|err| err.to_string())
                }
            }
        }
    }

    pub fn get_float(&self, key: &str, regex_option: Option<&Regex>) -> Result<f64, String> {
        let value = self
            .data
            .get(key)
            .ok_or(format!("Rustyscope Error couldn't find '{}' entry", key))?;
        match value {
            MetadataValue::Integer(x) => Ok(*x as f64),
            MetadataValue::Float(x) => Ok(*x),
            MetadataValue::String(s) => {
                let err_str = format!(
                    "Rustyscope error parsing '{}': '{}' was in an unexpected format.",
                    key, s
                );
                if let Some(re) = regex_option {
                    let caps = re.captures(s).ok_or(&err_str)?;
                    let x_str = caps.get(1).ok_or(&err_str)?.as_str();
                    x_str.parse::<f64>().map_err(|err| err.to_string())
                } else {
                    s.parse::<f64>().map_err(|err| err.to_string())
                }
            }
        }
    }
}
