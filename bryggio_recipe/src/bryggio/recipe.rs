use crate::bryggio;
use crate::bryggio::process;
use beerxml;
use brew_calculator::units::*;
use brew_calculator::{ibu, utils};
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
    pre_boil_gravity: Option<SpecificGravity>,
    og: Option<SpecificGravity>,
    fg: Option<SpecificGravity>,
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
    ibu_method: ibu::Method,
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
            // Both gravity measures should be inferred
            // from grain bill, efficiency et al.
            pre_boil_gravity: None,
            og: beerxml_recipe.og,
            fg: beerxml_recipe.fg,
            efficiency: beerxml_recipe.efficiency,
            hops: beerxml_recipe.hops.hop,
            fermentables: beerxml_recipe.fermentables.fermentable,
            miscs: beerxml_recipe.miscs.misc,
            yeasts: beerxml_recipe.yeasts.yeast,
            waters: beerxml_recipe.waters.water,
            mash: process::Mash {},
            boil: process::Boil::from_beerxml_recipe(
                beerxml_recipe.boil_size,
                beerxml_recipe.boil_time,
            ),
            fermentation: process::Fermentation {},
            carbonation: process::Carbonation {},
            notes: beerxml_recipe.notes,
            taste_notes: beerxml_recipe.taste_notes,
            taste_rating: beerxml_recipe.og,
            date: beerxml_recipe.date,
            ibu_method: beerxml_recipe
                .ibu_method
                .map_or(ibu::Method::Tinseth, |x| x.into()),
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

    /// Total IBU for recipe
    ///
    /// Calculates and sums the individual IBU contributions for all bittering hops.
    pub fn ibu(&self) -> Ibu {
        self.hops()
            .filter(|hop| hop.bittering())
            .fold(0.0, |acc, hop| {
                acc + ibu::ibu(
                    self.ibu_method,
                    hop.amount,
                    hop.alpha,
                    self.average_boil_volume(hop.time),
                    hop.time,
                    self.average_specific_gravity(hop.time),
                )
            })
    }

    /// Average boil volume
    ///
    /// Linearly interpolated average of boil volume on the interval $[T - t, T]$
    ///
    /// - $t$ [min]: `time`
    /// - $T$ [min]: End of boil
    fn average_boil_volume(&self, time: Minutes) -> Liters {
        let start_volume = utils::linear_interpolation(
            self.boil.boil_time - time,
            0.0,
            self.boil.boil_time,
            self.boil.pre_volume,
            self.batch_size,
        );
        (start_volume + self.batch_size) / 2.0
    }

    /// Average gravity
    ///
    /// Linearly interpolated average of specific gravity on the interval $[T - t, T]$
    ///
    /// - $t$ [min]: `time`
    /// - $T$ [min]: End of boil
    fn average_specific_gravity(&self, time: Minutes) -> Liters {
        let og = if let Some(og) = self.og {
            og
        } else {
            self.estimated_og()
        };

        let pre_boil_gravity = if let Some(pre_g) = self.pre_boil_gravity {
            pre_g
        } else {
            self.estimated_pre_boil_gravity()
        };

        let start_gravity = utils::linear_interpolation(
            self.boil.boil_time - time,
            0.0,
            self.boil.boil_time,
            pre_boil_gravity,
            og,
        );
        (start_gravity + og) / 2.0
    }

    pub fn estimated_pre_boil_gravity(&self) -> SpecificGravity {
        todo!("Calculate pre-boil gravity from recipe");
    }

    pub fn estimated_og(&self) -> SpecificGravity {
        todo!("Calculate OG from recipe");
    }
}
