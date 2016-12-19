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

mod game;
pub use self::game::Game;

mod game_play;
pub use self::game_play::GamePlay;

#[cfg(test)]
mod unittests;
