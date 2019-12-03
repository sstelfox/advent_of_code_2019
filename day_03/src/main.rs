use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use itertools::Itertools;

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
                return Err(format!(
                    "Numeric value `{}` isn't a valid usize: {}",
                    magnitude_str, err
                ));
            }
        };

        match direction {
            Some('D') => Ok(Direction::Down(magnitude)),
            Some('L') => Ok(Direction::Left(magnitude)),
            Some('R') => Ok(Direction::Right(magnitude)),
            Some('U') => Ok(Direction::Up(magnitude)),
            _ => Err(format!(
                "Got `{:?}` which is not a valid direction...",
                direction
            )),
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
        Location { x, y }
    }
}

pub fn parse_directions(input: &str) -> Result<Vec<Direction>, String> {
    let directions = input.trim().split(',');

    let mut res: Vec<Direction> = Vec::new();
    for dir in directions {
        match Direction::from_str(&dir) {
            Ok(d) => res.push(d),
            Err(err) => {
                return Err(err);
            }
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
    let lines: Vec<&str> = in_dat.lines().collect();

    let location_set: Option<(Vec<Location>, Vec<Location>)> = lines
        .iter()
        .map(|l| relative_to_absolute(Location::new(0, 0), parse_directions(l).unwrap()))
        .collect_tuple();

    let (_first_location_set, _second_location_set) = match location_set {
        Some(ls) => ls,
        None => {
            println!("Input file didn't have exactly two input lines.");
            std::process::exit(1);
        },
    };

    // TODO:
    //
    // 1. I need to search the two lines for intersections (can't rely on points, have to use
    //    edges)
    // 2. For each intersection calculate the manhattan distance between the intersection and the
    //    origin.
    // 3. Return the distance (w + h) of the intersection with the lowest manhatten distance
}

#[cfg(test)]
mod tests;
