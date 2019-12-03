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
        (
            Location::new(12, -3),
            Direction::Down(9),
            Location::new(12, -12),
        ),
        (
            Location::new(7, 38),
            Direction::Left(7),
            Location::new(0, 38),
        ),
        (
            Location::new(7, 38),
            Direction::Right(100),
            Location::new(107, 38),
        ),
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

    assert_eq!(
        relative_to_absolute(initial_position, direction_list),
        expected_locations
    );
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
        (
            "R8,U5,L5,D3",
            vec![
                Direction::Right(8),
                Direction::Up(5),
                Direction::Left(5),
                Direction::Down(3),
            ],
        ),
        (
            "U7,R6,D4,L4",
            vec![
                Direction::Up(7),
                Direction::Right(6),
                Direction::Down(4),
                Direction::Left(4),
            ],
        ),
    ];

    for (input, expected) in cases {
        let result = parse_directions(&input).unwrap();
        assert_eq!(result, expected);
    }
}
