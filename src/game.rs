use rand;
use rand::Rng;

use game_play::GamePlay;
use board::Board;
use coordinate::Coordinate;
use bombardment_result::BombardmentResult;

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
