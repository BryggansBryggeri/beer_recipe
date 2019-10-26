use crate::utils;
use crate::Percent;
use serde;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Fermentable {
    name: String,
    version: u8,
    #[serde(rename = "TYPE")]
    type_: Type,
    amount: f32,
    #[serde(rename = "YIELD")]
    yield_: Percent,
    color: f32,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    add_after_boil: Option<bool>,
    origin: Option<String>,
    supplier: Option<String>,
    notes: Option<String>,
    coarse_fine_diff: Option<Percent>,
    moisture: Option<Percent>,
    diastatic_power: Option<f32>,
    protein: Option<Percent>,
    max_in_batch: Option<Percent>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    recommend_mash: Option<bool>,
    ibu_gal_per_lb: Option<f32>,
}

#[derive(Debug, PartialEq)]
enum Type {
    Grain,
    Sugar,
    Extract,
    DryExtract,
    Adjunct,
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Grain" => Ok(Type::Grain),
            "Sugar" => Ok(Type::Sugar),
            "Extract" => Ok(Type::Extract),
            "Dry Extract" => Ok(Type::DryExtract),
            "Adjunct" => Ok(Type::Adjunct),
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
    fn required_fields_only() {
        let xml_input = r"
            <FERMENTABLE>
                <NAME>Pale 2-row Malt</NAME>]
                <VERSION>1</VERSION>
                <AMOUNT>5.0</AMOUNT>
                <TYPE>Grain</TYPE>
                <YIELD>73.4</YIELD>
                <COLOR>3.0</COLOR>
            </FERMENTABLE>
        ";
        let parsed_fermentable: Fermentable = serde_xml_rs::from_str(xml_input).unwrap();
        let true_fermentable = Fermentable {
            name: "Pale 2-row Malt".into(),
            version: 1,
            type_: Type::Grain,
            amount: 5.0,
            yield_: 73.4,
            color: 3.0,
            add_after_boil: None,
            origin: None,
            supplier: None,
            notes: None,
            coarse_fine_diff: None,
            moisture: None,
            diastatic_power: None,
            protein: None,
            max_in_batch: None,
            recommend_mash: None,
            ibu_gal_per_lb: None,
        };
        assert_eq!(parsed_fermentable, true_fermentable);
    }

    #[test]
    fn hopped_extract() {
        let xml_input = r"
        <FERMENTABLE>
            <NAME>Fustons Hopped Amber</NAME>
            <VERSION>1</VERSION>
            <AMOUNT>0.50</AMOUNT>
            <NOTES>Hopped amber extract suitable as a base for english ales.
            </NOTES>
            <YIELD>78.0</YIELD>
            <TYPE>Extract</TYPE>
            <COLOR>13</COLOR>
            <IBU_GAL_PER_LB>16.6</IBU_GAL_PER_LB>
        </FERMENTABLE>
        ";
        let parsed_fermentable: Fermentable = serde_xml_rs::from_str(xml_input).unwrap();
        let true_fermentable = Fermentable {
            name: "Fustons Hopped Amber".into(),
            version: 1,
            type_: Type::Extract,
            amount: 0.50,
            yield_: 78.0,
            color: 13.0,
            add_after_boil: None,
            origin: None,
            supplier: None,
            notes: Some("Hopped amber extract suitable as a base for english ales.".into()),
            coarse_fine_diff: None,
            moisture: None,
            diastatic_power: None,
            protein: None,
            max_in_batch: None,
            recommend_mash: None,
            ibu_gal_per_lb: Some(16.6),
        };
        assert_eq!(parsed_fermentable, true_fermentable);
    }

    #[test]
    fn crystal_malt_specialty_grain_all_applicable_fields() {
        let xml_input = r"
            <FERMENTABLE>
                <NAME>Crystal 40 L</NAME>
                <VERSION>1</VERSION>
                <AMOUNT>0.50</AMOUNT>
                <TYPE>Grain</TYPE>
                <YIELD>74.0</YIELD>
                <COLOR>40.0</COLOR>
                <ORIGIN>United Kingdom</ORIGIN>
                <SUPPLIER>Fussybrewer Malting</SUPPLIER>
                <NOTES>Darker crystal malt.
                Adds body and improves head retention.
                Also called caramel malt.
                </NOTES>
                <COARSE_FINE_DIFF>1.5</COARSE_FINE_DIFF>
                <MOISTURE>4.0</MOISTURE>
                <DIASTATIC_POWER>0.0</DIASTATIC_POWER>
                <PROTEIN>13.2</PROTEIN>
                <MAX_IN_BATCH>10.0</MAX_IN_BATCH>
            </FERMENTABLE>
            ";
        let parsed_fermentable: Fermentable = serde_xml_rs::from_str(xml_input).unwrap();
        let true_fermentable = Fermentable {
            name: "Crystal 40 L".into(),
            version: 1,
            type_: Type::Grain,
            amount: 0.50,
            yield_: 74.0,
            color: 40.0,
            add_after_boil: None,
            origin: Some("United Kingdom".into()),
            supplier: Some("Fussybrewer Malting".into()),
            notes: Some(
                "Darker crystal malt.
                Adds body and improves head retention.
                Also called caramel malt."
                    .into(),
            ),
            coarse_fine_diff: Some(1.5),
            moisture: Some(4.0),
            diastatic_power: Some(0.0),
            protein: Some(13.2),
            max_in_batch: Some(10.0),
            recommend_mash: None,
            ibu_gal_per_lb: None,
        };
        assert_eq!(parsed_fermentable, true_fermentable);
    }
}
