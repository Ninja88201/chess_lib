use std::ops::Index;

use crate::{Move, Tile};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MoveList {
    moves: [Move; 256],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::default(); 256],
            len: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        debug_assert!(self.len < 256);
        self.moves[self.len] = m;
        self.len += 1;
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.len].iter()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn contains_move(&self, from: Tile, to: Tile) -> bool {
        self.moves.iter().any(|m| m.from() == from && m.to() == to)
    }
    pub fn contains(&self, mov: &Move) -> bool {
        self.moves.contains(mov)
    }
}
impl Index<usize> for MoveList
{
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[index]
    }
}