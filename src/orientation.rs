use std::fmt;

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

