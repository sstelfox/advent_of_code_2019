use std::fs::File;
use std::io::Read;

pub fn calculate_fuel(mass: usize) -> usize {
    (mass / 3) - 2
}

fn main() {
    let mut in_dat_fh = File::open("./data/input_01.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let result: usize = in_dat.lines().map(|i| calculate_fuel(i.parse::<usize>().unwrap())).sum();
    println!("Fuel required: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_calculations() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }
}
