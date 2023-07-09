use core::fmt;
use std::{
    fmt::{Display, Formatter},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{
    aprs_type::{into_aprs_type, AprsType},
    position::Position,
    utils::deserialize_time,
};

pub type Callsign = String;

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
