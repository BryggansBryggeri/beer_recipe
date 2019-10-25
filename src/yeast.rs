use crate::utils;
use crate::Percent;
use serde;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Yeast {
    name: String,
    version: u8,
    #[serde(rename = "TYPE")]
    type_: Type,
    form: Form,
    amount: f32,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    amount_is_weight: Option<bool>,
    laboratory: Option<String>,
    product_id: Option<String>,
    min_temperature: Option<f32>,
    max_temperature: Option<f32>,
    flocculation: Option<Flocculation>,
    attenuation: Option<Percent>,
    notes: Option<String>,
    best_for: Option<String>,
    times_cultured: Option<u8>,
    max_reuse: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    add_to_secondary: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Type {
    Ale,
    Lager,
    Wheat,
    Wine,
    Champagne,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Form {
    Liquid,
    Dry,
    Slant,
    Culture,
}

#[derive(Debug, PartialEq)]
enum Flocculation {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl<'de> Deserialize<'de> for Flocculation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Low" => Ok(Flocculation::Low),
            "Medium" => Ok(Flocculation::Medium),
            "High" => Ok(Flocculation::High),
            "Very High" => Ok(Flocculation::VeryHigh),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown type"])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn required_fields_only() {
        let xml_input = r"
            <YEAST>
                <NAME>Ole English Ale Yeast</NAME>
                <VERSION>1</VERSION>
                <TYPE>Ale</TYPE>
                <FORM>Liquid</FORM>
                <AMOUNT>0.100</AMOUNT>
            </YEAST>
        ";
        let parsed_yeast: Yeast = serde_xml_rs::from_str(xml_input).unwrap();
        let true_yeast = Yeast {
            name: "Ole English Ale Yeast".into(),
            version: 1,
            type_: Type::Ale,
            form: Form::Liquid,
            amount: 0.100,
            amount_is_weight: None,
            notes: None,
            laboratory: None,
            product_id: None,
            min_temperature: None,
            max_temperature: None,
            flocculation: None,
            attenuation: None,
            best_for: None,
            times_cultured: None,
            max_reuse: None,
            add_to_secondary: None,
        };
        assert_eq!(parsed_yeast, true_yeast);
    }

    #[test]
    fn yeast_with_more_popular_fields() {
        let xml_input = r"
        <YEAST>
            <NAME>German Ale</NAME>
            <TYPE>Ale</TYPE>
            <VERSION>1</VERSION>
            <FORM>Liquid</FORM>
            <AMOUNT>0.250</AMOUNT>
            <LABORATORY>Wyeast Labs</LABORATORY>
            <PRODUCT_ID>1007</PRODUCT_ID>
            <MIN_TEMPERATURE>12.8</MIN_TEMPERATURE>
            <MAX_TEMPERATURE>20.0</MAX_TEMPERATURE>
            <ATTENUATION>75.0</ATTENUATION>
            <NOTES>Crisp dry flavor with a hint of mild flavor.
            Great for many continental ales.
            </NOTES>
            <BEST_FOR>German Ales, Alts, Kolsch, Dry Stouts </BEST_FOR>
            <FLOCCULATION>Low</FLOCCULATION>
        </YEAST>
        ";
        let parsed_yeast: Yeast = serde_xml_rs::from_str(xml_input).unwrap();
        let true_yeast = Yeast {
            name: "German Ale".into(),
            version: 1,
            type_: Type::Ale,
            form: Form::Liquid,
            amount: 0.250,
            amount_is_weight: None,
            notes: Some(
                "Crisp dry flavor with a hint of mild flavor.
            Great for many continental ales."
                    .into(),
            ),
            laboratory: Some("Wyeast Labs".into()),
            product_id: Some("1007".into()),
            min_temperature: Some(12.8),
            max_temperature: Some(20.0),
            flocculation: Some(Flocculation::Low),
            attenuation: Some(75.0),
            best_for: Some("German Ales, Alts, Kolsch, Dry Stouts".into()),
            times_cultured: None,
            max_reuse: None,
            add_to_secondary: None,
        };
        assert_eq!(parsed_yeast, true_yeast);
    }
}
