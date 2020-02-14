use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use beer_recipe::beerxml::hop;

#[test]
fn hops_file_read() {
    let file = File::open("tests/data/beerxml/hops.xml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let parsed_record: RecordSetList<hop::Hop> = serde_xml_rs::from_str(&contents).unwrap();
    assert_eq!(parsed_record.hop.len(), 5);
}
