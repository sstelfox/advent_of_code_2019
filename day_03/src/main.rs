use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Down(usize),
    Left(usize),
    Right(usize),
    Up(usize),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let direction = chars.next();
        let magnitude_str: String = chars.collect();

        let magnitude = match magnitude_str.parse::<usize>() {
            Ok(val) => val,
            Err(err) => {
                return Err(format!("Numeric value `{}` isn't a valid usize: {}", magnitude_str, err));
            },
        };

        match direction {
            Some('D') => Ok(Direction::Down(magnitude)),
            Some('L') => Ok(Direction::Left(magnitude)),
            Some('R') => Ok(Direction::Right(magnitude)),
            Some('U') => Ok(Direction::Up(magnitude)),
            _ => Err(format!("Got `{:?}` which is not a valid direction...", direction)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Location {
    x: isize,
    y: isize,
}

impl Location {
    pub fn apply_direction(&self, dir: &Direction) -> Location {
        match dir {
            Direction::Down(v) => Location::new(self.x, self.y - *v as isize),
            Direction::Left(v) => Location::new(self.x - *v as isize, self.y),
            Direction::Right(v) => Location::new(self.x + *v as isize, self.y),
            Direction::Up(v) => Location::new(self.x, self.y + *v as isize),
        }
    }

    /// Calculates the absolute sum of differences between this location and another provided one.
    pub fn manhattan_distance(&self, other: &Location) -> usize {
        let x_dist: usize = (self.x - other.x).abs() as usize;
        let y_dist: usize = (self.y - other.y).abs() as usize;

        x_dist + y_dist
    }

    pub fn new(x: isize, y: isize) -> Self {
        Location {
            x,
            y,
        }
    }
}

pub fn parse_directions(input: &str) -> Result<Vec<Direction>, String> {
    let directions = input.trim().split(',');

    let mut res: Vec<Direction> = Vec::new();
    for dir in directions {
        match Direction::from_str(&dir) {
            Ok(d) => res.push(d),
            Err(err) => { return Err(err); },
        }
    }

    Ok(res)
}

pub fn relative_to_absolute(start: Location, directions: Vec<Direction>) -> Vec<Location> {
    let mut points: Vec<Location> = Vec::new();
    let mut current = start;

    for dir in directions.iter() {
        let new_current = current.apply_direction(&dir);
        points.push(current);
        current = new_current;
    }

    points.push(current);

    points
}

fn main() {
    let mut in_dat_fh = File::open("./data/input_03.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    // TODO:
    //
    // 1. The challenges for this one (and this data file) have two lines, each which needs to be
    //    parsed independently.
    // 2. I need to change the relative directions to absolute coordinates
    // 3. I need to search the two lines for intersections (can't rely on points, have to use
    //    edges)
    // 4. For each intersection calculate the manhattan distance between the intersection and the
    //    origin.
    // 5. Return the distance (w + h) of the intersection with the lowest manhatten distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let reference_point = Location::new(0, 0);

        let good_cases: Vec<(Location, usize)> = vec![
            (Location::new(0, 3), 3),
            (Location::new(3, 0), 3),
            (Location::new(-6, -6), 12),
            (Location::new(-3, 6), 9),
        ];

        for (loc, expected) in good_cases {
            assert_eq!(reference_point.manhattan_distance(&loc), expected);
        }
    }

    #[test]
    fn test_absolute_translation() {
        let good_cases: Vec<(Location, Direction, Location)> = vec![
            (Location::new(12, -3), Direction::Down(9), Location::new(12, -12)),
            (Location::new(7, 38), Direction::Left(7), Location::new(0, 38)),
            (Location::new(7, 38), Direction::Right(100), Location::new(107, 38)),
            (Location::new(0, 0), Direction::Up(4), Location::new(0, 4)),
        ];

        for (loc, dir, expected) in good_cases {
            assert_eq!(loc.apply_direction(&dir), expected);
        }
    }

    #[test]
    fn test_series_of_absolute_translations() {
        let initial_position = Location::new(0, 0);

        let direction_list: Vec<Direction> = vec![
            Direction::Down(73),
            Direction::Down(7),
            Direction::Right(45),
            Direction::Left(20),
            Direction::Up(90),
            Direction::Left(50),
        ];

        let expected_locations: Vec<Location> = vec![
            Location::new(0, 0),
            Location::new(0, -73),
            Location::new(0, -80),
            Location::new(45, -80),
            Location::new(25, -80),
            Location::new(25, 10),
            Location::new(-25, 10),
        ];

        assert_eq!(relative_to_absolute(initial_position, direction_list), expected_locations);
    }

    #[test]
    fn test_individual_direction() {
        let good_cases: Vec<(&'static str, Direction)> = vec![
            ("D23", Direction::Down(23)),
            ("L100", Direction::Left(100)),
            ("R2", Direction::Right(2)),
            ("U12384", Direction::Up(12384)),
        ];

        for (input, expected) in good_cases {
            assert_eq!(Direction::from_str(&input), Ok(expected));
        }
    }

    #[test]
    fn test_parsing_directions() {
        let cases: Vec<(&'static str, Vec<Direction>)> = vec![
            ("R8,U5,L5,D3", vec![Direction::Right(8), Direction::Up(5), Direction::Left(5), Direction::Down(3)]),
            ("U7,R6,D4,L4", vec![Direction::Up(7), Direction::Right(6), Direction::Down(4), Direction::Left(4)]),
        ];

        for (input, expected) in cases {
            let result = parse_directions(&input).unwrap();
            assert_eq!(result, expected);
        }
    }
}
