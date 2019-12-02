use std::fs::File;
use std::io::Read;

pub fn calculate_fuel(mass: usize) -> usize {
    let fuel_requirement = mass / 3;

    if fuel_requirement < 2 {
        return 0;
    }

    fuel_requirement - 2
}

// What I should actually do is solve the equation, but this matches the specification in the
// advent challenge and I suspect is less accurate in a way that would effect what the advent
// considers correct. Instead I'll have to implement what was written.
pub fn recursive_fuel_cost(mass: usize) -> usize {
    let mut new_mass = mass;
    let mut total_fuel_mass = 0;

    loop {
        let fuel_mass = calculate_fuel(new_mass);
        if fuel_mass == 0 { break; }

        total_fuel_mass += fuel_mass;
        new_mass = fuel_mass;
    }

    total_fuel_mass
}

fn main() {
    let mut in_dat_fh = File::open("./data/input_01.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let input_masses: Vec<usize> = in_dat.lines().map(|i| i.parse::<usize>().unwrap()).collect();

    let first_result: usize = input_masses.iter().map(|i| calculate_fuel(*i)).sum();
    println!("Fuel required: {}", first_result);

    let second_result: usize = input_masses.iter().map(|i| recursive_fuel_cost(*i)).sum();
    println!("Recursive fuel calculation: {}", second_result);
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

    #[test]
    fn test_recursive_fuel_calculations() {
        assert_eq!(recursive_fuel_cost(12), 2);
        assert_eq!(recursive_fuel_cost(1969), 966);
        assert_eq!(recursive_fuel_cost(100756), 50346);
    }
}
