//! # Equipment
//!
//! Though an equipment record is optional, when used it in a recipe or on its own it provides details needed to
//! calculate total water usage as well as water needed for each step.
//! It also contains information about the thermal parameters of the mash tun and large batch hop utilization factors.
use crate::utils;
use brew_calculator::units::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Equipment {
    name: String,
    version: u8,
    ///The pre-boil volume used in this particular instance for this equipment setup.
    ///Note that this may be a calculated value depending on the `calc_boil_volume` parameter.
    boil_size: f32,
    ///The target volume of the batch at the start of fermentation.
    batch_size: f32,
    ///Volume of the mash tun in liters.
    tun_volume: Option<Liters>,
    ///Weight of the mash tun in kilograms.
    ///Used primarily to calculate the thermal parameters of
    ///the mash tun – in conjunction with the volume and specific heat.
    tun_weight: Option<f32>,
    ///Cal/(gram deg C)
    tun_specific_heat: Option<f32>,
    ///The amount of top up water normally added just prior to starting fermentation.
    top_up_water: Option<Liters>,
    ///The amount of wort normally lost during transition from the boiler to the fermentation vessel.
    ///Includes both unusable wort due to trub and wort lost to the chiller and transfer systems.
    trub_chiller_loss: Option<Liters>,
    ///The percentage of wort lost to evaporation per hour
    evap_rate: Option<f32>,
    boil_time: Option<Minutes>,
    ///If `true`, then
    ///`boil_size = (batch_size - top_up_water - trub_chiller_loss) * (1 + boil_time * evap_rate
    ///)`.
    ///Then `boil size` should match this value.
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    calc_boil_volume: Option<bool>,
    ///Amount lost to the lauter tun and equipment associated with the lautering process.
    lauter_deadspace: Option<Liters>,
    ///Amount normally added to the boil kettle before the boil.
    top_up_kettle: Option<Liters>,
    hop_utilization: Option<Percent>,
    notes: Option<String>,
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml {
    use super::*;
    use serde_xml_rs;
    #[test]
    /// Modification to official example:
    /// Fixed incorrect closing tags:
    /// ```
    /// <TUN_VOLUME>18.93</MASH_TUN_VOLUME>
    /// <TUN_WEIGHT>2.0</MASH_TUN_WEIGHT>
    /// ```
    fn example() {
        let xml_input = r"
            <EQUIPMENT>
                <NAME>8 Gal pot with 5 gal Igloo Cooler</NAME>
                <VERSION>1</VERSION>
                <TUN_VOLUME>18.93</TUN_VOLUME>
                <TUN_WEIGHT>2.0</TUN_WEIGHT>
                <TUN_SPECIFIC_HEAT>0.3</TUN_SPECIFIC_HEAT>
                <BATCH_SIZE>18.93</BATCH_SIZE>
                <BOIL_SIZE>22.71</BOIL_SIZE>
                <TOP_UP_WATER>0.0</TOP_UP_WATER>
                <TRUB_CHILLER_LOSS>0.95</TRUB_CHILLER_LOSS>
                <EVAP_RATE>9.0</EVAP_RATE>
                <BOIL_TIME>60.0</BOIL_TIME>
                <CALC_BOIL_VOLUME>TRUE</CALC_BOIL_VOLUME>
                <LAUTER_DEADSPACE>0.95</LAUTER_DEADSPACE>
                <TOP_UP_KETTLE>0.0</TOP_UP_KETTLE>
                <HOP_UTILIZATION>100.0</HOP_UTILIZATION>
                <NOTES>Popular all grain setup.  5 Gallon Gott or Igloo cooler as mash tun with false bottom, and 7-9 gallon brewpot capable of boiling at least 6 gallons of wort.  Primarily used for single infusion mashes.</NOTES>
            </EQUIPMENT>
            ";
        let parsed_equip: Equipment = serde_xml_rs::from_str(xml_input).unwrap();
        let true_equip = Equipment {
            name: "8 Gal pot with 5 gal Igloo Cooler".into(),
            version: 1,
            boil_size: 22.71,
            batch_size: 18.93,
            tun_volume: Some(18.93),
            tun_weight: Some(2.0),
            tun_specific_heat: Some(0.3),
            top_up_water: Some(0.0),
            trub_chiller_loss: Some(0.95),
            evap_rate: Some(9.0),
            boil_time: Some(60.0),
            calc_boil_volume: Some(true),
            lauter_deadspace: Some(0.95),
            top_up_kettle: Some(0.0),
            hop_utilization: Some(100.0),
            notes: Some(
                "Popular all grain setup.  5 Gallon Gott or Igloo cooler as mash tun with false bottom, and 7-9 gallon brewpot capable of boiling at least 6 gallons of wort.  Primarily used for single infusion mashes.".into()),
        };
        assert_eq!(parsed_equip, true_equip);
    }
}
