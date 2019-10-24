use serde;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Hop {
    name: String,
    version: u8,
    alpha: f32,
    amount: f32,
    /// Original beerxml key is `use`
    #[serde(rename = "USE")]
    use_: Use,
    /// Minutes
    time: f32,
    notes: Option<String>,
    #[serde(rename = "TYPE")]
    type_: Option<Type>,
    form: Option<Form>,
    beta: Option<f32>,
    hsi: Option<f32>,
    origin: Option<String>,
    substitutes: Option<String>,
    humulene: Option<f32>,
    caryophyllene: Option<f32>,
    cohumulone: Option<f32>,
    myrcene: Option<f32>,
}

#[derive(Debug, PartialEq)]
enum Use {
    Boil,
    DryHop,
    Mash,
    FirstWort,
    Aroma,
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

#[derive(Deserialize, Debug)]
enum Type {
    Bittering,
    Aroma,
    Both,
}

#[derive(Deserialize, Debug)]
enum Form {
    Pellet,
    Plug,
    Leaf,
}

#[cfg(test)]
mod tests {
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
        <USE>Dry Hop</USE>
        <TIME>60</TIME>
        </HOP>";
        let hop: Hop = serde_xml_rs::from_str(xml_input).unwrap();
        assert_eq!(hop.use_, Use::DryHop);
    }
}
