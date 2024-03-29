//! # Mash
//!
//!A mash profile is a record used either within a recipe or outside the recipe to precisely
//!specify the mash method used.  The record consists of some informational items followed by a
//!<MASH_STEPS> record that contains the actual mash steps.
use crate::utils;
use brew_calculator::units::*;
use serde::{Deserialize, Serialize};

///Mash profile for a recipe
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Mash {
    pub name: String,
    pub version: u8,
    ///The temperature of the grain before adding it to the mash.
    pub grain_temp: Celsius,
    #[serde(bound(deserialize = "Vec<MashStep>: Deserialize<'de>"))]
    pub mash_steps: MashSteps,
    pub notes: Option<String>,
    ///Grain tun temperature - may be used to adjust the infusion temperature for equipment.
    pub tun_temp: Option<Celsius>,
    ///Temperature of the sparge water
    pub sparge_temp: Option<Celsius>,
    pub ph: Option<PH>,
    pub tun_weight: Option<Kilograms>,
    ///Cal/(gram deg C)
    pub tun_specific_heat: Option<f32>,
    ///If `true`, mash infusion and decoction calculations should take into account the temperature effects of the equipment
    ///(tun specific heat and tun weight).
    ///If `false`, the tun is assumed to be pre-heated.
    ///Default is `false`.
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    pub equip_adjust: Option<bool>,
}

/// Wrapper type for MashStep vectors
///
/// Awkward extra type to conform to XML not having vectors but rather having a plural tag
/// enclosing multiple single tag
/// ```xml
///<MASH_STEPS>
///     <MASH_STEP>
///         ...
///     </MASH_STEP>
///     .
///     .
///     .
///     <MASH_STEP>
///         ...
///     </MASH_STEP>
///</MASH_STEPS>
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct MashSteps {
    #[serde(default = "Vec::new")]
    pub mash_step: Vec<MashStep>,
}

/// A mash step is an internal record used within a mash profile to denote a separate step in a multi-step mash.
/// A mash step is not intended for use outside of a mash profile.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "MASH_STEP")]
pub struct MashStep {
    pub name: String,
    pub version: u8,
    #[serde(rename = "TYPE")]
    pub type_: Type,
    pub infuse_amount: Option<Liters>,
    pub step_temp: Celsius,
    pub step_time: Minutes,
    pub ramp_time: Option<Minutes>,
    pub end_temp: Option<Celsius>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Infusion,
    Temperature,
    Decoction,
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml_mash {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn infusion_step() {
        let xml_input = r"
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
            ";
        let parsed_mash: Mash = serde_xml_rs::from_str(xml_input).unwrap();
        let true_mash_steps = MashSteps {
            mash_step: vec![MashStep {
                name: "Conversion Step, 68C".into(),
                version: 1,
                type_: Type::Infusion,
                step_temp: 68.0,
                step_time: 60.0,
                infuse_amount: Some(10.0),
                ramp_time: None,
                end_temp: None,
            }],
        };
        let true_mash = Mash {
            name: "Single Step Infusion, 68 C".into(),
            version: 1,
            grain_temp: 22.0,
            mash_steps: true_mash_steps,
            notes: None,
            tun_temp: None,
            sparge_temp: None,
            ph: None,
            tun_weight: None,
            tun_specific_heat: None,
            equip_adjust: None,
        };
        assert_eq!(parsed_mash, true_mash);
    }
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml_mash_step {
    use super::*;
    use serde_xml_rs;

    #[test]
    fn infusion_step() {
        let xml_input = r"
            <MASH_STEP>
                <NAME>Conversion step</NAME>
                <VERSION>1</VERSION>
                <TYPE>Infusion</TYPE>
                <STEP_TEMP>68.0</STEP_TEMP>
                <STEP_TIME>70.0</STEP_TIME>
                <INFUSE_AMOUNT>5.0</INFUSE_AMOUNT>
            </MASH_STEP>
            ";
        let parsed_mash_step: MashStep = serde_xml_rs::from_str(xml_input).unwrap();
        let true_mash_step = MashStep {
            name: "Conversion step".into(),
            version: 1,
            type_: Type::Infusion,
            step_temp: 68.0,
            step_time: 70.0,
            infuse_amount: Some(5.0),
            ramp_time: None,
            end_temp: None,
        };
        assert_eq!(parsed_mash_step, true_mash_step);
    }

    #[test]
    fn decoction_step() {
        let xml_input = r"
            <MASH_STEP>
                 <NAME>Conversion Decoction</NAME>
                 <VERSION>1</VERSION>
                 <TYPE>Decoction</TYPE>
                 <STEP_TEMP>68.0</STEP_TEMP>
                <STEP_TIME>90.0</STEP_TIME>
            </MASH_STEP>
            ";
        let parsed_mash_step: MashStep = serde_xml_rs::from_str(xml_input).unwrap();
        let true_mash_step = MashStep {
            name: "Conversion Decoction".into(),
            version: 1,
            type_: Type::Decoction,
            step_temp: 68.0,
            step_time: 90.0,
            infuse_amount: None,
            ramp_time: None,
            end_temp: None,
        };
        assert_eq!(parsed_mash_step, true_mash_step);
    }
}
