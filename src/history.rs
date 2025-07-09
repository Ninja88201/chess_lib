use crate::{Move, SanMove};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct History {
    pub last_move: Move,
    pub last_zobrist: u64,
    pub san_string: SanMove,
}
impl History
{
    pub fn new(mov: Move, hash: u64, san: SanMove) -> Self
    {
        Self { 
            last_move: mov, 
            last_zobrist: hash, 
            san_string: san 
        }
    }
}