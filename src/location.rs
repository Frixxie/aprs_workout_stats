use core::fmt;
use std::{
    fmt::{Display, Formatter},
    time::Duration,
};

use geo::Point;
use serde::{Deserialize, Serialize};

pub type Callsign = String;

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
    type Error = LocationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "l" => Ok(AprsType::Location),
            "msg" => Ok(AprsType::Message),
            "wx" => Ok(AprsType::Weather),
            _ => Err(LocationError {
                message: format!("Unknown type: {}", value),
            }),
        }
    }
}

fn into_aprs_type<'de, D>(deserializer: D) -> Result<AprsType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    AprsType::try_from(s).map_err(serde::de::Error::custom)
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let secs = s.parse::<u64>().map_err(serde::de::Error::custom)?;
    Ok(Duration::new(secs, 0))
}

#[derive(Debug)]
pub struct LocationError {
    pub message: String,
}

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocationError: {}", self.message)
    }
}

impl std::error::Error for LocationError {}

fn parse_degrees<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(deserialize_with = "parse_degrees")]
    lat: f64,
    #[serde(deserialize_with = "parse_degrees")]
    lng: f64,
}

impl From<Position> for Point<f64> {
    fn from(p: Position) -> Self {
        Point::new(p.lng, p.lat)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEntry {
    name: Callsign,
    #[serde(rename = "type", deserialize_with = "into_aprs_type")]
    _type: AprsType,
    #[serde(deserialize_with = "deserialize_time")]
    time: Duration,
    #[serde(rename = "lasttime", deserialize_with = "deserialize_time")]
    last_time: Duration,
    #[serde(flatten)]
    pos: Position,
    symbol: String,
    srccall: Callsign,
    dstcall: Callsign,
    phg: Option<String>,
    comment: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    command: String,
    result: String,
    what: String,
    found: u32,
    entries: Vec<LocationEntry>,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Location: {} entries", self.found)
    }
}
