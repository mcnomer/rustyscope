use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum MetadataValue {
    Integer(i64),
    Float(f64),
    // IntegerWithUnits(i64, String),
    // FloatWithUnits(f64, String),
    String(String),
}

impl Display for MetadataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataValue::Integer(x) => write!(f, "{x}"),
            MetadataValue::Float(x) => write!(f, "{x}"),
            MetadataValue::String(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone)]
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
    MetadataValue::String(s.to_owned())
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<&MetadataValue, String> {
        self.data
            .get(key)
            .ok_or_else(|| format!("Rustyscope Error: couldn't find '{}' entry", key))
    }

    pub fn insert_line(&mut self, line: &str) -> std::io::Result<()> {
        let (k, v) = line.split_once(": ").ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                "error inserting line - can't split without delimiter (: )",
            )
        })?;
        let parsed_value = parse_value(v);
        self.data.insert(k.to_lowercase().to_owned(), parsed_value);
        Ok(())
    }

    fn regex_extract<'a>(
        &self,
        key: &str,
        s: &'a str,
        regex_option: Option<&Regex>,
    ) -> Result<&'a str, String> {
        match regex_option {
            Some(re) => re
                .captures(s)
                .and_then(|caps| caps.get(1))
                .map(|x| x.as_str())
                .ok_or_else(|| {
                    format!("Rustyscope error parsing '{key}': '{s}' was in an unexpected format.")
                }),
            None => Ok(s),
        }
    }

    pub fn get_int<T>(&self, key: &str, regex_option: Option<&Regex>) -> Result<T, String>
    where
        T: TryFrom<i64>,
        <T as TryFrom<i64>>::Error: Display,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        let value = self.get(key)?;
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
                let str_to_parse = self.regex_extract(key, s, regex_option)?;
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
        let value = self.get(key)?;
        match value {
            MetadataValue::Integer(x) => {
                T::try_from(*x as f64).map_err(|e| format!("Rustscope error parsing '{key}': {e}"))
            }
            MetadataValue::Float(x) => {
                T::try_from(*x).map_err(|e| format!("Rustscope error parsing '{key}': {e}"))
            }
            MetadataValue::String(s) => {
                let str_to_parse = self.regex_extract(key, s, regex_option)?;
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
            MetadataValue::String(s) => self
                .regex_extract(key, s, regex_option)
                .map(|string| string.to_string()),
        }
    }
}
