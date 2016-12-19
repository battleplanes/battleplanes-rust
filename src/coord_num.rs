use std::fmt;

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

