use crate::{bitboard::Bitboard, board::Board};

impl Board
{
    pub fn get_neighbours(&self, square: u8) -> Bitboard
    {
        let mut n = Bitboard::EMPTY;
        let deltas = [1, -1, 8, -8, 9, 7, -7, -9];
        for d in deltas {
            n.set_bit((square as i8 + d) as u8, true);
        }
        n
    } 
}