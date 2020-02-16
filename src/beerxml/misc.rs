//! # Misc
//!
//! The term "misc" encompasses all non-fermentable miscellaneous ingredients that are not hops or yeast and do not significantly change the gravity of the beer.
//! For example: spices, clarifying agents, water treatments, etc…
use crate::units::*;
use crate::utils;
use serde;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Misc {
    name: String,
    version: u8,
    #[serde(rename = "TYPE")]
    type_: Type,
    #[serde(rename = "USE")]
    use_: Use,
    ///Amount of time in minutes.
    time: Minutes,
    ///Amount of item used. The default measurements are by weight, but this may be the
    ///measurement in volume units if `amount_is_weight` is set to `true`
    amount: f32,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    amount_is_weight: Option<bool>,
    use_for: Option<String>,
    notes: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Miscs {
    #[serde(default = "Vec::new")]
    pub misc: Vec<Misc>,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Use {
    Boil,
    Mash,
    Primary,
    Secondary,
    Bottling,
}

#[derive(Debug, PartialEq)]
enum Type {
    Spice,
    Fining,
    WaterAgent,
    Herb,
    Flavor,
    Other,
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Spice" => Ok(Type::Spice),
            "Fining" => Ok(Type::Fining),
            "Water Agent" => Ok(Type::WaterAgent),
            "Herb" => Ok(Type::Herb),
            "Flavor" => Ok(Type::Flavor),
            "Other" => Ok(Type::Other),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown type"])),
        }
    }
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn irish_moss_minimal_fields() {
        let xml_input = r"
            <MISC>
                <NAME>Irish Moss</NAME>
                <VERSION>1</VERSION>
                <TYPE>Fining</TYPE>
                <USE>Boil</USE>
                <TIME>15.0</TIME>
                <AMOUNT>0.010</AMOUNT>
            </MISC>
        ";
        let parsed_misc: Misc = serde_xml_rs::from_str(xml_input).unwrap();
        let true_misc = Misc {
            name: "Irish Moss".into(),
            version: 1,
            type_: Type::Fining,
            use_: Use::Boil,
            time: 15.0,
            amount: 0.010,
            amount_is_weight: None,
            use_for: None,
            notes: None,
        };
        assert_eq!(parsed_misc, true_misc);
    }

    #[test]
    ///If you add coriander, I will call the police
    fn coriander_spice_with_typical_fields() {
        let xml_input = r"
            <MISC>
                <NAME>Coriander</NAME>
                <TYPE>Spice</TYPE>
                <VERSION>1</VERSION>
                <USE>Boil</USE>
                <TIME>5.0</TIME>
                <AMOUNT>0.025</AMOUNT>
                <USE_FOR>Belgian Wit Spice</USE_FOR>
                <NOTES>Used in Belgian Wit, Whites, and Holiday ales.  Very good when used in light wheat ales.  Often used with Bitter Orange Peel. Crack open seeds and add at the end of the boil to extract aroma and flavor.
                </NOTES>
            </MISC>
        ";
        let parsed_misc: Misc = serde_xml_rs::from_str(xml_input).unwrap();
        let true_misc = Misc {
            name: "Coriander".into(),
            version: 1,
            type_: Type::Spice,
            use_: Use::Boil,
            time: 5.0,
            amount: 0.025,
            amount_is_weight: None,
            use_for: Some("Belgian Wit Spice".into()),
            notes: Some(
                "Used in Belgian Wit, Whites, and Holiday ales.  Very good when used in light wheat ales.  Often used with Bitter Orange Peel. Crack open seeds and add at the end of the boil to extract aroma and flavor."
                .into()),
        };
        assert_eq!(parsed_misc, true_misc);
    }
}
