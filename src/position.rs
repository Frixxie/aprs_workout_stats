use geo::Point;
use serde::{Deserialize, Serialize};

pub fn parse_degrees<'de, D>(deserializer: D) -> Result<f64, D::Error>
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
