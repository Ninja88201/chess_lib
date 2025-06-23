use crate::Board;

impl Board
{
    #[inline(always)]
    pub fn positions(&mut self, depth: i32) -> i64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.generate_moves(self.white_turn);
        let mut positions = 0;
        for m in moves {
            if self.make_move_unchecked(m).is_ok() {
                if !self.is_in_check(!self.white_turn) {
                    positions += self.positions(depth - 1);
                }
                self.undo_move();
            }
        }
        positions
    }
}