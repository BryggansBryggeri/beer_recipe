//! # Style
//!
//! The term "style" encompasses beer styles.
//! The beer style may be from the BJCP style guide, Australian, UK or local style guides.
//! Generally a recipe is designed to one style.
use serde;
use serde::{Deserialize, Serialize};

use crate::units::*;
use crate::RecordSet;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Style {
    pub name: String,
    pub category: String,
    pub version: u8,
    /// Supposed to be text but seems to be a semantic u8
    pub category_number: u8,
    pub style_letter: String,
    pub style_guide: String,
    #[serde(rename = "TYPE")]
    pub type_: Type,
    ///The minimum original specific gravity as measured relative to water.
    pub og_min: SpecificGravity,
    ///The maximum original specific gravity as measured relative to water.
    pub og_max: SpecificGravity,
    ///The minimum final specific gravity as measured relative to water.
    pub fg_min: SpecificGravity,
    ///The maximum final specific gravity as measured relative to water.
    pub fg_max: SpecificGravity,
    ///The recommended minimum bitterness for this style as measured in
    ///International Bitterness Units (IBUs)
    pub ibu_min: IBU,
    ///The recommended maximum bitterness for this style as measured in
    ///International Bitterness Units (IBUs)
    pub ibu_max: IBU,
    pub color_min: SRMColor,
    pub color_max: SRMColor,
    ///Minimum recommended carbonation for this style in volumes of CO2
    pub carb_min: Option<VolumesCO2>,
    ///Maximum recommended carbonation for this style in volumes of CO2
    pub carb_max: Option<VolumesCO2>,
    pub abv_min: Option<ABV>,
    pub abv_max: Option<ABV>,
    pub notes: Option<String>,
    pub profile: Option<String>,
    pub ingredients: Option<String>,
    pub examples: Option<String>,
}

impl RecordSet for Style {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Type {
    Lager,
    Ale,
    Mead,
    Wheat,
    Mixed,
    Cider,
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn bohemian_pilsner() {
        let xml_input = r"
            <STYLE>
                <NAME>Bohemian Pilsner</NAME>
                <CATEGORY>European Pale Ale</CATEGORY>
                <CATEGORY_NUMBER>2</CATEGORY_NUMBER>
                <STYLE_LETTER>A</STYLE_LETTER>
                <STYLE_GUIDE>BJCP</STYLE_GUIDE>
                <VERSION>1</VERSION>
                <TYPE>Lager</TYPE>
                <OG_MIN>1.044</OG_MIN>
                <OG_MAX>1.056</OG_MAX>
                <FG_MIN>1.013</FG_MIN>
                <FG_MAX>1.017</FG_MAX>
                <IBU_MIN>35.0</IBU_MIN>
                <IBU_MAX>45.0</IBU_MAX>
                <COLOR_MIN>3.0</COLOR_MIN>
                <COLOR_MAX>5.0</COLOR_MAX>
                <NOTES>Famous beer of Pilsen, Czech republic.  Light to medium body with some sweetness.  Saaz hop flavor and aroma with no lingering bitterness.
                </NOTES>
            </STYLE>
            ";
        let parsed_style: Style = serde_xml_rs::from_str(xml_input).unwrap();
        let true_style = Style {
            name: "Bohemian Pilsner".into(),
            category: "European Pale Ale".into(),
            version: 1,
            notes: Some("Famous beer of Pilsen, Czech republic.  Light to medium body with some sweetness.  Saaz hop flavor and aroma with no lingering bitterness.".into()),
            type_: Type::Lager,
            category_number: 2,
            style_letter: "A".into(),
            style_guide: "BJCP".into(),
            og_min: 1.044,
            og_max: 1.056,
            fg_min: 1.013,
            fg_max: 1.017,
            ibu_min: 35.0,
            ibu_max: 45.0,
            color_min: 3.0,
            color_max: 5.0,
            carb_min: None,
            carb_max: None,
            abv_min: None,
            abv_max: None,
            profile: None,
            ingredients: None,
            examples: None,
        };
        assert_eq!(parsed_style, true_style);
    }

    #[test]
    fn dry_irish_stout_all_fields() {
        let xml_input = r"
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
                <ABV_MIN>3.2</ABV_MIN>
                <ABV_MAX>5.5</ABV_MAX>
                <CARB_MIN>1.6</CARB_MIN>
                <CARB_MAX>2.1</CARB_MAX>
                <NOTES>Famous Irish Stout.  Dry, roasted, almost coffee like flavor.  Often soured with pasteurized sour beer. 
                </NOTES>
                <PROFILE>Full body perception due to flaked barley, though starting gravity may be low.  Dry roasted flavor.
                </PROFILE>
                <INGREDIENTS>Made with black barley and flaked barley,  Hard water.  Irish Ale Yeast.
                </INGREDIENTS>
                <EXAMPLES>Guinness</EXAMPLES>
            </STYLE>
            ";
        let parsed_style: Style = serde_xml_rs::from_str(xml_input).unwrap();
        let true_style = Style {
            name: "Dry Stout".into(),
            category: "Stout".into(),
            version: 1,
            notes: Some("Famous Irish Stout.  Dry, roasted, almost coffee like flavor.  Often soured with pasteurized sour beer.".into()),
            type_: Type::Ale,
            category_number: 16,
            style_letter: "A".into(),
            style_guide: "BJCP".into(),
            og_min: 1.035,
            og_max: 1.050,
            fg_min: 1.007,
            fg_max: 1.011,
            ibu_min: 30.0,
            ibu_max: 50.0,
            color_min: 35.0,
            color_max: 200.0,
            carb_min: Some(1.6),
            carb_max: Some(2.1),
            abv_min: Some(3.2),
            abv_max: Some(5.5),
            profile: Some("Full body perception due to flaked barley, though starting gravity may be low.  Dry roasted flavor.".into()),
            ingredients: Some("Made with black barley and flaked barley,  Hard water.  Irish Ale Yeast.".into()),
            examples: Some("Guinness".into()),
        };
        assert_eq!(parsed_style, true_style);
    }
}
