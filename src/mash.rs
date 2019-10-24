use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
    equip_adjust: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct MashStep {}
