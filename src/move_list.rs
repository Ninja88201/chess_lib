use rand::Rng;

use crate::Move;

#[derive(Clone)]
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
    pub fn choose_random(&self) -> Option<Move> {
        if self.is_empty() {
            None
        } else {
            let mut rng = rand::rng();
            Some(self.moves[rng.random_range(0..self.len())])
        }
    }
}