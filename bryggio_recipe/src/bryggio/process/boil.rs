use brew_calculator::units::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Boil {
    pub pre_volume: Liters,
    pub boil_time: Minutes,
}

impl Boil {
    pub(crate) fn from_beerxml_recipe(boil_size: Liters, boil_time: Minutes) -> Self {
        Self {
            pre_volume: boil_size,
            boil_time,
        }
    }
}
