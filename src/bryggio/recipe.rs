use crate::beerxml;
use crate::bryggio;
use crate::bryggio::process;
use crate::units::*;
use crate::utils;
use brew_calculator;
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
    yeasts: Vec<bryggio::Yeast>,
    waters: Vec<bryggio::Water>,
    miscs: Vec<bryggio::Misc>,
    mash: process::Mash,
    boil: process::Boil,
    fermentation: process::Fermentation,
    carbonation: process::Carbonation,
    notes: Option<String>,
    taste_notes: Option<String>,
    taste_rating: Option<f32>,
    pub(crate) date: Option<String>,
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
            mash: process::Mash {},
            boil: process::Boil {
                volume: beerxml_recipe.boil_size,
            },
            fermentation: process::Fermentation {},
            carbonation: process::Carbonation {},
            notes: beerxml_recipe.notes,
            taste_notes: beerxml_recipe.taste_notes,
            taste_rating: beerxml_recipe.og,
            date: beerxml_recipe.date,
        }
    }
}

impl Recipe {
    pub fn hops(&self) -> std::slice::Iter<bryggio::Hop> {
        self.hops.iter()
    }

    pub fn fermentables(&self) -> std::slice::Iter<bryggio::Fermentable> {
        self.fermentables.iter()
    }

    pub fn yeasts(&self) -> std::slice::Iter<bryggio::Yeast> {
        self.yeasts.iter()
    }

    pub fn waters(&self) -> std::slice::Iter<bryggio::Water> {
        self.waters.iter()
    }

    pub fn miscs(&self) -> std::slice::Iter<bryggio::Misc> {
        self.miscs.iter()
    }

    pub fn ibu(&self) -> Ibu {
        // TODO: This should be calculated
        let tmp_gravity = 1.05;

        self.hops()
            .filter(|hop| hop.bittering())
            .fold(0.0, |acc, hop| {
                acc + brew_calculator::ibu::ibu(
                    hop.amount,
                    hop.alpha,
                    self.boil.volume,
                    hop.time,
                    tmp_gravity,
                )
            })
    }
}
