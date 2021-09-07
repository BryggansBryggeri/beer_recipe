use beer_recipe::beerxml;
use beer_recipe::bryggio;
use std::f32;
use std::fs;
use std::io::prelude::*;
use std::path;

fn main() {
    let recipe_files = list_recipes("src/bin/recipes");

    let recipes = recipe_files
        .iter()
        .flat_map(read_file_to_string)
        .flat_map(|raw| serde_xml_rs::from_str::<beerxml::Recipe>(&raw));
    let max_hop = recipes
        .map(|recipe| {
            println!("\nRecipe: {}\n---------------", &recipe.name);
            max_hop_amount_in_recipe(&recipe.into())
        })
        .fold(f32::NEG_INFINITY, f32::max);
    println!("\n\nLargest single hop amount found: {}kg/l", &max_hop);
}

fn max_hop_amount_in_recipe(recipe: &bryggio::Recipe) -> f32 {
    let hops = recipe.hops();
    hops.filter(|hop| hop.use_ != beerxml::hop::Use::DryHop)
        .map(|hop| {
            println!(
                "\tHop: {} - {}kg/l",
                &hop.name,
                hop.amount / recipe.batch_size
            );
            hop.amount / recipe.batch_size
        })
        .fold(f32::NEG_INFINITY, f32::max)
}

fn read_file_to_string<P: AsRef<path::Path>>(file_name: P) -> std::io::Result<String> {
    let mut file = fs::File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn list_recipes(recipe_dir: &str) -> Vec<path::PathBuf> {
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
    files
        .filter_map(Result::ok)
        .map(|file| {
            println!(
                "{}",
                file.path().as_path().file_name().unwrap().to_str().unwrap()
            );
            file
        })
        .map(|file| file.path())
        .collect()
}
