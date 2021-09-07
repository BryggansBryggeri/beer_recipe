use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub(crate) fn opt_bool_de_from_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_ascii_lowercase();
    let value = (bool::from_str(&s).map_err(de::Error::custom))?;
    Ok(Some(value))
}
