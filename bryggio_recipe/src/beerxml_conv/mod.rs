use crate::bryggio::{process, recipe::RecipeSrc, Recipe};
use beerxml;
use brew_calculator::ibu;
use std::convert::From;

impl From<beerxml::Recipe> for Recipe<BeerXmlSrc> {
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
            taste_rating: beerxml_recipe.taste_rating,
            date: beerxml_recipe.date,
            ibu_method: beerxml_recipe.ibu_method.unwrap_or_default(),
            recipe_src: BeerXmlSrc {
                ibu_method: beerxml_recipe.ibu_method,
            },
        }
    }
}

/// Original values in the BeerXML source recipe
///
/// Enables faithful translation back to BeerXML.
#[derive(Debug, Clone, Copy)]
pub struct BeerXmlSrc {
    ibu_method: Option<ibu::Method>,
}

impl RecipeSrc for BeerXmlSrc {}
