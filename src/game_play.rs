use std::fmt;

use rand;
use rand::Rng;

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
    pub fn new_random_state() -> GamePlay {
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

