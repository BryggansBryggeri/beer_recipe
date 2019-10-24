use crate::equipment::Equipment;
use crate::fermentable::Fermentable;
use crate::hop::Hop;
use crate::mash_step::MashStep;
use crate::misc::Misc;
use crate::style::Style;
use crate::water::Water;
use crate::yeast::Yeast;
use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Recipe {
    name: String,
    version: u8,
    #[serde(rename = "TYPE")]
    type_: Type,
    style: Style,
    equipment: Equipment,
    brewer: String,
    asst_brewer: Option<String>,
    batch_size: f32,
    boil_time: f32,
    /// Not used for `Type::Extract`
    efficiency: f32,
    hops: Vec<Hop>,
    fermentables: Vec<Fermentable>,
    miscs: Vec<Misc>,
    yeasts: Vec<Yeast>,
    waters: Vec<Water>,
    mash: Vec<MashStep>,
    notes: Option<String>,
    taste_notes: Option<String>,
    tast_rating: Option<f32>,
    og: Option<f32>,
    fg: Option<f32>,
    fermentation_stages: Option<u8>,
    primary_age: Option<f32>,
    primary_temp: Option<f32>,
    secondary_age: Option<f32>,
    secondary_temp: Option<f32>,
    tertiary_age: Option<f32>,
    tertiary_temp: Option<f32>,
    age: Option<f32>,
    temp: Option<f32>,
    date: Option<String>,
    carbonation: Option<f32>,
    forced_carbonation: Option<bool>,
    priming_sugar_name: Option<String>,
    carbonation_temp: Option<f32>,
    priming_sugar_equiv: Option<f32>,
    keg_priming_factor: Option<f32>,
}

#[derive(Deserialize, Debug)]
enum Type {
    Extract,
    PartialMash,
    AllGrain,
}

#[cfg(test)]
mod tests {
    //use super::*;
    //use serde_xml_rs;
    #[test]
    fn dummy() {
        assert_eq!(2, 1 + 1);
    }
}
