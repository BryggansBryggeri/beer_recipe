use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use beer_recipe::recipe::Recipe;
use beer_recipe::RecordSetList;

#[test]
/// DISASTATIC_POWER -> DIASTATIC_POWER
fn beerxml_recipe() {
    let file = File::open("tests/data/beerxml_recipe.xml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let parsed_record: Recipe = serde_xml_rs::from_str(&contents).unwrap();
}
