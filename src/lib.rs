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

mod board;
pub use self::board::Board;

mod bombardment_result;
pub use self::bombardment_result::BombardmentResult;

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
            if self.board_opponent.planes().len() == 0 {
                self.gameplay = YouWon;
            } else {
                self.gameplay = OpponentBombards;
            }
            println!("gameplay became {} in {} on {}", self.gameplay, file!(), line!());
            return;
        }
        if self.gameplay == OpponentBombards {
            if self.board_you.planes().len() == 0 {
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
                        self.scrapbook_you.hits_mut().push(coord.clone())
                    },
                    Miss => {
                        self.scrapbook_you.misses_mut().push(coord.clone())
                    },
                    Kill => {
                        let maybe_plane = self.board_opponent.find_plane_at(&coord);
                        if let Some(plane) = maybe_plane {
                            println!("revealing plane {} on your scrapbook", plane.id());
                            self.scrapbook_you.killed_planes_mut().push(plane.clone());
                        } else {
                            println!("not revealing plane killed at {}", coord);
                        }
                        self.scrapbook_you.kills_mut().push(coord.clone());
                        self.scrapbook_you.hits_mut().push(coord.clone())
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
        let ref mut scrapbook_opponent = &mut self.scrapbook_opponent;
        if 0 == scrapbook_opponent.empty_indices().len() {
            return (Retry, None)
        }
        let wanted : usize = rand::thread_rng().gen::<usize>() % scrapbook_opponent.empty_indices().len();

        let (tile_num, flag) = {
            let ref mut indices = scrapbook_opponent.empty_indices_mut();
            let tile_num = *indices.iter().nth(wanted).unwrap();
            let flag = indices.remove(&tile_num);
            (tile_num, flag)
        };

        if flag {
            let tile = Coordinate::new_from_usize(tile_num);
            let result = self.board_you.hit_at(tile);
            match result {
                Hit => {
                    scrapbook_opponent.hits_mut().push(tile.clone())
                },
                Miss => {
                    scrapbook_opponent.misses_mut().push(tile.clone())
                },
                Kill => {
                    scrapbook_opponent.kills_mut().push(tile.clone())
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
