use std::cmp;
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
            Some('D') => Ok(Self::Down(magnitude)),
            Some('L') => Ok(Self::Left(magnitude)),
            Some('R') => Ok(Self::Right(magnitude)),
            Some('U') => Ok(Self::Up(magnitude)),
            _ => Err(format!(
                "Got `{:?}` which is not a valid direction...",
                direction
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    x: isize,
    y: isize,

    distance: usize,
}

impl Location {
    pub fn apply_direction(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Down(v) => Self::new(self.x, self.y - *v as isize, self.distance + *v),
            Direction::Left(v) => Self::new(self.x - *v as isize, self.y, self.distance + *v),
            Direction::Right(v) => Self::new(self.x + *v as isize, self.y, self.distance + *v),
            Direction::Up(v) => Self::new(self.x, self.y + *v as isize, self.distance + *v),
        }
    }

    /// Calculates the absolute sum of differences between this location and another provided one.
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        let x_dist: usize = (self.x - other.x).abs() as usize;
        let y_dist: usize = (self.y - other.y).abs() as usize;

        x_dist + y_dist
    }

    pub fn new(x: isize, y: isize, distance: usize) -> Self {
        Self { x, y, distance }
    }
}

#[derive(Debug, PartialEq)]
pub struct LineSegment(Location, Location);

impl LineSegment {
    /// This will give the intersecting location of the two lines defined by the line segments but
    /// not necessarily the line segments themselves. The `intersects()` method will indicate
    /// whether or not the intersection occurs at the line segment itself.
    ///
    /// This will return None if the two lines are parallel, even if the two lines are *the same
    /// line*. There is an infinite number of intersections between a line and itself.
    ///
    /// Now that I think about it... I could have just done this and then tested that the resulting
    /// intersection lies on both segments... That's probably would have been way easier... Oh
    /// well...
    pub fn intersecting_location(&self, other: &Self) -> Option<Location> {
        // Get our 'self' line segments in 0 = ax + by + c form
        let self_a = self.1.y - self.0.y;
        let self_b = self.0.x - self.1.x;
        let self_c = self_a * self.0.x + self_b * self.0.y;

        let other_a = other.1.y - other.0.y;
        let other_b = other.0.x - other.1.x;
        let other_c = other_a * other.0.x + other_b * other.0.y;

        let determinant = self_a * other_b - other_a * self_b;

        // The lines are parallel, but could be the same line. For us we only care if an endpoint
        // matches one of the other lines endpoints. If they overlap more than that there are
        // infinite matching points and we'll just bail out without finding a point.
        if determinant == 0 {
            if self.0 == other.0 {
                return Some(Location::new(
                    self.0.x,
                    self.0.y,
                    self.0.distance + other.0.distance,
                ));
            }

            if self.0 == other.1 {
                return Some(Location::new(
                    self.0.x,
                    self.0.y,
                    self.0.distance + other.1.distance,
                ));
            }

            if self.1 == other.0 {
                return Some(Location::new(
                    self.1.x,
                    self.1.y,
                    self.1.distance + other.0.distance,
                ));
            }

            if self.1 == other.1 {
                return Some(Location::new(
                    self.1.x,
                    self.1.y,
                    self.1.distance + other.1.distance,
                ));
            }

            return None;
        }

        let x = (other_b * self_c - self_b * other_c) / determinant;
        let y = (self_a * other_c - other_a * self_c) / determinant;

        // Calculate the new distance the intersection will be at using a temporary point
        let new_point = Location::new(x, y, 0);
        let first_distance = self.0.manhattan_distance(&new_point);
        let second_distance = other.0.manhattan_distance(&new_point);
        let new_distance = self.0.distance + first_distance + other.0.distance + second_distance;

        Some(Location::new(x, y, new_distance))
    }

    /// This one is a bit trickier to explain. This calculates all of the possible three point
    /// orientation combinations of the lines with points on the other line (the inverse ordering
    /// doesn't matter as it will always either be the opposite or they'll both by definition still be
    /// colinear).
    ///
    /// The possible conditions are:
    ///
    /// 1.  The line segments are intersecting
    /// 2.  The lines (if continuing on forever) would intersect but the segments do not
    /// 3.  The lines will never intersect (parallel, non-colinear)
    /// 4.  The line segments are colinear and do not overlap (no intersection)
    /// 5.  The line segments are colinear and overlap (infinite solutions), for us this has finite
    ///     solutions as we only care about whole number intersections. This is also likely not to
    ///     happen with our data sets.
    ///
    /// When l1-l2 & l3-l4 intersect (l1, l2, l3) and (l1, l2, l4) will have different orientations
    /// (the virtual lines l2-l3, and l2-l4 will rotate to either side of the l1-l2 line, This doesn't
    /// catch the case where either l3 or l4 is on the line l1-l2 or when the lines would intersect but
    /// the segments do not. To catch this we also need to check that (l3, l4, l1) and (l3, l4, l2)
    /// also have different orientations. This covers the cases 1 & 2 which are the general cases.
    ///
    /// To decide if 3 or 4 (both are false for intersections) is true we need to eliminate the
    /// possibility 5. If the orientation of any of the sets are colinear then we need to check if the
    /// last point in the set is on the segment of line of the between the first two in the set. If
    /// this is true for any of the combinations then then the line segments overlap.
    pub fn intersects(&self, other: &Self) -> bool {
        let orientations: [Orientation; 4] = [
            Orientation::from_three_locations(&self.0, &self.1, &other.0),
            Orientation::from_three_locations(&self.0, &self.1, &other.1),
            Orientation::from_three_locations(&other.0, &other.1, &self.0),
            Orientation::from_three_locations(&other.0, &other.1, &self.1),
        ];

        // The first case is proven true through these orientation differences, it seems like this can
        // be simplified somehow but it's not immediately obvious to me. That's fine this is probably
        // fine.
        if orientations[0] != orientations[1] && orientations[2] != orientations[3] {
            return true;
        }

        // If one of these are true, then the points are colinear and overlapping
        if orientations[0] == Orientation::Colinear && self.is_present(&other.0) {
            return true;
        }

        if orientations[1] == Orientation::Colinear && self.is_present(&other.1) {
            return true;
        }

        if orientations[2] == Orientation::Colinear && other.is_present(&self.0) {
            return true;
        }

        if orientations[3] == Orientation::Colinear && other.is_present(&self.1) {
            return true;
        }

        // The lines are parallel and non-overlapping (may be colinear)
        false
    }

    /// Checks whether the point is present on this line segment
    pub fn is_present(&self, point: &Location) -> bool {
        point.x <= cmp::max(self.0.x, self.1.x)
            && point.x >= cmp::min(self.0.x, self.1.x)
            && point.y <= cmp::max(self.0.y, self.1.y)
            && point.y >= cmp::min(self.0.y, self.1.y)
    }
}

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Clockwise,
    CounterClockwise,
    Colinear,
}

impl Orientation {
    /// This caculates the three point orientation of any three points so we can determine the
    /// relation between the points for the edge and general cases of segment intersection. This is
    /// calculated using the slope between p1/p2, and p2/p3. If the slope is the same
    /// (difference of zero) the two lines are colinear. If the slope of p1/p2 is less than p2/p3
    /// than the p2/p3 slope is bending counterclockwise from the p1/p2 slope, when it's more it's
    /// bending more clockwise from the slope.
    ///
    /// These orientations can be used to quickly check whether the segments intersect at all. If
    /// so we can then go on to attempt to solve the equations to get the answer.
    pub fn from_three_locations(l1: &Location, l2: &Location, l3: &Location) -> Self {
        let orientation = (l2.y - l1.y) * (l3.x - l2.x) - (l2.x - l1.x) * (l3.y - l2.y);

        match orientation {
            orient if orient < 0 => Self::CounterClockwise,
            orient if orient > 0 => Self::Clockwise,
            _ => Self::Colinear,
        }
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

pub fn relative_to_absolute(start: Location, directions: &[Direction]) -> Vec<Location> {
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

pub fn location_set_to_line_set(location_set: Vec<Location>) -> Vec<LineSegment> {
    let mut line_segments: Vec<LineSegment> = Vec::new();

    let mut set_iter = location_set.into_iter();
    let mut last_element = if let Some(e) = set_iter.next() {
        e
    } else {
        // No locations were provided
        return line_segments;
    };

    for next_element in set_iter {
        line_segments.push(LineSegment(last_element, next_element.clone()));
        last_element = next_element;
    }

    line_segments
}

fn main() {
    let mut in_dat_fh = File::open("./data/input_03.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let lines: Vec<&str> = in_dat.lines().collect();

    let location_set: Option<(Vec<Location>, Vec<Location>)> = lines
        .iter()
        .map(|l| relative_to_absolute(Location::new(0, 0, 0), &parse_directions(&l).unwrap()))
        .collect_tuple();

    // TODO:
    //
    // 1. I need to search the two lines for intersections (can't rely on points, have to use
    //    edges). Alright once again I've got two ways forward.
    //
    //    I can do the naive thing and build the ascii map as the example does and record all the
    //    intersections only made between the two lines. I would have to use slightly different
    //    indicators to be able to differentiate the two lines. This would unecessarily use a
    //    pretty crazy amount of memory but I would get cool ASCII maps out of it.
    //
    //    The other option and the one that seems correct is to solve a system of equations over
    //    each set of points looking for intersections and recording those. It initially seems
    //    harder but I think it's going to be signficantly faster both to run and to code as there
    //    won't be any of the odd edge cases as there would be with the ASCII maps.
    //
    //    There is one odd case that I don't know how this intersection check should behave, which
    //    is the condition where the two line segments are overlapping and colinear. Is each
    //    integer point an intersection? Only the end? None of them? I'm guessing each point for
    //    now, but I'd also guess this probably won't come up.
    //
    //    The only portion I have left is calculating the actual intersection between line segments
    //    and iterating through the possibility space.
    //
    //    I expect the output of this step to be a series of locations where the two paths have
    //    intersected.
    // 2. For each intersection calculate the manhattan distance between the intersection and the
    //    origin. Pretty straight forward, already have this written just need the points from the
    //    last step.
    // 3. Return the distance (w + h) of the intersection with the lowest manhatten distance. Also
    //    straight forward, this just needs to do a min() over the results from the last step.

    let (first_location_set, second_location_set) = if let Some(ls) = location_set {
        ls
    } else {
        println!("Input file didn't have exactly two input lines.");
        std::process::exit(1);
    };

    let mut intersection_list: Vec<Location> = Vec::new();

    let first_line_set = location_set_to_line_set(first_location_set);
    let second_line_set = location_set_to_line_set(second_location_set);

    for first_line in &first_line_set {
        for second_line in &second_line_set {
            if first_line.intersects(&second_line) {
                // We know these two lines intersect now, I just have to calculate the position
                // they intersect at.
                match first_line.intersecting_location(&second_line) {
                    Some(loc) => intersection_list.push(loc),
                    None => {
                        // This is a weird edge case where the two line segments representing the
                        // same line and are overlapping. This means one end of the line segment is
                        // in the other one. We need to figure out which one then add that to our
                        // list
                        if first_line.is_present(&second_line.0) {
                            intersection_list.push(second_line.0.clone());
                        } else if first_line.is_present(&second_line.1) {
                            intersection_list.push(second_line.1.clone());
                        } else {
                            // This should never be the case but log it in case something extremely
                            // weird happens...
                            println!(
                                "Weird intersection case: {:?}, {:?}",
                                first_line, second_line
                            );
                        }
                    }
                }
            }
        }
    }

    println!(
        "Found {} intersections in data set",
        intersection_list.len()
    );

    // Only thing left is to calculate the distances and return the smallest intersection. We'll be
    // calculating from the origin, and due to how the relative to absolute positioning works, our
    // first intersection should be at the origin (which we also want to remove so we can get a
    // valid answer).
    let mut intersection_iter = intersection_list.iter();
    let origin = if let Some(o) = intersection_iter.next() {
        if o != &Location::new(0, 0, 0) {
            println!(
                "Expectation fail, the first intersection wasn't the origin: {:?}",
                o
            );
            std::process::exit(1);
        }

        o
    } else {
        println!("Expectation fail, there should be at least one intersection right?");
        std::process::exit(1);
    };

    match intersection_iter
        .map(|il| origin.manhattan_distance(&il))
        .min()
    {
        Some(min_dist) => println!("Minimum distance to intersection is: {}", min_dist),
        None => println!("Couldn't find the minimum distance..."),
    }

    let mut intersection_iter = intersection_list.iter();
    // Discard the first one as it is our origin and has a distance of 0
    intersection_iter.next();

    // For part two we need to find the intersection that had the smallest total distance
    let min_location = intersection_iter.map(|l| l.distance).min();
    println!("Minimum intersection distance: {:?}", min_location);
}

#[cfg(test)]
mod tests;
