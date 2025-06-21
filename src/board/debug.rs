use crate::board::Board;

impl Board
{
    pub fn positions(&mut self, depth: i32) -> i64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.generate_legal_moves(self.white_turn);
        let mut positions = 0;
        for m in moves {
            if self.make_move_unchecked(m).is_ok() {
                positions += self.positions(depth - 1);
                self.undo_move();
            }
        }
        return positions
    }
}