
use crate::{Board, MoveList};

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

        for &m in moves.iter() {
            if self.make_move_unchecked(m).is_ok() {
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
    #[inline(always)]
    pub fn positions_divide(&mut self, depth: i32) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.generate_moves(self.white_turn, &mut moves);
        let mut positions = 0;

        for &m in moves.iter() {
            if self.make_move_unchecked(m).is_ok() {
                if !self.is_in_check(!self.white_turn) && !self.is_checkmate(!self.white_turn) {
                    let count = self.positions(depth - 1);
                    positions += count;
                    println!("{}, {}", m, count)
                }
                self.undo_move();
            }
        }

        positions
    }
    
    // pub fn positions_memo(&mut self, depth: i32, memo: &mut HashMap<String, i64>) -> i64 {
    //     if depth == 0 {
    //         return 1;
    //     }

    //     // Hash the board state and depth to use as a cache key
    //     let board_key = self.get_board_hash();
    //     if let Some(&cached_positions) = memo.get(&(board_key.clone(), depth)) {
    //         return cached_positions;
    //     }

    //     let mut moves = MoveList::new();
    //     self.generate_moves(self.white_turn, &mut moves);
    //     let mut positions = 0;

    //     for &m in moves.iter() {
    //         if self.make_move_unchecked(m).is_ok() {
    //             if !self.is_in_check(!self.white_turn) {
    //                 if depth == 1 {
    //                     positions += 1;
    //                 } else {
    //                     positions += self.positions_memo(depth - 1, memo);
    //                 }
    //             }
    //             self.undo_move();
    //         }
    //     }

    //     memo.insert((board_key.clone(), depth), positions);
    //     positions
    // }
}