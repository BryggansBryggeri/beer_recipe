use crate::utils;
use crate::{Temperature, Time, Volume};
use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Mash {
    name: String,
    version: u8,
    grain_temp: f32,
    mash_steps: Vec<MashStep>,
    notes: Option<String>,
    tun_temp: Option<f32>,
    sparge_weight: Option<f32>,
    ph: Option<f32>,
    tun_weight: Option<f32>,
    tun_specific_heat: Option<f32>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    equip_adjust: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "MASH_STEP")]
pub struct MashStep {
    name: String,
    version: u8,
    #[serde(rename = "TYPE")]
    type_: Type,
    infuse_amount: Option<Volume>,
    step_temp: Temperature,
    step_time: Time,
    ramp_time: Option<Time>,
    end_temp: Option<Temperature>,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Type {
    Infusion,
    Temperature,
    Decoction,
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml {
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
}
