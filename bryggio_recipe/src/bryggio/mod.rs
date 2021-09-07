use crate::beerxml;

mod process;
pub mod recipe;
mod state_machine;
pub use recipe::Recipe;

/// Redefine BeerXML types as Bryggio types.
/// This is for future convenience
/// If the original BeerXML types are ever replaced with custom Bryggio types,
/// then only the definitions below need to change and the entire code base
/// will be in sync.
type Hop = beerxml::Hop;
type Fermentable = beerxml::Fermentable;
type Yeast = beerxml::Yeast;
type Water = beerxml::Water;
type Equipment = beerxml::Equipment;
type Style = beerxml::Style;
type Misc = beerxml::Misc;
type Type = beerxml::Type;
type IbuMethod = beerxml::IbuMethod;
