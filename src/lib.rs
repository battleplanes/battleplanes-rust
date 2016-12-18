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

mod coord_letter;
pub use self::coord_letter::CoordLetter;

mod coord_num;
pub use self::coord_num::CoordNum;

mod coordinate;
pub use self::coordinate::Coordinate;

mod plane;
pub use self::plane::Plane;

mod orientation;
pub use self::orientation::Orientation;

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

    pub fn add_new_plane_at(&mut self, head: &str, orientation: &str) -> Result<usize, String> {
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
                        let t = "Plane would fall off the map, try again.".to_string();
                        self.previous_error_message = Some(t.clone());
                        Err(t)
                    },
                    false => {
                        // TODO: return list of overlapping other planes in error message
                        for other in &self.planes {
                            if plane.is_overlapping_with(&other) {
                                let t = format!("Plane would overlap with another one: {}, try again.", other.id());
                                self.previous_error_message = Some(t.clone());
                                return Err(t);
                            }
                        }
                        let head_offset = plane.head().as_usize();
                        self.empty_indices.remove(&head_offset);
                        for tile in plane.tile_iterator() {
                            self.empty_indices.remove(&tile);
                        }
                        self.planes.push(plane);
                        self.previous_error_message = None;
                        Ok(self.planes.last().unwrap().id())
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
            if self.planes[i].head() == &coord {
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
    pub fn find_plane_at(&self, at: &Coordinate) -> Option<&Plane> {
        for plane in &self.planes {
            if plane.has_tile(at) {
                return Some(plane);
            }
            if plane.head() == at {
                return Some(plane);
            }
        }
        for plane in &self.killed_planes {
            if plane.has_tile(at) {
                return Some(plane);
            }
            if plane.head() == at {
                return Some(plane);
            }
        }
        None
    }
    pub fn get_plane_by_id(&self, id: usize) -> Option<&Plane> {
        for plane in &self.planes {
            if plane.id() == id {
                return Some(&plane);
            }
        }
        None
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
    pub reveal_killed: bool,
}

impl Game {
    pub fn new_random_starter(reveal_killed: bool) -> Game {
        Game {
            gameplay: GamePlay::new_random_state(),
            board_you: Board::new(),
            board_opponent: Board::new(),
            scrapbook_you: Board::new(),
            scrapbook_opponent: Board::new(),
            reveal_killed: reveal_killed,
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
                        let maybe_plane = self.board_opponent.find_plane_at(&coord);
                        if let Some(plane) = maybe_plane {
                            println!("revealing plane {} on your scrapbook", plane.id());
                            self.scrapbook_you.killed_planes.push(plane.clone());
                        } else {
                            println!("not revealing plane killed at {}", coord);
                        }
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
mod unittests;
