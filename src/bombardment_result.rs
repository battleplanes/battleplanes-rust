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


