use crate::{Board, MoveList};

impl Board {
    #[inline(always)]
    pub fn positions(&mut self, depth: usize) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.generate_legal_moves(self.white_turn, &mut moves);
        let mut positions = 0;

        for &m in moves.iter() {
            self.make_move_unchecked(m);
            if depth == 1 {
                positions += 1;
            } else {
                positions += self.positions(depth - 1);
            }
            self.undo_move();
        }

        positions
    }
    #[inline(always)]
    pub fn positions_divide(&mut self, depth: usize) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        // self.generate_moves(self.white_turn, &mut moves);
        self.generate_legal_moves(self.white_turn, &mut moves);
        let mut positions = 0;

        for &m in moves.iter() {
            self.make_move_unchecked(m);
            let count = self.positions(depth - 1);
            positions += count;
            println!("{}, {}", m, count);
            self.undo_move();
        }

        positions
    }
}
