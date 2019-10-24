mod equipment;
mod fermentable;
mod hop;
mod mash;
mod misc;
mod recipe;
mod style;
mod water;
mod yeast;

pub type Percent = f32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
