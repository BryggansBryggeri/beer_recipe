//! # Water
//!
//! The term "water" encompasses water profiles.
//! Though not strictly required for recipes, the water record allows supporting programs to record the water profile used for brewing a particular batch.
use crate::units::*;
use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Water {
    name: String,
    version: u8,
    amount: PartsPerMillion,
    calcium: PartsPerMillion,
    bicarbonate: PartsPerMillion,
    sulfate: PartsPerMillion,
    chloride: PartsPerMillion,
    sodium: PartsPerMillion,
    magnesium: PartsPerMillion,
    ph: Option<PH>,
    notes: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Waters {
    #[serde(default = "Vec::new")]
    pub water: Vec<Water>,
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn sample_water_profile() {
        let xml_input = r"
            <WATER>
                <NAME>Burton on Trent, UK</NAME>
                <VERSION>1</VERSION>
                <AMOUNT>20.0</AMOUNT>
                <CALCIUM>295.0</CALCIUM>
                <MAGNESIUM>45.0</MAGNESIUM>
                <SODIUM>55.0</SODIUM>
                <SULFATE>725.0</SULFATE>
                <CHLORIDE>25.0</CHLORIDE>
                <BICARBONATE>300.0</BICARBONATE>
                <PH>8.0</PH>
                <NOTES>
                Use for distinctive pale ales strongly hopped.  Very hard water accentuates the hops flavor. Example: Bass Ale
                </NOTES>
            </WATER>
        ";
        let parsed_water: Water = serde_xml_rs::from_str(xml_input).unwrap();
        let true_water = Water {
            name: "Burton on Trent, UK".into(),
            version: 1,
            amount: 20.0,
            calcium: 295.0,
            bicarbonate: 300.0,
            sulfate: 725.0,
            chloride: 25.0,
            sodium: 55.0,
            magnesium: 45.0,
            ph: Some(8.0),
            notes: Some("Use for distinctive pale ales strongly hopped.  Very hard water accentuates the hops flavor. Example: Bass Ale".into()),
        };
        assert_eq!(parsed_water, true_water);
    }
}
