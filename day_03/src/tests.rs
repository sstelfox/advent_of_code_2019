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
        relative_to_absolute(initial_position, &direction_list),
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

#[test]
fn test_location_orientation() {
    let cases: Vec<(Location, Location, Location, Orientation)> = vec![
        (
            Location::new(0, 0),
            Location::new(0, 5),
            Location::new(0, 10),
            Orientation::Colinear,
        ),
        (
            Location::new(0, 0),
            Location::new(0, 5),
            Location::new(5, 10),
            Orientation::Clockwise,
        ),
        (
            Location::new(0, 0),
            Location::new(0, 5),
            Location::new(-5, 10),
            Orientation::CounterClockwise,
        ),
        (
            Location::new(0, 0),
            Location::new(4, 4),
            Location::new(1, 1),
            Orientation::Colinear,
        ),
        (
            Location::new(0, 0),
            Location::new(4, 4),
            Location::new(1, 2),
            Orientation::CounterClockwise,
        ),
    ];

    for (p1, p2, p3, orientation) in cases {
        assert_eq!(
            Orientation::from_three_locations(&p1, &p2, &p3),
            orientation
        );
    }
}

#[test]
fn test_location_on_segments() {
    let cases: Vec<(Location, Location, Location, bool)> = vec![
        (
            Location::new(0, 0),
            Location::new(0, 10),
            Location::new(0, 5),
            true,
        ),
        (
            Location::new(1, 1),
            Location::new(5, 5),
            Location::new(3, 3),
            true,
        ),
        (
            Location::new(1, 1),
            Location::new(5, 5),
            Location::new(3, 0),
            false,
        ),
        (
            Location::new(1, 1),
            Location::new(1, 1),
            Location::new(1, 1),
            true,
        ),
    ];

    for (p1, p2, p3, expectation) in cases {
        assert_eq!(LineSegment(p1, p2).is_present(&p3), expectation);
    }
}

#[test]
fn test_intersection_checks() {
    let cases: Vec<(Location, Location, Location, Location, bool)> = vec![
        // Normal intersection
        (Location::new(1, 1), Location::new(5, 5), Location::new(5, 1), Location::new(1, 5), true),

        // Overlapping endpoint
        (Location::new(1, 1), Location::new(5, 5), Location::new(3, 3), Location::new(1, 6), true),

        // Non-intersecting segments (the lines would intersect)
        (Location::new(-5, 3), Location::new(5, 3), Location::new(0, -5), Location::new(0, 0), false),

        // Non-intersecting segments (the lines would intersect at an endpoint)
        (Location::new(-5, 3), Location::new(5, 3), Location::new(-5, -5), Location::new(-5, 0), false),

        // Parallel but non-intersecting
        (Location::new(1, 1), Location::new(5, 5), Location::new(1, 2), Location::new(5, 6), false),

        // Colinear and intersecting
        (Location::new(-5, 0), Location::new(-1, 0), Location::new(-2, 0), Location::new(3, 0), true),

        // Colinear and non-intersecting
        (Location::new(-7, 2), Location::new(-4, 2), Location::new(0, 2), Location::new(4, 2), false),
    ];

    for (p1, p2, p3, p4, expectation) in cases {
        let line_seg1 = LineSegment(p1, p2);
        let line_seg2 = LineSegment(p3, p4);

        assert_eq!(line_seg1.intersects(&line_seg2), expectation);
    }
}

#[test]
fn test_location_set_to_line_set() {
    let location_set = vec![];
    let line_set: Vec<LineSegment> = vec![];
    assert_eq!(location_set_to_line_set(location_set), line_set);

    // One location isn't enough to make a line
    let location_set = vec![Location::new(0, 0)];
    let line_set: Vec<LineSegment> = vec![];
    assert_eq!(location_set_to_line_set(location_set), line_set);

    // Two is, and here after I'd expect N-1 line segments
    let location_set = vec![Location::new(-12, 56), Location::new(3, 7)];
    let line_set: Vec<LineSegment> = vec![LineSegment(Location::new(-12, 56), Location::new(3, 7))];
    assert_eq!(location_set_to_line_set(location_set), line_set);

    let location_set = vec![Location::new(1, 2), Location::new(3, 4), Location::new(5, 6)];
    let line_set: Vec<LineSegment> = vec![
        LineSegment(Location::new(1, 2), Location::new(3, 4)),
        LineSegment(Location::new(3, 4), Location::new(5, 6)),
    ];
    assert_eq!(location_set_to_line_set(location_set), line_set);
}

#[test]
fn test_line_segment_intersection_calculation() {
    let cases: Vec<(Location, Location, Location, Location, Option<Location>)> = vec![
        // Parallel
        (Location::new(1, 1), Location::new(1, 2), Location::new(2, 1), Location::new(2, 2), None),

        // Meet at origin (overlapping line segments)
        (Location::new(0, 2), Location::new(0, -2), Location::new(2, 0), Location::new(-2, 0), Some(Location::new(0, 0))),

        // Meet at a non-overlapping location
        (Location::new(1, 5), Location::new(2, 6), Location::new(1, 9), Location::new(2, 8), Some(Location::new(3, 7))),

        // Parallel touching at one point only
        (Location::new(0, 0), Location::new(9, 0), Location::new(0, 0), Location::new(-9, 0), Some(Location::new(0, 0))),
    ];

    for (l1, l2, l3, l4, result) in cases {
        let line_seg1 = LineSegment(l1, l2);
        let line_seg2 = LineSegment(l3, l4);

        assert_eq!(line_seg1.intersecting_location(&line_seg2), result);
    }
}
