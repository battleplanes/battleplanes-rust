use super::*;

#[test]
fn read_coordletter_from_str() {
    assert_eq!(Some(CoordLetter::A), CoordLetter::new("A"));
    assert_eq!(Some(CoordLetter::B), CoordLetter::new("B"));
    assert_eq!(Some(CoordLetter::C), CoordLetter::new("C"));
    assert_eq!(Some(CoordLetter::D), CoordLetter::new("D"));
    assert_eq!(Some(CoordLetter::E), CoordLetter::new("E"));
    assert_eq!(Some(CoordLetter::F), CoordLetter::new("F"));
    assert_eq!(Some(CoordLetter::G), CoordLetter::new("G"));
    assert_eq!(Some(CoordLetter::H), CoordLetter::new("H"));
    assert_eq!(Some(CoordLetter::I), CoordLetter::new("I"));
    assert_eq!(Some(CoordLetter::J), CoordLetter::new("J"));
    assert_eq!(None, CoordLetter::new("X"));
}
#[test]
fn convert_coordletter_to_str() {
    assert_eq!("A", format!("{}", CoordLetter::new("A").unwrap()));
}
#[test]
fn read_coordnum_from_str() {
    assert_eq!(Some(CoordNum::One), CoordNum::new("1"));
}
#[test]
fn convert_coordnum_to_str() {
    assert_eq!("1", format!("{}", CoordNum::new("1").unwrap()));
}
#[test]
fn move_coordletter() {
    let a = CoordLetter::A;
    assert_eq!(None, a.new_moved_by(-1));
    assert_eq!(Some(CoordLetter::B), a.new_moved_by(1));
    let j = CoordLetter::J;
    assert_eq!(None, j.new_moved_by(1));
    assert_eq!(Some(CoordLetter::I), j.new_moved_by(-1));
}
#[test]
fn move_coordnum() {
    let one = CoordNum::One;
    assert_eq!(None, one.new_moved_by(-1));
    assert_eq!(Some(CoordNum::Two), one.new_moved_by(1));
    let ten = CoordNum::Ten;
    assert_eq!(None, ten.new_moved_by(1));
    assert_eq!(Some(CoordNum::Nine), ten.new_moved_by(-1));
}
#[test]
fn read_coordinate() {
    assert_eq!(Coordinate(CoordLetter::A, CoordNum::Five), Coordinate::new("A5").unwrap());
    assert_eq!(Coordinate(CoordLetter::B, CoordNum::Seven), Coordinate::new("B7").unwrap());
}
#[test]
fn convert_coordinate_to_str() {
    assert_eq!("A5", format!("{}", Coordinate::new("A5").unwrap()));
}
#[test]
fn coordinate_equality() {
    assert_eq!(Coordinate::new("A5"), Coordinate::new("A5"));
    assert!(Coordinate::new("A5") != Coordinate::new("A6"));
}
#[test]
fn move_coordinate_within_reach_of_plane() {
    assert_eq!(Coordinate(CoordLetter::F, CoordNum::Six), Coordinate::new("E5").unwrap().new_moved_by(1, 1).unwrap());
    assert_eq!(Coordinate(CoordLetter::D, CoordNum::Four), Coordinate::new("E5").unwrap().new_moved_by(-1, -1).unwrap());
}
#[test]
fn iterate_tiles_all_visible_north() {
    let p = Plane::new("E5", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates = vec!["C6", "D6", "E6", "F6", "G6", "E7", "D8", "E8", "F8"];
    for expected in expected_coordinates {
        assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_invisible_north() {
    let p = Plane::new("J10", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    for _ in 0..9 {
        assert_eq!(Some(None), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_lefthand_invisible_north() {
    let p = Plane::new("A1", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        None, None, Coordinate::new("A2"), Coordinate::new("B2"), Coordinate::new("C2"),
        Coordinate::new("A3"),
        None, Coordinate::new("A4"), Coordinate::new("B4")
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_righthand_invisible_north() {
    let p = Plane::new("J1", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        Coordinate::new("H2"), Coordinate::new("I2"), Coordinate::new("J2"), None, None,
        Coordinate::new("J3"),
        Coordinate::new("I4"), Coordinate::new("J4"), None,
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_visible_south() {
    let p = Plane::new("E5", "S").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates = vec!["G4", "F4", "E4", "D4", "C4", "E3", "F2", "E2", "D2"];
    for expected in expected_coordinates {
        assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_invisible_south() {
    let p = Plane::new("J10", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    for _ in 0..9 {
        assert_eq!(Some(None), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_lefthand_invisible_south() {
    let p = Plane::new("A1", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        None, None, Coordinate::new("A2"), Coordinate::new("B2"), Coordinate::new("C2"),
        Coordinate::new("A3"),
        None, Coordinate::new("A4"), Coordinate::new("B4")
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_righthand_invisible_south() {
    let p = Plane::new("J1", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        Coordinate::new("H2"), Coordinate::new("I2"), Coordinate::new("J2"), None, None,
        Coordinate::new("J3"),
        Coordinate::new("I4"), Coordinate::new("J4"), None,
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_visible_east() {
    let p = Plane::new("E5", "E").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates = vec!["D3", "D4", "D5", "D6", "D7", "C5", "B4", "B5", "B6"];
    for expected in expected_coordinates {
        assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_invisible_east() {
    let p = Plane::new("A10", "E").unwrap();
    let mut iter = p.coordinate_iterator();
    for _ in 0..9 {
        assert_eq!(Some(None), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_lefthand_invisible_east() {
    let p = Plane::new("J1", "E").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        None, None, Coordinate::new("I1"), Coordinate::new("I2"), Coordinate::new("I3"),
        Coordinate::new("H1"),
        None, Coordinate::new("G1"), Coordinate::new("G2")
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_righthand_invisible_east() {
    let p = Plane::new("J10", "E").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        Coordinate::new("I8"), Coordinate::new("I9"), Coordinate::new("I10"), None, None,
        Coordinate::new("H10"),
        Coordinate::new("G9"), Coordinate::new("G10"), None,
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_visible_west() {
    let p = Plane::new("E5", "W").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates = vec!["F7", "F6", "F5", "F4", "F3", "G5", "H6", "H5", "H4"];
    for expected in expected_coordinates {
        assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_all_invisible_west() {
    let p = Plane::new("J10", "W").unwrap();
    let mut iter = p.coordinate_iterator();
    for _ in 0..9 {
        assert_eq!(Some(None), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn bug_tiles_all_visible_h7_n() {
    let p = Plane::new("H7", "N").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates = vec!["F8", "G8", "H8", "I8", "J8", "H9", "G10", "H10", "I10"];
    for expected in expected_coordinates {
        assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
    }
    assert_eq!(None, iter.next());
}
#[test]
fn bug_tiles_all_visible_a3_w() {
    let p = Plane::new("A3", "W").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates = vec!["B5", "B4", "B3", "B2", "B1", "C3", "D4", "D3", "D2"];
    for expected in expected_coordinates {
        assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_lefthand_invisible_west() {
    let p = Plane::new("A10", "W").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        None, None, Coordinate::new("B10"), Coordinate::new("B9"), Coordinate::new("B8"),
        Coordinate::new("C10"),
        None, Coordinate::new("D10"), Coordinate::new("D9")
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn iterate_tiles_righthand_invisible_west() {
    let p = Plane::new("A1", "W").unwrap();
    let mut iter = p.coordinate_iterator();
    let expected_coordinates : Vec<Option<Coordinate>> = vec![
        Coordinate::new("B3"), Coordinate::new("B2"), Coordinate::new("B1"), None, None,
        Coordinate::new("C1"),
        Coordinate::new("D2"), Coordinate::new("D1"), None,
    ];
    for expected in expected_coordinates {
        assert_eq!(Some(expected), iter.next());
    }
    assert_eq!(None, iter.next());
}
#[test]
fn not_outside_of_map() {
    let heads = vec![("E5", "N"), ("E5", "S"), ("E5", "E"), ("E5", "W")];
    for plane_head in heads {
        let p = Plane::new(plane_head.0, plane_head.1).unwrap();
        assert_eq!(false, p.is_outside_of_map());
    }
}
#[test]
fn outside_of_map() {
    let heads = vec![
        ("A1", "N"), ("A1", "S"), ("A1", "E"), ("A1", "W"),
        ("A10", "N"), ("A10", "S"), ("A10", "E"), ("A10", "W"),
        ("J1", "N"), ("J1", "S"), ("J1", "E"), ("J1", "W"),
        ("J10", "N"), ("J10", "S"), ("J10", "E"), ("J10", "W"),
        ("A2", "N"), ("B1", "W"), ("I1", "N"), ("J2", "E"),
        ("B10", "S"), ("A9", "W"), ("I10", "S"), ("J9", "E"),
    ];
    for plane_head in heads {
        let p = Plane::new(plane_head.0, plane_head.1).unwrap();
        assert_eq!(true, p.is_outside_of_map());
    }
}
#[test]
fn overlapping_planes() {
    let overlapping_pairs = vec![
        (("C1", "N"), ("D1", "N")),
        (("C1", "N"), ("E1", "N")),
        (("C1", "N"), ("F1", "N")),
        (("C1", "N"), ("G1", "N")),
        (("E3", "W"), ("C2", "N")),
    ];
    for plane_positions_pair in overlapping_pairs {
        let p1 = Plane::new((plane_positions_pair.0).0, (plane_positions_pair.0).1).unwrap();
        let p2 = Plane::new((plane_positions_pair.1).0, (plane_positions_pair.1).1).unwrap();
        assert_eq!(true, p1.is_overlapping_with(&p2));
        assert_eq!(true, p2.is_overlapping_with(&p1));
    }
}
#[test]
fn non_overlapping_planes() {
    let overlapping_pairs = vec![
        (("C1", "N"), ("H1", "N")),
    ];
    for plane_positions_pair in overlapping_pairs {
        let p1 = Plane::new((plane_positions_pair.0).0, (plane_positions_pair.0).1).unwrap();
        let p2 = Plane::new((plane_positions_pair.1).0, (plane_positions_pair.1).1).unwrap();
        assert_eq!(false, p1.is_overlapping_with(&p2));
    }
}
#[test]
fn find_plane_at() {
    let mut board = Board::new();
    if let Ok(new_id) = board.add_new_plane_at("E5", "N") {
        let plane = board.get_plane_by_id(new_id).unwrap();

        let inside_coordinates = vec![
            "E5",
            "C6", "D6", "E6", "F6", "G6",
            "E7",
            "D8", "E8", "F8",
        ];
        for raw_coord in inside_coordinates {
            let coord = Coordinate::new(raw_coord).unwrap();
            match board.find_plane_at(&coord) {
                Some(found_plane) => {
                    assert_eq!(plane.id(), found_plane.id());
                },
                None => {
                    panic!("fail for {}", raw_coord);
                },
            }
        }
    }
}
