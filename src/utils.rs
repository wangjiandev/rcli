use std::{
    fs::File,
    io::{stdin, Read},
};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>, anyhow::Error> {
    if input == "-" {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}
