use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Water {
    name: String,
    version: u8,
    amount: f32,
    calcium: f32,
    bicarbonate: f32,
    sulfate: f32,
    chloride: f32,
    sodium: f32,
    magnesium: f32,
    ph: Option<f32>,
    notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs;
    #[test]
    fn required_fields_only() {
        let xml_input = r"
            <WATER>
                <NAME>Burton on Trent, UK</NAME>
                <VERSION>1</VERSION>
                <AMOUNT>20.0</AMOUNT>
                <CALCIUM>295.0</CALCIUM>
                <MAGNESIUM>45.0</MAGNESIUM>
                <SODIUM>55.0</SODIUM>
                <SULFATE>725.0</SULFATE>
                <CHLORIDE>25.0</CHLORIDE>
                <BICARBONATE>300.0</BICARBONATE>
                <PH>8.0</PH>
                <NOTES>
                Use for distinctive pale ales strongly hopped.  Very hard water accentuates the hops flavor. Example: Bass Ale
                </NOTES>
            </WATER>
        ";
        let parsed_yeast: Water = serde_xml_rs::from_str(xml_input).unwrap();
        let true_yeast = Water {
            name: "Burton on Trent, UK".into(),
            version: 1,
            amount: 20.0,
            calcium: 295.0,
            bicarbonate: 300.0,
            sulfate: 725.0,
            chloride: 25.0,
            sodium: 55.0,
            magnesium: 45.0,
            ph: Some(8.0),
            notes: Some("Use for distinctive pale ales strongly hopped.  Very hard water accentuates the hops flavor. Example: Bass Ale".into()),
        };
        assert_eq!(parsed_yeast, true_yeast);
    }
}
