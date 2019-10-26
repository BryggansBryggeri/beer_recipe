use serde;
use serde::de::DeserializeOwned;
use serde::Deserialize;
mod equipment;
mod fermentable;
pub mod hop;
mod mash;
mod misc;
mod recipe;
mod style;
mod utils;
mod water;
mod yeast;

pub type Percent = f32;
pub type PartsPerMillion = f32;
pub type PH = f32;
pub type SpecificGravity = f32;
pub type IBU = f32;
pub type SRMColor = f32;
pub type VolumesCO2 = f32;
pub type ABV = Percent;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
