use serde;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use beer_recipe::hop;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
struct Hops {
    hop: Vec<hop::Hop>,
}

#[test]
fn hops_file_read() {
    let file = File::open("tests/data/hops.xml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let parsed_hop: Hops = serde_xml_rs::from_str(&contents).unwrap();

    assert_eq!(parsed_hop.hop.len(), 5);
}
