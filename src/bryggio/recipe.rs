use std::convert::From;

struct Recipe {
    name: String,
    #[serde(rename = "TYPE")]
    type_: Type,
    style: Style,
    equipment: Option<Equipment>,
    brewer: String,
    asst_brewer: Option<String>,
    batch_size: f32,
    og: Option<f32>,
    fg: Option<f32>,
    /// Not used for `Type::Extract`
    efficiency: f32,
    hops: Vec<Hop>,
    fermentables: Vec<Fermentable>,
    miscs: Vec<Misc>,
    yeasts: Vec<Yeast>,
    waters: Vec<Water>,
    boil: Boil,
    mash: Mash,
    fermentation: Fermentation
    notes: Option<String>,
    taste_notes: Option<String>,
    taste_rating: Option<f32>,
    primary_age: Option<f32>,
    primary_temp: Option<f32>,
    secondary_age: Option<f32>,
    secondary_temp: Option<f32>,
    tertiary_age: Option<f32>,
    tertiary_temp: Option<f32>,
    age: Option<f32>,
    age_temp: Option<f32>,
    date: Option<String>,
    carbonation: Option<f32>,
    #[serde(default)]
    #[serde(deserialize_with = "utils::opt_bool_de_from_str")]
    forced_carbonation: Option<bool>,
    priming_sugar_name: Option<String>,
    carbonation_temp: Option<f32>,
    priming_sugar_equiv: Option<f32>,
    keg_priming_factor: Option<f32>,
}

impl Recipe{
    fn ibu(self) -> Ibu {
    }

    fn measured_abv(self) -> Option<Abv> {
    }

    fn estimated_abv(self) -> Abv {
    }
}
