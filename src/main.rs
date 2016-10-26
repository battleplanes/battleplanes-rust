/// # GistID: 88581d0eeffef36e8f90b89c6ba2234a

use std::fmt;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub enum CoordLetter {
    A, B, C, D, E, F, G, H, I, J,
}

impl CoordLetter {
    pub fn new(from: &str) -> Option<CoordLetter> {
        match from {
            //TODO: a nicer way
            "A" => Some(CoordLetter::A),
            "B" => Some(CoordLetter::B),
            "C" => Some(CoordLetter::C),
            "D" => Some(CoordLetter::D),
            "E" => Some(CoordLetter::E),
            "F" => Some(CoordLetter::F),
            "G" => Some(CoordLetter::G),
            "H" => Some(CoordLetter::H),
            "I" => Some(CoordLetter::I),
            "J" => Some(CoordLetter::J),
            _ => None
        }
    }
    pub fn new_moved_by(&self, offset: i32) -> Option<CoordLetter> {
        match (self, offset) {
            //TODO: a nicer way
            (_, 0) => Some(self.clone()),
            (&CoordLetter::A, 1) => Some(CoordLetter::B),
            (&CoordLetter::B, 1) => Some(CoordLetter::C),
            (&CoordLetter::C, 1) => Some(CoordLetter::D),
            (&CoordLetter::D, 1) => Some(CoordLetter::E),
            (&CoordLetter::E, 1) => Some(CoordLetter::F),
            (&CoordLetter::F, 1) => Some(CoordLetter::G),
            (&CoordLetter::G, 1) => Some(CoordLetter::H),
            (&CoordLetter::H, 1) => Some(CoordLetter::I),
            (&CoordLetter::I, 1) => Some(CoordLetter::J),
            (&CoordLetter::B, -1) => Some(CoordLetter::A),
            (&CoordLetter::C, -1) => Some(CoordLetter::B),
            (&CoordLetter::D, -1) => Some(CoordLetter::C),
            (&CoordLetter::E, -1) => Some(CoordLetter::D),
            (&CoordLetter::F, -1) => Some(CoordLetter::E),
            (&CoordLetter::G, -1) => Some(CoordLetter::F),
            (&CoordLetter::H, -1) => Some(CoordLetter::G),
            (&CoordLetter::I, -1) => Some(CoordLetter::H),
            (&CoordLetter::J, -1) => Some(CoordLetter::I),
            (&CoordLetter::A, 2) => Some(CoordLetter::C),
            (&CoordLetter::B, 2) => Some(CoordLetter::D),
            (&CoordLetter::C, 2) => Some(CoordLetter::E),
            (&CoordLetter::D, 2) => Some(CoordLetter::F),
            (&CoordLetter::E, 2) => Some(CoordLetter::G),
            (&CoordLetter::F, 2) => Some(CoordLetter::H),
            (&CoordLetter::G, 2) => Some(CoordLetter::I),
            (&CoordLetter::H, 2) => Some(CoordLetter::J),
            (&CoordLetter::C, -2) => Some(CoordLetter::A),
            (&CoordLetter::D, -2) => Some(CoordLetter::B),
            (&CoordLetter::E, -2) => Some(CoordLetter::C),
            (&CoordLetter::F, -2) => Some(CoordLetter::D),
            (&CoordLetter::G, -2) => Some(CoordLetter::E),
            (&CoordLetter::H, -2) => Some(CoordLetter::F),
            (&CoordLetter::I, -2) => Some(CoordLetter::G),
            (&CoordLetter::J, -2) => Some(CoordLetter::H),
            (&CoordLetter::A, 3) => Some(CoordLetter::D),
            (&CoordLetter::B, 3) => Some(CoordLetter::E),
            (&CoordLetter::C, 3) => Some(CoordLetter::F),
            (&CoordLetter::D, 3) => Some(CoordLetter::G),
            (&CoordLetter::E, 3) => Some(CoordLetter::H),
            (&CoordLetter::F, 3) => Some(CoordLetter::I),
            (&CoordLetter::G, 3) => Some(CoordLetter::J),
            (&CoordLetter::D, -3) => Some(CoordLetter::A),
            (&CoordLetter::E, -3) => Some(CoordLetter::B),
            (&CoordLetter::F, -3) => Some(CoordLetter::C),
            (&CoordLetter::G, -3) => Some(CoordLetter::D),
            (&CoordLetter::H, -3) => Some(CoordLetter::E),
            (&CoordLetter::I, -3) => Some(CoordLetter::F),
            (&CoordLetter::J, -3) => Some(CoordLetter::G),
            _ => None,
        }
    }
}

impl Copy for CoordLetter { }
impl Clone for CoordLetter {
    fn clone(&self) -> CoordLetter {
        *self
    }
}

impl PartialEq for CoordLetter {
    fn eq(&self, other: &CoordLetter) -> bool {
        match (self, other) {
            // TODO: a nicer way
            (&CoordLetter::A, &CoordLetter::A) => true,
            (&CoordLetter::B, &CoordLetter::B) => true,
            (&CoordLetter::C, &CoordLetter::C) => true,
            (&CoordLetter::D, &CoordLetter::D) => true,
            (&CoordLetter::E, &CoordLetter::E) => true,
            (&CoordLetter::F, &CoordLetter::F) => true,
            (&CoordLetter::G, &CoordLetter::G) => true,
            (&CoordLetter::H, &CoordLetter::H) => true,
            (&CoordLetter::I, &CoordLetter::I) => true,
            (&CoordLetter::J, &CoordLetter::J) => true,
            _ => false,
        }
    }
}

impl From<usize> for CoordLetter {
    fn from(u: usize) -> CoordLetter {
        match u % 10 {
            0 => CoordLetter::A,
            1 => CoordLetter::B,
            2 => CoordLetter::C,
            3 => CoordLetter::D,
            4 => CoordLetter::E,
            5 => CoordLetter::F,
            6 => CoordLetter::G,
            7 => CoordLetter::H,
            8 => CoordLetter::I,
            _ => CoordLetter::J,
        }
    }
}

impl fmt::Display for CoordLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            // TODO: a nicer way
            &CoordLetter::A => "A",
            &CoordLetter::B => "B",
            &CoordLetter::C => "C",
            &CoordLetter::D => "D",
            &CoordLetter::E => "E",
            &CoordLetter::F => "F",
            &CoordLetter::G => "G",
            &CoordLetter::H => "H",
            &CoordLetter::I => "I",
            &CoordLetter::J => "J",
        })
    }
}

#[derive(Debug)]
pub enum CoordNum {
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
}

impl CoordNum {
    pub fn new(from: &str) -> Option<CoordNum> {
        match from {
            // TODO: a nicer way
            "1" => Some(CoordNum::One),
            "2" => Some(CoordNum::Two),
            "3" => Some(CoordNum::Three),
            "4" => Some(CoordNum::Four),
            "5" => Some(CoordNum::Five),
            "6" => Some(CoordNum::Six),
            "7" => Some(CoordNum::Seven),
            "8" => Some(CoordNum::Eight),
            "9" => Some(CoordNum::Nine),
            "10" => Some(CoordNum::Ten),
            _ => None,
        }
    }
    pub fn new_moved_by(&self, offset: i32) -> Option<CoordNum> {
        match (self, offset) {
            // TODO: a nicer way
            (_, 0) => Some(self.clone()),
            (&CoordNum::One, 1) => Some(CoordNum::Two),
            (&CoordNum::Two, 1) => Some(CoordNum::Three),
            (&CoordNum::Three, 1) => Some(CoordNum::Four),
            (&CoordNum::Four, 1) => Some(CoordNum::Five),
            (&CoordNum::Five, 1) => Some(CoordNum::Six),
            (&CoordNum::Six, 1) => Some(CoordNum::Seven),
            (&CoordNum::Seven, 1) => Some(CoordNum::Eight),
            (&CoordNum::Eight, 1) => Some(CoordNum::Nine),
            (&CoordNum::Nine, 1) => Some(CoordNum::Ten),
            (&CoordNum::Two, -1) => Some(CoordNum::One),
            (&CoordNum::Three, -1) => Some(CoordNum::Two),
            (&CoordNum::Four, -1) => Some(CoordNum::Three),
            (&CoordNum::Five, -1) => Some(CoordNum::Four),
            (&CoordNum::Six, -1) => Some(CoordNum::Five),
            (&CoordNum::Seven, -1) => Some(CoordNum::Six),
            (&CoordNum::Eight, -1) => Some(CoordNum::Seven),
            (&CoordNum::Nine, -1) => Some(CoordNum::Eight),
            (&CoordNum::Ten, -1) => Some(CoordNum::Nine),
            (&CoordNum::One, 2) => Some(CoordNum::Three),
            (&CoordNum::Two, 2) => Some(CoordNum::Four),
            (&CoordNum::Three, 2) => Some(CoordNum::Five),
            (&CoordNum::Four, 2) => Some(CoordNum::Six),
            (&CoordNum::Five, 2) => Some(CoordNum::Seven),
            (&CoordNum::Six, 2) => Some(CoordNum::Eight),
            (&CoordNum::Seven, 2) => Some(CoordNum::Nine),
            (&CoordNum::Eight, 2) => Some(CoordNum::Ten),
            (&CoordNum::Three, -2) => Some(CoordNum::One),
            (&CoordNum::Four, -2) => Some(CoordNum::Two),
            (&CoordNum::Five, -2) => Some(CoordNum::Three),
            (&CoordNum::Six, -2) => Some(CoordNum::Four),
            (&CoordNum::Seven, -2) => Some(CoordNum::Five),
            (&CoordNum::Eight, -2) => Some(CoordNum::Six),
            (&CoordNum::Nine, -2) => Some(CoordNum::Seven),
            (&CoordNum::Ten, -2) => Some(CoordNum::Eight),
            (&CoordNum::One, 3) => Some(CoordNum::Four),
            (&CoordNum::Two, 3) => Some(CoordNum::Five),
            (&CoordNum::Three, 3) => Some(CoordNum::Six),
            (&CoordNum::Four, 3) => Some(CoordNum::Seven),
            (&CoordNum::Five, 3) => Some(CoordNum::Eight),
            (&CoordNum::Six, 3) => Some(CoordNum::Nine),
            (&CoordNum::Seven, 3) => Some(CoordNum::Nine),
            (&CoordNum::Four, -3) => Some(CoordNum::One),
            (&CoordNum::Five, -3) => Some(CoordNum::Two),
            (&CoordNum::Six, -3) => Some(CoordNum::Three),
            (&CoordNum::Seven, -3) => Some(CoordNum::Four),
            (&CoordNum::Eight, -3) => Some(CoordNum::Five),
            (&CoordNum::Nine, -3) => Some(CoordNum::Six),
            (&CoordNum::Ten, -3) => Some(CoordNum::Seven),
            _ => None,
        }
    }
}

impl Copy for CoordNum { }
impl Clone for CoordNum {
    fn clone(&self) -> CoordNum {
        *self
    }
}

impl PartialEq for CoordNum {
    fn eq(&self, other: &CoordNum) -> bool {
        match(self, other) {
            // TODO: a nicer way
            (&CoordNum::One, &CoordNum::One) => true,
            (&CoordNum::Two, &CoordNum::Two) => true,
            (&CoordNum::Three, &CoordNum::Three) => true,
            (&CoordNum::Four, &CoordNum::Four) => true,
            (&CoordNum::Five, &CoordNum::Five) => true,
            (&CoordNum::Six, &CoordNum::Six) => true,
            (&CoordNum::Seven, &CoordNum::Seven) => true,
            (&CoordNum::Eight, &CoordNum::Eight) => true,
            (&CoordNum::Nine, &CoordNum::Nine) => true,
            (&CoordNum::Ten, &CoordNum::Ten) => true,
            _ => false,
        }
    }
}

impl From<usize> for CoordNum {
    fn from(u: usize) -> CoordNum {
        match u % 10 {
            0 => CoordNum::One,
            1 => CoordNum::Two,
            2 => CoordNum::Three,
            3 => CoordNum::Four,
            4 => CoordNum::Five,
            5 => CoordNum::Six,
            6 => CoordNum::Seven,
            7 => CoordNum::Eight,
            8 => CoordNum::Nine,
            _ => CoordNum::Ten,
        }
    }
}

impl fmt::Display for CoordNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            // TODO: a nicer way
            &CoordNum::One => "1",
            &CoordNum::Two => "2",
            &CoordNum::Three => "3",
            &CoordNum::Four => "4",
            &CoordNum::Five => "5",
            &CoordNum::Six => "6",
            &CoordNum::Seven => "7",
            &CoordNum::Eight => "8",
            &CoordNum::Nine => "9",
            &CoordNum::Ten => "10",
        })
    }
}

#[derive(Debug)]
pub struct Coordinate (CoordLetter, CoordNum);

impl Coordinate {
    pub fn new(from: &str) -> Option<Coordinate> {
        match (CoordLetter::new(&from[0..1]), CoordNum::new(&from[1..])) {
            (Some(letter), Some(number)) => Some(Coordinate(letter, number)),
            _ => None,
        }
    }
    pub fn new_moved_by(&self, x: i32, y: i32) -> Option<Coordinate> {
        match (self.0.new_moved_by(x), self.1.new_moved_by(y)) {
            (Some(moved_letter), Some(moved_num)) => Some(Coordinate(moved_letter, moved_num)),
            _ => None,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn new(from: &str) -> Option<Orientation> {
        match from {
            // TODO: a nicer way
            "N" => Some(Orientation::North),
            "E" => Some(Orientation::East),
            "S" => Some(Orientation::South),
            "W" => Some(Orientation::West),
            _ => None,
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            // TODO: a nicer way
            &Orientation::North => "North",
            &Orientation::South => "South",
            &Orientation::East => "East",
            &Orientation::West => "West",
        })
    }
}

pub struct PlanePositionIterator<'a> {
    head: &'a Coordinate,
    orientation: &'a Orientation,
    current_tile: usize,
}

impl<'a> Iterator for PlanePositionIterator<'a> {
    type Item = Option<Coordinate>;
    fn next(&mut self) -> Option<Option<Coordinate>> {
        // TODO: a nicer way
        if self.current_tile > 8 {
            return None;
        }
        let tiles = match self.orientation {
            &Orientation::North => vec![(-2,  1), (-1,  1), ( 0,  1), ( 1,  1), ( 2,  1), ( 0,  2), (-1,  3), ( 0,  3), ( 1,  3)],
            &Orientation::South => vec![( 2, -1), ( 1, -1), ( 0, -1), (-1, -1), (-2, -1), ( 0, -2), ( 1, -3), ( 0, -3), (-1, -3)],
            &Orientation::East =>  vec![(-1, -2), (-1, -1), (-1,  0), (-1,  1), (-1,  2), (-2,  0), (-3, -1), (-3,  0), (-3,  1)],
            &Orientation::West =>  vec![( 1,  2), ( 1,  1), ( 1,  0), ( 1, -1), ( 1, -2), ( 2,  0), ( 3,  1), ( 3,  0), ( 3, -1)],
        };
        let move_x = tiles[self.current_tile].0;
        let move_y = tiles[self.current_tile].1;
        self.current_tile += 1;
        let t = self.head.new_moved_by(move_x, move_y);
        Some(t)
    }
}

pub struct Plane {
    head: Coordinate,
    orientation: Orientation,
}

impl Plane {
    pub fn new(from: &str, orientation: &str) -> Option<Plane> {
        match (Coordinate::new(from), Orientation::new(orientation)) {
            (Some(c), Some(o)) => Some(Plane {
                head: c,
                orientation: o,
            }),
            _ => None,
        }
    }
    pub fn tile_iterator(&self) -> PlanePositionIterator {
        PlanePositionIterator {
            head: &self.head,
            orientation: &self.orientation,
            current_tile: 0,
        }
    }
    pub fn is_outside_of_map(&self) -> bool {
        self.tile_iterator().find(|x: &Option<Coordinate>| *x == None) == Some(None)
    }
    pub fn is_overlapping_with(&self, other: &Plane) -> bool {
        if self.head == other.head {
            return true;
        }
        for tile in self.tile_iterator().filter_map(|t| t) {
            for other_tile in other.tile_iterator().filter_map(|t| t) {
                if tile == other_tile {
                    return true;
                }
            }
        }
        false
    }
    pub fn has_tile(&self, needle: &Coordinate) -> bool {
        for coord in self.tile_iterator().filter_map(|t| t) {
            if *needle == coord {
                return true;
            }
        }
        return false;
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.head, self.orientation)
    }
}

struct Board {
    planes: Vec<Plane>,
    killed_planes: Vec<Plane>,
    hits: Vec<Coordinate>,
    misses: Vec<Coordinate>,
}

impl Board {
    fn new() -> Board {
        Board {
            planes: Vec::new(),
            killed_planes: Vec::new(),
            hits: Vec::new(),
            misses: Vec::new(),
        }
    }
    fn add_new_plane_at(&mut self, head: &str, orientation: &str) -> Result<&Plane, String> {
        if self.is_in_gameplay() {
            return Err("Cannot add planes mid-game".to_string());
        }
        let t_plane = Plane::new(head, orientation);
        match t_plane {
            None => Err(format!("plane cannot spawn at {} in direction {}", head, orientation)),
            Some(plane) => {
                match plane.is_outside_of_map() {
                    true => Err("Plane falls off the map".to_string()),
                    false => {
                        for other in &self.planes {
                            if plane.is_overlapping_with(&other) {
                                return Err(format!("Plane would overlap with another one: {}", other));
                            }
                        }
                        self.planes.push(plane);
                        Ok(self.planes.last().unwrap())
                    },
                }
            }
        }
    }
    fn is_in_gameplay(&self) -> bool {
        !self.hits.is_empty() || !self.misses.is_empty()
    }
    fn is_initialized(&self) -> bool {
        self.planes.len() == 3
    }
    fn hit_at(&mut self, tile: &str) -> bool {
        match Coordinate::new(tile) {
            Some(coord) => {
                for i in 0..self.planes.len() {
                    if self.planes[i].has_tile(&coord) {
                        self.hits.push(coord);
                        return true;
                    }
                    if self.planes[i].head == coord {
                        let killed_plane = self.planes.remove(i);
                        self.killed_planes.push(killed_plane);
                        return true;
                    }
                }
                self.misses.push(coord);
                false
            },
            None => false,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ascii_board = format!("
  ABCDEFGHIJ
 1          1 
 2          2 
 3          3 
 4          4 
 5          5 
 6          6 
 7          7 
 8          8 
 9          9 
10          10
  ABCDEFGHIJ
");
        let mut byte_grid = ascii_board.into_bytes();
        let offset_heuristic = |x:usize, y:usize| 16 + y*15 + x;
        for plane in &self.planes {
            let head_c : u8 = match &plane.orientation {
                &Orientation::North => '^' as u8,
                &Orientation::South => 'v' as u8,
                &Orientation::West => '<' as u8,
                &Orientation::East => '>' as u8,
            };
            let head_x = plane.head.0 as usize;
            let head_y = plane.head.1 as usize;
            let head_offset = offset_heuristic(head_x, head_y);
            byte_grid[head_offset] = head_c;

            for tile in plane.tile_iterator().filter_map(|t| t) {
                let tile_x = tile.0 as usize;
                let tile_y = tile.1 as usize;
                let tile_offset = offset_heuristic(tile_x, tile_y);
                byte_grid[tile_offset] = 'o' as u8;
            }
        }
        for hit in &self.hits {
            let hit_x = hit.0 as usize;
            let hit_y = hit.1 as usize;
            let hit_offset = offset_heuristic(hit_x, hit_y);
            byte_grid[hit_offset] = 'x' as u8;
        }
        for miss in &self.misses {
            let miss_x = miss.0 as usize;
            let miss_y = miss.1 as usize;
            let miss_offset = offset_heuristic(miss_x, miss_y);
            byte_grid[miss_offset] = '*' as u8;
        }
        let displayed = String::from_utf8(byte_grid).unwrap();
        write!(f, "{}", displayed)
    }
}

fn main() {
    let mut board = Board::new();
    loop {
        println!("{}", board);
        match board.planes.len() {
            3 => {
                let new_hit = read_line_with_prompt("Bombard coordinate: ");
                if board.hit_at(new_hit.as_str()) {
                    println!("You've hit at {}", new_hit);
                } else {
                    println!("You've missed at {}", new_hit);
                }
            },
            _ => {
                let new_head = read_line_with_prompt("new plane at: ");
                let new_orientation = read_line_with_prompt("orientation: ");
                match board.add_new_plane_at(new_head.as_str(), new_orientation.as_str()) {
                    Ok(_) => {},
                    Err(msg) => {
                        println!("{}", msg);
                        continue
                    }
                };
            },
        }
    }
}

fn read_line_with_prompt(prompt: &str) -> String {
    let mut value = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush();
        let read_count = io::stdin().read_line(&mut value).expect(prompt);
        if read_count > 1 {
            break;
        }
    }
    value.trim().to_string()
}

#[cfg(test)]
mod test {
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
        let mut iter = p.tile_iterator();
        let expected_coordinates = vec!["C6", "D6", "E6", "F6", "G6", "E7", "D8", "E8", "F8"];
        for expected in expected_coordinates {
            assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_all_invisible_north() {
        let p = Plane::new("J10", "N").unwrap();
        let mut iter = p.tile_iterator();
        for _ in 0..9 {
            assert_eq!(Some(None), iter.next());
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_lefthand_invisible_north() {
        let p = Plane::new("A1", "N").unwrap();
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
        let expected_coordinates = vec!["G4", "F4", "E4", "D4", "C4", "E3", "F2", "E2", "D2"];
        for expected in expected_coordinates {
            assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_all_invisible_south() {
        let p = Plane::new("J10", "N").unwrap();
        let mut iter = p.tile_iterator();
        for _ in 0..9 {
            assert_eq!(Some(None), iter.next());
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_lefthand_invisible_south() {
        let p = Plane::new("A1", "N").unwrap();
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
        let expected_coordinates = vec!["D3", "D4", "D5", "D6", "D7", "C5", "B4", "B5", "B6"];
        for expected in expected_coordinates {
            assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_all_invisible_east() {
        let p = Plane::new("A10", "E").unwrap();
        let mut iter = p.tile_iterator();
        for _ in 0..9 {
            assert_eq!(Some(None), iter.next());
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_lefthand_invisible_east() {
        let p = Plane::new("J1", "E").unwrap();
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
        let expected_coordinates = vec!["F7", "F6", "F5", "F4", "F3", "G5", "H6", "H5", "H4"];
        for expected in expected_coordinates {
            assert_eq!(expected, format!("{}", iter.next().unwrap().unwrap()));
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_all_invisible_west() {
        let p = Plane::new("J10", "W").unwrap();
        let mut iter = p.tile_iterator();
        for _ in 0..9 {
            assert_eq!(Some(None), iter.next());
        }
        assert_eq!(None, iter.next());
    }
    #[test]
    fn iterate_tiles_lefthand_invisible_west() {
        let p = Plane::new("A10", "W").unwrap();
        let mut iter = p.tile_iterator();
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
        let mut iter = p.tile_iterator();
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
        ];
        for plane_positions_pair in overlapping_pairs {
            let p1 = Plane::new((plane_positions_pair.0).0, (plane_positions_pair.0).1).unwrap();
            let p2 = Plane::new((plane_positions_pair.1).0, (plane_positions_pair.1).1).unwrap();
            assert_eq!(true, p1.is_overlapping_with(&p2));
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
}
