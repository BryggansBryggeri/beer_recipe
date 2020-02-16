use crate::beerxml;
use crate::bryggio;
use crate::bryggio::process;
use crate::units::*;
use crate::utils;
use serde;
use serde::Deserialize;
use std::convert::From;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Recipe {
    pub name: String,
    #[serde(rename = "TYPE")]
    type_: bryggio::Type,
    style: bryggio::Style,
    brewer: String,
    asst_brewer: Option<String>,
    equipment: Option<bryggio::Equipment>,
    pub batch_size: Liters,
    og: Option<f32>,
    fg: Option<f32>,
    /// Not used for `Type::Extract`
    efficiency: f32,
    hops: Vec<bryggio::Hop>,
    fermentables: Vec<bryggio::Fermentable>,
    miscs: Vec<bryggio::Misc>,
    yeasts: Vec<bryggio::Yeast>,
    waters: Vec<bryggio::Water>,
    boil: process::Boil,
    mash: process::Mash,
    fermentation: process::Fermentation,
    notes: Option<String>,
    taste_notes: Option<String>,
    taste_rating: Option<f32>,
    pub(crate) date: Option<String>,
    pub(crate) carbonation: Option<VolumesCO2>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    pub(crate) forced_carbonation: Option<bool>,
    pub(crate) priming_sugar_name: Option<String>,
    pub(crate) carbonation_temp: Option<Celsius>,
    pub(crate) priming_sugar_equiv: Option<f32>,
    pub(crate) keg_priming_factor: Option<f32>,
}

impl From<beerxml::Recipe> for Recipe {
    fn from(beerxml_recipe: beerxml::Recipe) -> Self {
        Recipe {
            name: beerxml_recipe.name,
            type_: beerxml_recipe.type_,
            style: beerxml_recipe.style,
            brewer: beerxml_recipe.brewer,
            asst_brewer: beerxml_recipe.asst_brewer,
            equipment: beerxml_recipe.equipment,
            batch_size: beerxml_recipe.batch_size,
            og: beerxml_recipe.og,
            fg: beerxml_recipe.fg,
            efficiency: beerxml_recipe.efficiency,
            hops: beerxml_recipe.hops.hop,
            fermentables: beerxml_recipe.fermentables.fermentable,
            miscs: beerxml_recipe.miscs.misc,
            yeasts: beerxml_recipe.yeasts.yeast,
            waters: beerxml_recipe.waters.water,
            boil: process::Boil {},
            mash: process::Mash {},
            fermentation: process::Fermentation {},
            notes: beerxml_recipe.notes,
            taste_notes: beerxml_recipe.taste_notes,
            taste_rating: beerxml_recipe.og,
            date: beerxml_recipe.date,
            carbonation: beerxml_recipe.carbonation,
            forced_carbonation: beerxml_recipe.forced_carbonation,
            priming_sugar_name: beerxml_recipe.priming_sugar_name,
            carbonation_temp: beerxml_recipe.carbonation_temp,
            priming_sugar_equiv: beerxml_recipe.priming_sugar_equiv,
            keg_priming_factor: beerxml_recipe.keg_priming_factor,
        }
    }
}

impl Recipe {
    pub fn hops(&self) -> std::slice::Iter<beerxml::Hop> {
        self.hops.iter()
    }
}
