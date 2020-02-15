use beer_recipe::beerxml::recipe::Recipe;
use std::fs;
use std::io::prelude::*;
use std::path;

fn main() {
    let recipe_files = list_recipes("../recipe/recept_arkiv");
    for file in recipe_files {
        let raw_read = match read_file_to_string(&file.path()) {
            Ok(raw_read) => raw_read,
            Err(err) => panic!("File read"),
        };
        let recipe: Result<Recipe, _> = serde_xml_rs::from_str(&raw_read);
        println!(
            "{}: {}",
            &file.path().as_path().file_name().unwrap().to_str().unwrap(),
            recipe.is_ok()
        );
    }
}

pub fn read_file_to_string<P: AsRef<path::Path>>(file_name: P) -> std::io::Result<String> {
    let mut file = fs::File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn list_recipes(recipe_dir: &str) -> Vec<fs::DirEntry> {
    let device_path = path::Path::new(recipe_dir);
    if !device_path.exists() {
        panic!("No such dir: {}", recipe_dir);
    } else {
    }
    let files = match fs::read_dir(device_path) {
        Ok(files) => files,
        Err(_error) => {
            panic!("Unable to list DSB files {}.", recipe_dir);
        }
    };
    files.filter_map(Result::ok).collect()
}
