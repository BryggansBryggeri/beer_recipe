use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use beerxml::recipe::Recipe;

/// DISASTATIC_POWER -> DIASTATIC_POWER
#[test]
fn beerxml_recipe() {
    let file = File::open("tests/data/beerxml/recipe.xml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let parsed_record: Recipe = serde_xml_rs::from_str(&contents).unwrap();
}
