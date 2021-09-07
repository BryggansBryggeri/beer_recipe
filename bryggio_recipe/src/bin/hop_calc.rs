use beer_recipe::bryggio;
use std::fs;
use std::io::prelude::*;
use std::path;

fn main() {
    let recipe_file = path::Path::new("src/bin/recipes/BellsTwoHeartedAleClone.xml");
    let recipe = read_file_to_string(recipe_file).unwrap();

    let recipe = serde_xml_rs::from_str::<beerxml::Recipe>(&recipe).unwrap();
    let recipe: bryggio::Recipe = recipe.into();
    println!(
        "Total IBU for '{}': {}",
        recipe_file.file_stem().unwrap().to_str().unwrap(),
        recipe.ibu().unwrap()
    );
}

fn read_file_to_string<P: AsRef<path::Path>>(file_name: P) -> std::io::Result<String> {
    let mut file = fs::File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
