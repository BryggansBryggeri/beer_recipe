//! # Hop
use crate::units::*;
use serde;
use serde::Deserialize;
use serde::Deserializer;

use crate::RecordSet;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Hop {
    pub name: String,
    version: u8,
    pub alpha: Percent,
    pub amount: Kilograms,
    #[serde(rename = "USE")]
    pub use_: Use,
    /// The time as measured in minutes.
    pub time: Minutes,
    notes: Option<String>,
    #[serde(rename = "TYPE")]
    type_: Option<Type>,
    form: Option<Form>,
    beta: Option<Percent>,
    ///Hop Stability Index
    ///
    ///Defined as the percentage of hop alpha lost in 6 months of storage
    hsi: Option<Percent>,
    origin: Option<String>,
    substitutes: Option<String>,
    humulene: Option<Percent>,
    caryophyllene: Option<Percent>,
    cohumulone: Option<Percent>,
    myrcene: Option<Percent>,
}

impl Hop {
    pub fn bittering(&self) -> bool {
        self.use_ != Use::Aroma && self.use_ != Use::DryHop
    }
}

impl RecordSet for Hop {}

#[derive(Debug, PartialEq, Clone)]
pub enum Use {
    Boil,
    DryHop,
    Mash,
    FirstWort,
    Aroma,
}

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Hops {
    #[serde(default = "Vec::new")]
    pub hop: Vec<Hop>,
}

impl<'de> Deserialize<'de> for Use {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Boil" => Ok(Use::Boil),
            "Dry Hop" => Ok(Use::DryHop),
            "Mash" => Ok(Use::Mash),
            "First Wort" => Ok(Use::FirstWort),
            "Aroma" => Ok(Use::Aroma),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown type"])),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
enum Type {
    Bittering,
    Aroma,
    Both,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
enum Form {
    Pellet,
    Plug,
    Leaf,
}

#[cfg(test)]
/// Official tests from 'http://www.beerxml.com/beerxml.htm'
mod beerxml {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn required_fields_only() {
        let xml_input = r"
            <HOP>
                <NAME>Cascade</NAME>
                <VERSION>1</VERSION>
                <ALPHA>5.0</ALPHA>
                <AMOUNT>0.100</AMOUNT>
                <USE>Boil</USE>
                <TIME>60</TIME>
            </HOP>";
        let parsed_hop: Hop = serde_xml_rs::from_str(xml_input).unwrap();
        let true_hop = Hop {
            name: "Cascade".into(),
            version: 1,
            alpha: 5.0,
            amount: 0.100,
            use_: Use::Boil,
            time: 60.0,
            notes: None,
            type_: None,
            form: None,
            beta: None,
            hsi: None,
            origin: None,
            substitutes: None,
            humulene: None,
            caryophyllene: None,
            cohumulone: None,
            myrcene: None,
        };
        assert_eq!(parsed_hop, true_hop);
    }

    #[test]
    fn dry_hop_for_three_days() {
        let xml_input = r"
            <HOP>
                <NAME>Fuggles</NAME>
                <VERSION>1</VERSION>
                <ALPHA>4.5</ALPHA>
                <AMOUNT>0.250</AMOUNT>
                <USE>Dry Hop</USE>
                <TIME>10080.0</TIME>
            </HOP>";
        let parsed_hop: Hop = serde_xml_rs::from_str(xml_input).unwrap();
        let true_hop = Hop {
            name: "Fuggles".into(),
            version: 1,
            alpha: 4.5,
            amount: 0.250,
            use_: Use::DryHop,
            time: 10080.0,
            notes: None,
            type_: None,
            form: None,
            beta: None,
            hsi: None,
            origin: None,
            substitutes: None,
            humulene: None,
            caryophyllene: None,
            cohumulone: None,
            myrcene: None,
        };
        assert_eq!(parsed_hop, true_hop);
    }

    #[test]
    fn mash_hops_with_all_fields_shuffled() {
        let xml_input = r#"
            <?xml version="1.0" encoding="ISO-8859-1"?>
            <HOP>
                <AMOUNT>0.050</AMOUNT>
                <VERSION>1</VERSION>
                <USE>Mash</USE>
                <ALPHA>4.5</ALPHA>
                <NOTES> This hop is a really cool hops - you can use it for anything.
                It leaps over buildings in a single bound, is faster than
                a speeding bullet and makes really bitter beer.
                </NOTES>
                <TIME>45.0</TIME>
                <BETA>5.5 </BETA>
                <NAME>Super Hops</NAME>
                <ORIGIN>Planet Krypton</ORIGIN>
                <SUBSTITUTES>Goldings, Fuggles, Super Alpha</SUBSTITUTES>
                <MYRCENE>24.4</MYRCENE>
                <HSI>30</HSI>
                <FORM>Leaf</FORM>
                <TYPE>Bittering</TYPE>
                <COHUMULONE>13.2</COHUMULONE>
            </HOP>"#;
        let parsed_hop: Hop = serde_xml_rs::from_str(xml_input).unwrap();
        let true_hop = Hop {
            name: "Super Hops".into(),
            version: 1,
            alpha: 4.5,
            amount: 0.050,
            use_: Use::Mash,
            time: 45.0,
            notes: Some(
                "This hop is a really cool hops - you can use it for anything.
                It leaps over buildings in a single bound, is faster than
                a speeding bullet and makes really bitter beer."
                    .into(),
            ),
            type_: Some(Type::Bittering),
            form: Some(Form::Leaf),
            beta: Some(5.5),
            hsi: Some(30.0),
            origin: Some("Planet Krypton".into()),
            substitutes: Some("Goldings, Fuggles, Super Alpha".into()),
            humulene: None,
            caryophyllene: None,
            cohumulone: Some(13.2),
            myrcene: Some(24.4),
        };
        assert_eq!(parsed_hop, true_hop);
    }
}
