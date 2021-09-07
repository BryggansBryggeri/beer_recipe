use thiserror::Error;

use crate::bryggio::{process, Equipment, Fermentable, Hop, Misc, Style, Type, Water, Yeast};
use brew_calculator::units::*;
use brew_calculator::{ibu, ibu::IbuCalc, utils};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Recipe<Src: RecipeSrc> {
    pub name: String,
    #[serde(rename = "TYPE")]
    pub(crate) type_: Type,
    pub(crate) style: Style,
    pub(crate) brewer: String,
    pub(crate) asst_brewer: Option<String>,
    pub(crate) equipment: Option<Equipment>,
    pub batch_size: Liters,
    pub(crate) pre_boil_gravity: Option<SpecificGravity>,
    pub(crate) og: Option<SpecificGravity>,
    pub(crate) fg: Option<SpecificGravity>,
    /// Not used for `Type::Extract`
    pub(crate) efficiency: f32,
    pub(crate) hops: Vec<Hop>,
    pub(crate) fermentables: Vec<Fermentable>,
    pub(crate) yeasts: Vec<Yeast>,
    pub(crate) waters: Vec<Water>,
    pub(crate) miscs: Vec<Misc>,
    pub(crate) mash: process::Mash,
    pub(crate) boil: process::Boil,
    pub(crate) fermentation: process::Fermentation,
    pub(crate) carbonation: process::Carbonation,
    pub(crate) notes: Option<String>,
    pub(crate) taste_notes: Option<String>,
    pub(crate) taste_rating: Option<f32>,
    pub(crate) date: Option<String>,
    pub(crate) ibu_method: ibu::Method,
    pub(crate) recipe_src: Src,
}

impl<Src: RecipeSrc> Recipe<Src> {
    pub fn hops(&self) -> std::slice::Iter<Hop> {
        self.hops.iter()
    }

    pub fn fermentables(&self) -> std::slice::Iter<Fermentable> {
        self.fermentables.iter()
    }

    pub fn yeasts(&self) -> std::slice::Iter<Yeast> {
        self.yeasts.iter()
    }

    pub fn waters(&self) -> std::slice::Iter<Water> {
        self.waters.iter()
    }

    pub fn miscs(&self) -> std::slice::Iter<Misc> {
        self.miscs.iter()
    }

    /// Total IBU for recipe
    ///
    /// Calculates and sums the individual IBU contributions for all bittering hops.
    pub fn ibu(&self) -> Ibu {
        self.hops()
            .filter(|hop| hop.bittering())
            .fold(0.0, |acc, hop| {
                acc + self.ibu_method.ibu(
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
    /// Linearly interpolated average of boil volume on the interval $\[T - t, T\]$
    ///
    /// - $t$ \[min\]: `time`
    /// - $T$ \[min\]: End of boil
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
    /// Linearly interpolated average of specific gravity on the interval $\[T - t, T\]$
    ///
    /// - $t$ \[min\]: `time`
    /// - $T$ \[min\]: End of boil
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

pub trait RecipeSrc {}

#[derive(Copy, Clone, Debug, Error)]
pub enum RecipeError {
    #[error("Tried calculating IBU with IBU method 'None'")]
    IbuCalcWithoutIbuMethod,
}
