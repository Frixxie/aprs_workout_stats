use std::time::Duration;

use serde::Deserialize;

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let secs = s.parse::<u64>().map_err(serde::de::Error::custom)?;
    Ok(Duration::new(secs, 0))
}
