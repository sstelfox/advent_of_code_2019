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

    pub fn parse(width: usize, height: usize, raw_data: &Vec<u8>) -> Result<Self, &str> {
        let layers = Vec::new();

        let layer_size = width * height;
        if layer_size == 0 {
            return Err("Both height and width need to sizes greater than zero");
        }

        if raw_data.len() == 0 {
            return Err("Provided data can't be zero length");
        }

        if raw_data.len() % layer_size != 0 {
            return Err("Input data could not be broken up into a normal number of layers");
        }

        // TODO: Split data into layers, add them to the vec

        Ok(Self { height, width, layers })
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
    let data_bytes: Vec<u8> = in_dat
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    println!("{:?}", data_bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_parsing() {
        // Reject zero in either height or width
        assert!(Image::parse(0, 100, &Vec::new()).is_err());
        assert!(Image::parse(100, 0, &Vec::new()).is_err());

        // Reject incorrect lengths
        assert!(Image::parse(1, 1, &Vec::new()).is_err());
        assert!(Image::parse(1, 2, &vec![0]).is_err());
        assert!(Image::parse(1, 1, &vec![0]).is_ok());
    }
}
