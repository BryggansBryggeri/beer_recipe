use serde::Deserialize;
#[derive(Deserialize, Debug, PartialEq)]
pub struct Boil {
    pub volume: f32,
}
