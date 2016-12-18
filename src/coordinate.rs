extern crate rand;

use std::fmt;

use rand::Rng;

use coord_letter::CoordLetter;
use coord_num::CoordNum;

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
    pub fn as_usize(&self) -> usize {
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

#[cfg(test)]
mod unittests {
    use super::*;
    #[test]
    fn read_coordinate() {
        assert_eq!(Coordinate(CoordLetter::A, CoordNum::Five), Coordinate::new("A5").unwrap());
        assert_eq!(Coordinate(CoordLetter::B, CoordNum::Seven), Coordinate::new("B7").unwrap());
    }
    #[test]
    fn move_coordinate_within_reach_of_plane() {
        assert_eq!(Coordinate(CoordLetter::F, CoordNum::Six), Coordinate::new("E5").unwrap().new_moved_by(1, 1).unwrap());
        assert_eq!(Coordinate(CoordLetter::D, CoordNum::Four), Coordinate::new("E5").unwrap().new_moved_by(-1, -1).unwrap());
    }
}
