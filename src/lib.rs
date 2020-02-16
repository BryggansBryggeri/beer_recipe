//! # Beer recipe
//!
//! A library for handling beer recipes
//!
//! ## BeerXML
//!
//! The starting point is the BeerXML standard:
//!
//! [http://www.beerxml.com/beerxml.htm](http://www.beerxml.com/beerxml.htm)
use serde;
use serde::de::DeserializeOwned;
use serde::Deserialize;
pub mod beerxml;
pub mod bryggio;
mod units;
mod utils;

pub trait RecordSet {}

/// Dynamic renaming of `hop` not working
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct RecordSetList<T>
where
    T: RecordSet + DeserializeOwned,
{
    #[serde(bound(deserialize = "Vec<T>: Deserialize<'de>"))]
    pub hop: Vec<T>,
}
