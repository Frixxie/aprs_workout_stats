use core::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AprsTypeError {
    pub message: String,
}

impl Display for AprsTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AprsTypeError: {}", self.message)
    }
}

impl std::error::Error for AprsTypeError {}

#[derive(Debug, Serialize, Deserialize)]
pub enum AprsType {
    Location,
    Message,
    Weather,
}

impl From<AprsType> for String {
    fn from(t: AprsType) -> String {
        match t {
            AprsType::Location => "l".to_string(),
            AprsType::Message => "msg".to_string(),
            AprsType::Weather => "wx".to_string(),
        }
    }
}

impl TryFrom<String> for AprsType {
    type Error = AprsTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "l" => Ok(AprsType::Location),
            "msg" => Ok(AprsType::Message),
            "wx" => Ok(AprsType::Weather),
            _ => Err(AprsTypeError {
                message: format!("Unknown type: {}", value),
            }),
        }
    }
}

pub fn into_aprs_type<'de, D>(deserializer: D) -> Result<AprsType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    AprsType::try_from(s).map_err(serde::de::Error::custom)
}
