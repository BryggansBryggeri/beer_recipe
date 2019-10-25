use crate::utils;
use crate::Percent;
use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Equipment {
    name: String,
    version: u8,
    boil_size: f32,
    batch_size: f32,
    tun_volume: Option<f32>,
    tun_weight: Option<f32>,
    tun_specific_heat: Option<f32>,
    top_up_water: Option<f32>,
    trub_chiller_loss: Option<f32>,
    evap_rate: Option<f32>,
    boil_time: Option<f32>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    calc_boil_volume: Option<bool>,
    lauter_deadspace: Option<f32>,
    top_up_kettle: Option<f32>,
    hop_utilization: Option<Percent>,
    notes: Option<String>,
}

#[cfg(test)]
mod tests {
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
