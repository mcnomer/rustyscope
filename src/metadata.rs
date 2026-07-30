use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

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

    pub fn get_int<T>(&self, key: &str, regex_option: Option<&Regex>) -> Result<T, String>
    where
        T: TryFrom<i64>,
        <T as TryFrom<i64>>::Error: Display,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        let value = self
            .data
            .get(key)
            .ok_or_else(|| format!("Rustyscope Error couldn't find '{}' entry", key))?;
        match value {
            MetadataValue::Integer(x) => {
                T::try_from(*x).map_err(|e| format!("Rustscope error parsing '{key}': {e}"))
            }
            MetadataValue::Float(x) => Err(format!(
                "Rustyscope error parsing '{}': '{}' should be an integer not a float.",
                key,
                x.to_string()
            )),
            MetadataValue::String(s) => {
                let str_to_parse = match regex_option {
                    Some(re) => re
                        .captures(s)
                        .and_then(|caps| caps.get(1))
                        .map(|x| x.as_str())
                        .ok_or_else(|| format!("Rustyscope error parsing '{key}': '{s}' was in an unexpected format."))?,
                    None => s.as_str()
                };
                str_to_parse.parse::<T>().map_err(|err| err.to_string())
            }
        }
    }

    pub fn get_float<T>(&self, key: &str, regex_option: Option<&Regex>) -> Result<T, String>
    where
        T: TryFrom<f64>,
        <T as TryFrom<f64>>::Error: Display,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        let value = self
            .data
            .get(key)
            .ok_or_else(|| format!("Rustyscope Error couldn't find '{}' entry", key))?;
        match value {
            MetadataValue::Integer(x) => {
                T::try_from(*x as f64).map_err(|e| format!("Rustscope error parsing '{key}': {e}"))
            }
            MetadataValue::Float(x) => {
                T::try_from(*x).map_err(|e| format!("Rustscope error parsing '{key}': {e}"))
            }
            MetadataValue::String(s) => {
                let str_to_parse = match regex_option {
                    Some(re) => re
                        .captures(s)
                        .and_then(|caps| caps.get(1))
                        .map(|x| x.as_str())
                        .ok_or_else(|| format!("Rustyscope error parsing '{key}': '{s}' was in an unexpected format."))?,
                    None => s.as_str()
                };
                str_to_parse.parse::<T>().map_err(|err| err.to_string())
            }
        }
    }

    pub fn get_string(&self, key: &str, regex_option: Option<&Regex>) -> Result<String, String> {
        let value = self
            .data
            .get(key)
            .ok_or_else(|| format!("Rustyscope Error couldn't find '{}' entry", key))?;
        match value {
            MetadataValue::Integer(x) => Ok(x.to_string()),
            MetadataValue::Float(x) => Ok(x.to_string()),
            MetadataValue::String(s) => Ok(match regex_option {
                Some(re) => re
                    .captures(s)
                    .and_then(|caps| caps.get(1))
                    .map(|x| x.as_str().to_owned())
                    .ok_or_else(|| {
                        format!(
                            "Rustyscope error parsing '{key}': '{s}' was in an unexpected format."
                        )
                    })?,
                None => s.to_owned(),
            }),
        }
    }
}
