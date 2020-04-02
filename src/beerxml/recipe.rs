use crate::beerxml::equipment::Equipment;
use crate::beerxml::fermentable::Fermentables;
use crate::beerxml::hop::Hops;
use crate::beerxml::mash::Mash;
use crate::beerxml::misc::Miscs;
use crate::beerxml::style::Style;
use crate::beerxml::water::Waters;
use crate::beerxml::yeast::Yeasts;
use crate::units::*;
use crate::utils;
use serde;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Recipe {
    pub name: String,
    pub version: u8,
    #[serde(rename = "TYPE")]
    pub(crate) type_: Type,
    pub(crate) style: Style,
    pub(crate) equipment: Option<Equipment>,
    pub(crate) brewer: String,
    pub(crate) asst_brewer: Option<String>,
    pub batch_size: f32,
    pub(crate) boil_size: Liters,
    pub(crate) boil_time: Minutes,
    /// Not used for `Type::Extract`
    pub(crate) efficiency: f32,
    pub(crate) hops: Hops,
    pub(crate) fermentables: Fermentables,
    pub(crate) miscs: Miscs,
    pub(crate) yeasts: Yeasts,
    pub(crate) waters: Waters,
    pub(crate) mash: Mash,
    pub(crate) notes: Option<String>,
    pub(crate) taste_notes: Option<String>,
    pub(crate) taste_rating: Option<f32>,
    pub(crate) og: Option<SpecificGravity>,
    pub(crate) fg: Option<SpecificGravity>,
    pub(crate) fermentation_stages: Option<u8>,
    pub(crate) primary_age: Option<Days>,
    pub(crate) primary_temp: Option<Celsius>,
    pub(crate) secondary_age: Option<Days>,
    pub(crate) secondary_temp: Option<Celsius>,
    pub(crate) tertiary_age: Option<Days>,
    pub(crate) tertiary_temp: Option<Celsius>,
    pub(crate) age: Option<Days>,
    pub(crate) age_temp: Option<Celsius>,
    pub(crate) date: Option<String>,
    pub(crate) carbonation: Option<VolumesCO2>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    pub(crate) forced_carbonation: Option<bool>,
    pub(crate) priming_sugar_name: Option<String>,
    pub(crate) carbonation_temp: Option<Celsius>,
    pub(crate) priming_sugar_equiv: Option<f32>,
    pub(crate) keg_priming_factor: Option<f32>,
    #[serde(default, with = "ibu_method")]
    pub(crate) ibu_method: Option<IbuMethod>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Type {
    Extract,
    PartialMash,
    AllGrain,
}

// TODO: This can be done with serde macro?
impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Extract" => Ok(Type::Extract),
            "Partial Mash" => Ok(Type::PartialMash),
            "All Grain" => Ok(Type::AllGrain),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown type"])),
        }
    }
}

/// IBU methods
#[derive(Debug, PartialEq)]
pub enum IbuMethod {
    Tinseth,
    Rager,
    Garetz,
}

// TODO: This can be done with serde macro?
mod ibu_method {
    use super::IbuMethod;
    use serde::{Deserialize, Deserializer};
    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Option<IbuMethod>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tinseth" => Ok(Some(IbuMethod::Tinseth)),
            "Rager" => Ok(Some(IbuMethod::Rager)),
            "Garetz" => Ok(Some(IbuMethod::Garetz)),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown type"])),
        }
    }
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
/// RATING -> TEST_RATING
mod beerxml {
    use super::*;
    use crate::beerxml::mash::Type as MashStepType;
    use crate::beerxml::mash::{MashStep, MashSteps};
    use crate::beerxml::style::Type as StyleType;
    use serde_xml_rs;
    #[test]
    fn minimal_recipe() {
        let xml_input = r#"
            <?xml version="1.0" encoding="ISO-8859-1"?>
              <RECIPE>
                <NAME>Dry Stout</NAME>
                <VERSION>1</VERSION>
                <TYPE>All Grain</TYPE>
                <BREWER>Brad Smith</BREWER>
                <BATCH_SIZE>18.93</BATCH_SIZE>
                <BOIL_SIZE>20.82</BOIL_SIZE>
                <BOIL_TIME>60.0</BOIL_TIME>
                <EFFICIENCY>72.0</EFFICIENCY>
                <STYLE>
                  <NAME>Dry Stout</NAME>
                  <CATEGORY>Stout</CATEGORY>
                  <CATEGORY_NUMBER>16</CATEGORY_NUMBER>
                  <STYLE_LETTER>A</STYLE_LETTER>
                  <STYLE_GUIDE>BJCP</STYLE_GUIDE>
                  <VERSION>1</VERSION>
                  <TYPE>Ale</TYPE>
                  <OG_MIN>1.035</OG_MIN>
                  <OG_MAX>1.050</OG_MAX>
                  <FG_MIN>1.007</FG_MIN>
                  <FG_MAX>1.011</FG_MAX>
                  <IBU_MIN>30.0</IBU_MIN>
                  <IBU_MAX>50.0</IBU_MAX>
                  <COLOR_MIN>35.0</COLOR_MIN>
                  <COLOR_MAX>200.0</COLOR_MAX>
                </STYLE>
                <HOPS>
                </HOPS>
                <FERMENTABLES>
                </FERMENTABLES>
                <MISCS>
                </MISCS>
                <WATERS>
                </WATERS>
                <YEASTS>
                </YEASTS>
                <MASH>
                  <NAME>Single Step Infusion, 68 C</NAME>
                  <VERSION>1</VERSION>
                  <GRAIN_TEMP>22.0</GRAIN_TEMP>
                  <MASH_STEPS>
                    <MASH_STEP>
                      <NAME>Conversion Step, 68C </NAME>
                      <VERSION>1</VERSION>
                      <TYPE>Infusion</TYPE>
                      <STEP_TEMP>68.0</STEP_TEMP>
                      <STEP_TIME>60.0</STEP_TIME>
                      <INFUSE_AMOUNT>10.0</INFUSE_AMOUNT>
                    </MASH_STEP>
                  </MASH_STEPS>
                </MASH>
            </RECIPE>"#;
        let parsed_recipe: Recipe = serde_xml_rs::from_str(xml_input).unwrap();
        let true_recipe = Recipe {
            name: "Dry Stout".into(),
            version: 1,
            type_: Type::AllGrain,
            brewer: "Brad Smith".into(),
            style: Style {
                name: "Dry Stout".into(),
                category: "Stout".into(),
                category_number: "16".into(),
                style_letter: "A".into(),
                style_guide: "BJCP".into(),
                version: 1,
                type_: StyleType::Ale,
                og_max: 1.05,
                og_min: 1.035,
                abv_max: None,
                abv_min: None,
                carb_max: None,
                carb_min: None,
                color_max: 200.0,
                color_min: 35.0,
                fg_max: 1.011,
                fg_min: 1.007,
                ibu_max: 50.0,
                ibu_min: 30.0,
                ingredients: None,
                notes: None,
                examples: None,
                profile: None,
            },
            equipment: None,
            age: None,
            age_temp: None,
            date: None,
            carbonation: None,
            carbonation_temp: None,
            fermentation_stages: None,
            fg: None,
            og: None,
            keg_priming_factor: None,
            notes: None,
            primary_age: None,
            primary_temp: None,
            priming_sugar_equiv: None,
            priming_sugar_name: None,
            secondary_age: None,
            secondary_temp: None,
            tertiary_age: None,
            tertiary_temp: None,
            taste_notes: None,
            taste_rating: None,
            forced_carbonation: None,
            asst_brewer: None,
            batch_size: 18.93,
            boil_size: 20.82,
            boil_time: 60.0,
            efficiency: 72.0,
            hops: Hops::default(),
            fermentables: Fermentables::default(),
            miscs: Miscs::default(),
            yeasts: Yeasts::default(),
            waters: Waters::default(),
            mash: Mash {
                name: "Single Step Infusion, 68 C".into(),
                version: 1,
                grain_temp: 22.0,
                mash_steps: MashSteps {
                    mash_step: vec![MashStep {
                        name: "Conversion Step, 68C".into(),
                        version: 1,
                        type_: MashStepType::Infusion,
                        step_temp: 68.0,
                        step_time: 60.0,
                        infuse_amount: Some(10.0),
                        ramp_time: None,
                        end_temp: None,
                    }],
                },
                notes: None,
                tun_temp: None,
                sparge_temp: None,
                ph: None,
                tun_weight: None,
                tun_specific_heat: None,
                equip_adjust: None,
            },
            ibu_method: None,
        };
        assert_eq!(parsed_recipe, true_recipe);
    }
}
