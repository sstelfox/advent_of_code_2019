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

    pub fn parse(width: usize, height: usize, raw_data: &[u8]) -> Result<Self, &str> {
        let mut layers = Vec::new();
        let mut data = raw_data;

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

        loop {
            let (layer_dat, remaining_data) = data.split_at(layer_size);
            layers.push(Layer::new(layer_dat.to_vec()));
            data = remaining_data;

            if remaining_data.len() == 0 {
                break;
            }
        }

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

    pub fn value_count(&self, value: u8) -> usize {
        let mut total = 0;

        for &d in &self.data {
            if d == value {
                total += 1;
            }
        }

        total
    }
}

pub fn str_to_data_bytes(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

fn main() {
    let mut in_dat_fh = File::open("./data/input.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let data_bytes = str_to_data_bytes(&in_dat);

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

    #[test]
    fn test_official_case() {
        let input = "123456789012";
        let parsed_input = Image::parse(3, 2, &str_to_data_bytes(&input)).unwrap();

        let expected_output = Image {
            height: 2,
            width: 3,
            layers: vec![
                Layer::new(vec![1, 2, 3, 4, 5, 6]),
                Layer::new(vec![7, 8, 9, 0, 1, 2]),
            ],
        };

        assert_eq!(parsed_input, expected_output);
    }

    #[test]
    fn test_layer_value_counting() {
        let layer = Layer::new(vec![0, 1, 1, 2, 3, 2, 1]);

        assert_eq!(layer.value_count(0), 1);
        assert_eq!(layer.value_count(1), 3);
        assert_eq!(layer.value_count(2), 2);
        assert_eq!(layer.value_count(3), 1);
        assert_eq!(layer.value_count(4), 0);
    }
}
