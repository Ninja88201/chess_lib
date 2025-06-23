use crate::{Board, MoveList};
use rayon::prelude::*;


impl Board
{
    #[inline(always)]
    pub fn positions(&mut self, depth: i32) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.generate_moves(self.white_turn, &mut moves);

        let mut positions = 0;
        for m in moves.iter() {
            if self.make_move_unchecked(*m).is_ok() {
                if !self.is_in_check(!self.white_turn) {
                    if depth == 1 {
                        positions += 1;
                    } else {
                        positions += self.positions(depth - 1);
                    }
                }
                self.undo_move();
            }
        }

        positions
    }

    pub fn positions_parallel(&mut self, depth: i32) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.generate_moves(self.white_turn, &mut moves);

        let board = self.clone(); // save current board state

        moves
            .iter()
            .par_bridge() // rayon: parallel iterator
            .map(|m| {
                let mut board = board.clone();
                if board.make_move_unchecked(*m).is_ok() {
                    if !board.is_in_check(!board.white_turn) {
                        return board.positions(depth - 1);
                    }
                }
                0
            })
            .sum()
    }

}