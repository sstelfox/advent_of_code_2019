use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct Image {
    height: usize,
    width: usize,

    layers: Vec<Layer>,
}

impl Image {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn parse(width: usize, height: usize, raw_data: &Vec<u8>) -> Self {
        let layers = Vec::new();

        // TODO: Split data into layers, add them to the vec
        unimplemented!();

        Self { height, width, layers }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

#[derive(Debug, PartialEq)]
pub struct Layer {
    data: Vec<u8>,
}

impl Layer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

fn main() {
    let mut in_dat_fh = File::open("./data/input.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let data_bytes: Vec<u8> = in_dat.trim().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();

    println!("{:?}", data_bytes);
}

#[cfg(test)]
mod tests {
    use super::*;
}
