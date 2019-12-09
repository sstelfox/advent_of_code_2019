use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct Image {
    height: usize,
    width: usize,

    layers: Vec<Layer>,
}

impl Image {
    pub fn checksum(&self) -> usize {
        // Note: If this was production code I would need to check that layers has > 0 elements and
        // return a Result instead, but that isn't a case I need to worry about here...

        // Find the layer with the fewest zeros
        let mut zero_count = self.layers.iter().enumerate().map(|(i, l)| (i, l.value_count(&Pixel::Black)));
        let (mut min_layer_idx, mut min_layer_count) = zero_count.next().unwrap();

        for (layer_idx, zero_count) in zero_count {
            if min_layer_count > zero_count {
                min_layer_idx = layer_idx;
                min_layer_count = zero_count;
            }
        }

        // Return the product of the count of 1s and 2s on the layer with the fewest zeros per the
        // spec defined in the problem
        self.layers[min_layer_idx].value_count(&Pixel::White) * self.layers[min_layer_idx].value_count(&Pixel::Transparent)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn parse(width: usize, height: usize, raw_data: &[Pixel]) -> Result<Self, &str> {
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
    data: Vec<Pixel>,
}

impl Layer {
    pub fn new(data: Vec<Pixel>) -> Self {
        Self { data }
    }

    pub fn value_count(&self, value: &Pixel) -> usize {
        let mut total = 0;

        for d in &self.data {
            if d == value {
                total += 1;
            }
        }

        total
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pixel {
    Black,
    White,
    Transparent,
}

impl Pixel {
    pub fn from_char(val: &char) -> Result<Self, &str> {
        match val {
            '0' => Ok(Self::Black),
            '1' => Ok(Self::White),
            '2' => Ok(Self::Transparent),
            _ => Err("invalid value attempted to become a pixel"),
        }
    }
}

pub fn str_to_pixels(input: &str) -> Vec<Pixel> {
    input
        .trim()
        .chars()
        .map(|c| Pixel::from_char(&c).unwrap())
        .collect()
}

fn main() {
    let mut in_dat_fh = File::open("./data/input.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let pixels = str_to_pixels(&in_dat);

    let image = Image::parse(25, 6, &pixels).unwrap();
    println!("Checksum: {}", image.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_parsing() {
        // Reject zero in either height or width
        assert!(Image::parse(0, 100, &vec![]).is_err());
        assert!(Image::parse(100, 0, &vec![]).is_err());

        // Reject incorrect lengths
        assert!(Image::parse(1, 1, &vec![]).is_err());
        assert!(Image::parse(1, 2, &vec![Pixel::Black]).is_err());
        assert!(Image::parse(1, 1, &vec![Pixel::Black]).is_ok());
    }

    #[test]
    fn test_modified_official_case() {
        // The official case is "123456789012" but that contains invalid values once the second
        // portion is revealed, I've replaced it with a unique non-repeating pattern containing
        // only valid values
        let input = "001210222011";
        let parsed_input = Image::parse(3, 2, &str_to_pixels(&input)).unwrap();

        let expected_output = Image {
            height: 2,
            width: 3,
            layers: vec![
                Layer::new(vec![Pixel::Black, Pixel::Black, Pixel::White, Pixel::Transparent, Pixel::White, Pixel::Black]),
                Layer::new(vec![Pixel::Transparent, Pixel::Transparent, Pixel::Transparent, Pixel::Black, Pixel::White, Pixel::White]),
            ],
        };

        assert_eq!(parsed_input, expected_output);
    }

    #[test]
    fn test_layer_value_counting() {
        let layer = Layer::new(vec![Pixel::Black, Pixel::White, Pixel::White, Pixel::Black, Pixel::Black, Pixel::Black, Pixel::White]);

        assert_eq!(layer.value_count(&Pixel::Black), 4);
        assert_eq!(layer.value_count(&Pixel::White), 3);
        assert_eq!(layer.value_count(&Pixel::Transparent), 0);
    }

    #[test]
    fn test_checksum() {
        let test_image = Image {
            height: 2,
            width: 3,
            layers: vec![
                // This layer should have a checksum of 4
                Layer::new(vec![Pixel::Black, Pixel::Black, Pixel::White, Pixel::White, Pixel::Transparent, Pixel::Transparent]),
                // This layer should not be selected, but would have a checksum of 2
                Layer::new(vec![Pixel::Black, Pixel::Black, Pixel::Black, Pixel::White, Pixel::White, Pixel::Transparent]),
            ],
        };

        assert_eq!(test_image.checksum(), 4);
    }
}
