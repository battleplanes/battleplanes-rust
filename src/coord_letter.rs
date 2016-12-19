use std::fmt;

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

