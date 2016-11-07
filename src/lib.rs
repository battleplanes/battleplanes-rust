/// TODOs
/// =====
///
/// * API clean-ups
/// * the concept of a `Player`
/// * AI improvements, levels, aiming heuristics
///   * infer level from planes placed on board (beginner, intermediate, advanced)
///   * set AI level player heuristics (beginner, intermediate, advanced, hack)
/// * users and tournaments
/// * different UIs: web, console, GUI, OpenGl
/// * better testability, e.g. stubbing out random generators
/// * internationalization, translation
extern crate rand;
use std::fmt;
use rand::Rng;
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum CoordLetter {
    A, B, C, D, E, F, G, H, I, J,
}

impl CoordLetter {
    pub fn new(from: &str) -> Option<CoordLetter> {
        use CoordLetter::*;
        match from {
            "A" => Some(A),
            "B" => Some(B),
            "C" => Some(C),
            "D" => Some(D),
            "E" => Some(E),
            "F" => Some(F),
            "G" => Some(G),
            "H" => Some(H),
            "I" => Some(I),
            "J" => Some(J),
            _ => None
        }
    }
    pub fn new_moved_by(&self, offset: i32) -> Option<CoordLetter> {
        let moved_usize: i32 = ((*self as usize) as i32) + offset;
        if moved_usize < 0 {
            return None;
        }
        if moved_usize > 9 {
            return None;
        }
        return Some(CoordLetter::from(moved_usize as usize));
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
        use CoordLetter::*;
        match (self, other) {
            (&A, &A) => true,
            (&B, &B) => true,
            (&C, &C) => true,
            (&D, &D) => true,
            (&E, &E) => true,
            (&F, &F) => true,
            (&G, &G) => true,
            (&H, &H) => true,
            (&I, &I) => true,
            (&J, &J) => true,
            _ => false,
        }
    }
}

impl From<usize> for CoordLetter {
    fn from(u: usize) -> CoordLetter {
        use CoordLetter::*;
        match u % 10 {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            4 => E,
            5 => F,
            6 => G,
            7 => H,
            8 => I,
            _ => J,
        }
    }
}

impl fmt::Display for CoordLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CoordLetter::*;
        write!(f, "{}", match self {
            &A => "A",
            &B => "B",
            &C => "C",
            &D => "D",
            &E => "E",
            &F => "F",
            &G => "G",
            &H => "H",
            &I => "I",
            &J => "J",
        })
    }
}

#[derive(Debug)]
pub enum CoordNum {
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
}

impl CoordNum {
    pub fn new(from: &str) -> Option<CoordNum> {
        use CoordNum::*;
        match from {
            "1" => Some(One),
            "2" => Some(Two),
            "3" => Some(Three),
            "4" => Some(Four),
            "5" => Some(Five),
            "6" => Some(Six),
            "7" => Some(Seven),
            "8" => Some(Eight),
            "9" => Some(Nine),
            "10" => Some(Ten),
            _ => None,
        }
    }
    pub fn new_moved_by(&self, offset: i32) -> Option<CoordNum> {
        let moved_usize: i32 = ((*self as usize) as i32) + offset;
        if moved_usize < 0 {
            return None;
        }
        if moved_usize > 9 {
            return None;
        }
        return Some(CoordNum::from(moved_usize as usize));
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
        use CoordNum::*;
        match(self, other) {
            (&One, &One) => true,
            (&Two, &Two) => true,
            (&Three, &Three) => true,
            (&Four, &Four) => true,
            (&Five, &Five) => true,
            (&Six, &Six) => true,
            (&Seven, &Seven) => true,
            (&Eight, &Eight) => true,
            (&Nine, &Nine) => true,
            (&Ten, &Ten) => true,
            _ => false,
        }
    }
}

impl From<usize> for CoordNum {
    fn from(u: usize) -> CoordNum {
        use CoordNum::*;
        match u % 10 {
            0 => One,
            1 => Two,
            2 => Three,
            3 => Four,
            4 => Five,
            5 => Six,
            6 => Seven,
            7 => Eight,
            8 => Nine,
            _ => Ten,
        }
    }
}
impl fmt::Display for CoordNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CoordNum::*;
        write!(f, "{}", match self {
            &One => "1",
            &Two => "2",
            &Three => "3",
            &Four => "4",
            &Five => "5",
            &Six => "6",
            &Seven => "7",
            &Eight => "8",
            &Nine => "9",
            &Ten => "10",
        })
    }
}

#[derive(Debug)]
#[derive(Copy)]
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
    pub fn new_random_coordinate() -> Coordinate {
        let mut rng = rand::thread_rng();
        let rand_x: usize= rng.gen_range(0, 10);
        let rand_y: usize = rng.gen_range(0, 10);
        Coordinate ( CoordLetter::from(rand_x), CoordNum::from(rand_y) )
    }
    pub fn new_from_usize(num: usize) -> Coordinate {
        let x : usize = num % 10;
        let y : usize = num / 10;
        Coordinate( CoordLetter::from(x), CoordNum::from(y) )
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
    fn as_usize(&self) -> usize {
        (self.1 as usize) * 10 + (self.0 as usize)
    }
}

/// This is implemented because a hit or a miss is put both on the player's
/// board, as well as on the opponent's scrapbook.
///
/// TODO
/// ====
/// * use a typed arena instead inside Game, see Game::you_hit_at();
///   https://docs.rs/releases/search?query=typed+arena
impl Clone for Coordinate {
    fn clone(&self) -> Coordinate { *self }
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

#[derive(Clone)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn new(from: &str) -> Option<Orientation> {
        use Orientation::*;
        match from {
            "N" => Some(North),
            "E" => Some(East),
            "S" => Some(South),
            "W" => Some(West),
            _ => None,
        }
    }
}

impl From<usize> for Orientation {
    fn from(u: usize) -> Orientation {
        use Orientation::*;
        match u % 4 {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => North,
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Orientation::*;
        write!(f, "{}", match self {
            &North => "N",
            &South => "S",
            &East => "E",
            &West => "W",
        })
    }
}

#[derive(Clone)]
pub struct PlanePositionIterator<'a> {
    head: &'a Coordinate,
    orientation: &'a Orientation,
    current_tile: usize,
}

impl<'a> Iterator for PlanePositionIterator<'a> {
    type Item = Option<Coordinate>;
    fn next(&mut self) -> Option<Option<Coordinate>> {
        use Orientation::*;
        if self.current_tile > 8 {
            return None;
        }
        let tiles = match self.orientation {
            &North => vec![(-2,  1), (-1,  1), ( 0,  1), ( 1,  1), ( 2,  1), ( 0,  2), (-1,  3), ( 0,  3), ( 1,  3)],
            &South => vec![( 2, -1), ( 1, -1), ( 0, -1), (-1, -1), (-2, -1), ( 0, -2), ( 1, -3), ( 0, -3), (-1, -3)],
            &East =>  vec![(-1, -2), (-1, -1), (-1,  0), (-1,  1), (-1,  2), (-2,  0), (-3, -1), (-3,  0), (-3,  1)],
            &West =>  vec![( 1,  2), ( 1,  1), ( 1,  0), ( 1, -1), ( 1, -2), ( 2,  0), ( 3,  1), ( 3,  0), ( 3, -1)],
        };
        let move_x = tiles[self.current_tile].0;
        let move_y = tiles[self.current_tile].1;
        self.current_tile += 1;
        let t = self.head.new_moved_by(move_x, move_y);
        Some(t)
    }
}
struct TileIterator<'a> {
    coordinate_iterator: PlanePositionIterator<'a>,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            match self.coordinate_iterator.next() {
                Some(maybe_coord) => {
                    match maybe_coord {
                        Some(coord) => {
                            return Some(coord.as_usize());
                        },
                        None => continue,
                    }
                },
                None => return None,
            }
        };
    }
}

#[derive(Clone)]
pub struct Plane {
    head: Coordinate,
    orientation: Orientation,
    id: usize,
}

impl Plane {
    pub fn new(from: &str, orientation: &str) -> Option<Plane> {
        Plane::new_with_id(from, orientation, 0)
    }
    fn new_with_id(from: &str, orientation: &str, id: usize) -> Option<Plane> {
        match (Coordinate::new(from), Orientation::new(orientation)) {
            (Some(c), Some(o)) => Some(Plane {
                head: c,
                orientation: o,
                id: id,
            }),
            _ => None,
        }
    }
    pub fn coordinate_iterator(&self) -> PlanePositionIterator {
        PlanePositionIterator {
            head: &self.head,
            orientation: &self.orientation,
            current_tile: 0,
        }
    }
    fn tile_iterator(&self) -> TileIterator {
        TileIterator {
            coordinate_iterator: self.coordinate_iterator(),
        }
    }
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }
    pub fn head(&self) -> &Coordinate {
        &self.head
    }
    pub fn is_outside_of_map(&self) -> bool {
        self.coordinate_iterator().find(|x: &Option<Coordinate>| *x == None) == Some(None)
    }
    pub fn is_overlapping_with(&self, other: &Plane) -> bool {
        if self.head == other.head {
            return true;
        }
        for tile in self.coordinate_iterator().filter_map(|t| t) {
            if tile == other.head {
                return true;
            }
            for other_tile in other.coordinate_iterator().filter_map(|t| t) {
                if tile == other_tile {
                    return true;
                }
                if self.head == other_tile {
                    return true;
                }
            }
        }
        false
    }
    pub fn has_tile(&self, needle: &Coordinate) -> bool {
        for coord in self.coordinate_iterator().filter_map(|t| t) {
            if *needle == coord {
                return true;
            }
        }
        return false;
    }
    pub fn id(&self) -> usize {
        self.id
    }
}

pub enum BombardmentResult {
    Hit,
    Miss,
    Kill,
    Retry,
}

impl Into<bool> for BombardmentResult {
    fn into(self) -> bool {
        use BombardmentResult::*;
        match self {
            Retry => false,
            Miss => false,
            Hit => true,
            Kill => true,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    planes: Vec<Plane>,
    killed_planes: Vec<Plane>,
    hits: Vec<Coordinate>,
    misses: Vec<Coordinate>,
    kills: Vec<Coordinate>,
    empty_indices: BTreeSet<usize>,
    previous_error_message: Option<String>,
}

impl Board {
    pub fn new() -> Board {
        let mut empty_indices: BTreeSet<usize> = BTreeSet::new();
        for i in 0..100 {
            empty_indices.insert(i);
        }
        Board {
            planes: Vec::new(),
            killed_planes: Vec::new(),
            hits: Vec::new(),
            misses: Vec::new(),
            kills: Vec::new(),
            empty_indices: empty_indices,
            previous_error_message: None,
        }
    }
    pub fn get_previous_hit_message(&self) -> String {
        match self.previous_error_message {
            Some(ref msg) => msg.clone(),
            None => "".to_string(),
        }
    }
    pub fn new_random() -> Board {
        let mut temp_board = Board::new();

        let mut rng = rand::thread_rng();
        let mut random_orientations = [0 as usize, 1, 2, 3];
        //TODO: better heuristics, AI levels
        loop {
            for _ in 0..100 {
                let head_candidate = Coordinate::new_random_coordinate();
                let raw_head_candidate = format!("{}", head_candidate);

                rng.shuffle(&mut random_orientations);
                for j in random_orientations.iter() {
                    let orientation_cadidate = Orientation::from(*j as usize);
                    let raw_orientation_candidate = format!("{}", orientation_cadidate);
                    match temp_board.add_new_plane_at(&raw_head_candidate, &raw_orientation_candidate) {
                        Ok(_) => {
                            break;
                        },
                        Err(_msg) => {
                            continue;
                        }
                    }
                }
                if temp_board.planes.len() == 3 {
                    break;
                }
            }
            if temp_board.planes.len() == 3 {
                break;
            } else {
                temp_board.clear_planes();
            }
        }
        temp_board
    }

    pub fn add_new_plane_at(&mut self, head: &str, orientation: &str) -> Result<&Plane, String> {
        let mut error_message = "";
        if self.is_in_gameplay() {
            let t = "Cannot add planes mid-game".to_string();
            self.previous_error_message = Some(t.clone());
            return Err(t);
        }
        let t_plane = Plane::new_with_id(head, orientation, self.planes.len()+1);
        match t_plane {
            None => {
                let t = format!("plane cannot spawn at {} in direction {}", head, orientation);
                self.previous_error_message = Some(t.clone());
                Err(t)
            },
            Some(plane) => {
                match plane.is_outside_of_map() {
                    true => {
                        let t = "Plane falls off the map".to_string();
                        self.previous_error_message = Some(t.clone());
                        Err(t)
                    },
                    false => {
                        // TODO: return list of overlapping other planes in error message
                        for other in &self.planes {
                            if plane.is_overlapping_with(&other) {
                                let t = format!("Plane would overlap with another one: {}", other.id);
                                self.previous_error_message = Some(t.clone());
                                return Err(t);
                            }
                        }
                        let head_offset = plane.head.as_usize();
                        self.empty_indices.remove(&head_offset);
                        for tile in plane.tile_iterator() {
                            self.empty_indices.remove(&tile);
                        }
                        self.planes.push(plane);
                        self.previous_error_message = None;
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
        self.planes.len() + self.killed_planes.len() == 3
    }
    pub fn hit_at(&mut self, coord: Coordinate) -> BombardmentResult {
        self.empty_indices.remove(&coord.as_usize());
        for i in 0..self.planes.len() {
            if self.planes[i].has_tile(&coord) {
                self.hits.push(coord);
                return BombardmentResult::Hit;
            }
            if self.planes[i].head == coord {
                let killed_plane = self.planes.remove(i);
                self.killed_planes.push(killed_plane);
                return BombardmentResult::Kill;
            }
        }
        self.misses.push(coord);
        BombardmentResult::Miss
    }
    pub fn planes(&self) -> &Vec<Plane> {
        &self.planes
    }
    pub fn killed_planes(&self) -> &Vec<Plane> {
        &self.killed_planes
    }
    pub fn hits(&self) -> &Vec<Coordinate> {
        &self.hits
    }
    pub fn misses(&self) -> &Vec<Coordinate> {
        &self.misses
    }
    pub fn kills(&self) -> &Vec<Coordinate> {
        &self.kills
    }
    pub fn clear_planes(&mut self) {
        self.planes = Vec::new();
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for plane in self.planes() {
            s.push_str(format!("{}{} ", plane.head(), plane.orientation()).as_str());
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum GamePlay {
    YouPlaceNewPlane,
    OpponentPlacesNewPlane,
    YouBombard,
    OpponentBombards,
    YouWon,
    OpponentWon,
}

impl GamePlay {
    fn new_random_state() -> GamePlay {
        use GamePlay::*;
        let mut rng = rand::thread_rng();
        match rng.gen() {
            true => YouPlaceNewPlane,
            false => OpponentPlacesNewPlane,
        }
    }
}
impl PartialEq for GamePlay {
    fn eq(&self, other: &GamePlay) -> bool {
        use GamePlay::*;
        match (self, other) {
            (&YouPlaceNewPlane, &YouPlaceNewPlane) => true,
            (&OpponentPlacesNewPlane, &OpponentPlacesNewPlane) => true,
            (&YouBombard, &YouBombard) => true,
            (&OpponentBombards, &OpponentBombards) => true,
            (&YouWon, &YouWon) => true,
            (&OpponentWon, &OpponentWon) => true,
            _ => false,
        }
    }
}

impl fmt::Display for GamePlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GamePlay::*;
        write!(f, "{}", match self {
            &YouPlaceNewPlane => "YouPlaceNewPlane",
            &OpponentPlacesNewPlane => "OpponentPlacesNewPlane",
            &YouBombard => "YouBombard",
            &OpponentBombards => "OpponentBombards",
            &YouWon => "YouWon",
            &OpponentWon => "OpponentWon",
        })
    }
}

#[derive(Clone)]
pub struct Game {
    pub gameplay: GamePlay,
    pub board_you: Board,
    pub board_opponent: Board,
    pub scrapbook_you: Board,
    pub scrapbook_opponent: Board,
}

impl Game {
    pub fn new_random_starter() -> Game {
        Game {
            gameplay: GamePlay::new_random_state(),
            board_you: Board::new(),
            board_opponent: Board::new(),
            scrapbook_you: Board::new(),
            scrapbook_opponent: Board::new(),
        }
    }

    pub fn next_logical_state(&mut self) {
        use GamePlay::*;
        println!("gameplay is {}", self.gameplay);
        if self.gameplay == YouPlaceNewPlane {
            if !self.board_opponent.is_initialized() {
                self.gameplay = OpponentPlacesNewPlane;
            } else {
                self.gameplay = OpponentBombards;
            }
            println!("gameplay became {} in {} on {}", self.gameplay, file!(), line!());
            return;
        }
        if self.gameplay == OpponentPlacesNewPlane {
            if !self.board_you.is_initialized() {
                self.gameplay = YouPlaceNewPlane;
            } else {
                self.gameplay = YouBombard;
            }
            println!("gameplay became {} in {} on {}", self.gameplay, file!(), line!());
            return;
        }
        if self.gameplay == YouBombard {
            if self.board_opponent.planes.len() == 0 {
                self.gameplay = YouWon;
            } else {
                self.gameplay = OpponentBombards;
            }
            println!("gameplay became {} in {} on {}", self.gameplay, file!(), line!());
            return;
        }
        if self.gameplay == OpponentBombards {
            if self.board_you.planes.len() == 0 {
                self.gameplay = OpponentWon;
            } else {
                self.gameplay = YouBombard;
            }
            println!("gameplay became {} in {} on {}", self.gameplay, file!(), line!());
            return;
        }
        println!("gameplay stayed {} in {} on {}", self.gameplay, file!(), line!());
    }
    pub fn you_hit_at(&mut self, target: &str) -> BombardmentResult {
        use BombardmentResult::*;
        match Coordinate::new(target) {
            None => {
                Retry
            },
            Some(coord) => {
                let result = self.board_opponent.hit_at(coord);
                match result {
                    Hit => {
                        self.scrapbook_you.hits.push(coord.clone())
                    },
                    Miss => {
                        self.scrapbook_you.misses.push(coord.clone())
                    },
                    Kill => {
                        self.scrapbook_you.kills.push(coord.clone());
                        self.scrapbook_you.hits.push(coord.clone())
                    },
                    Retry => {
                    },
                };
                result
            },
        }
    }
    pub fn opponent_hits_randomly(&mut self) -> (BombardmentResult, Option<Coordinate>) {
        use BombardmentResult::*;
        if 0 == self.scrapbook_opponent.empty_indices.len() {
            return (Retry, None)
        }
        let wanted : usize = rand::thread_rng().gen::<usize>() % self.scrapbook_opponent.empty_indices.len();
        let ref mut indices = self.scrapbook_opponent.empty_indices;
        let tile_num = *indices.iter().nth(wanted).unwrap();
        if indices.remove(&tile_num) {
            let tile = Coordinate::new_from_usize(tile_num);
            let result = self.board_you.hit_at(tile);
            match result {
                Hit => {
                    self.scrapbook_opponent.hits.push(tile.clone())
                },
                Miss => {
                    self.scrapbook_opponent.misses.push(tile.clone())
                },
                Kill => {
                    self.scrapbook_opponent.kills.push(tile.clone())
                },
                Retry => {
                },
            };
            return (result, Some(tile))
        }
        (Retry, None)
    }
}
//TODO: implement me
/*
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &North => "N",
            &South => "S",
            &East => "E",
            &West => "W",
        })
    }
}
*/

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
}
