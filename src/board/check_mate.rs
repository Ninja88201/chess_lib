use crate::board::{Board, Piece};

impl Board {

    fn attacks_square(&self, from: u8, piece: Piece, white: bool, target: u8) -> bool {
        self.generate_attacks_from_piece(from, piece, white).get_bit(target)
    }
    pub fn square_attacked(&self, square: u8, by_white: bool) -> bool {
        let (opponent, _) = self.get_players(by_white);
        for s in opponent.pieces() {
            if self.attacks_square(s, opponent.get_piece(s).unwrap(), by_white, square) {
                return true;
            }
        }
        false
    }

    pub fn is_in_check(&self, white: bool) -> bool {
        let (player, _) = self.get_players(white);
        return self.square_attacked(player.get_king_square(), !white);
    }
    pub fn is_checkmate(&mut self, white: bool) -> bool {
        if !self.is_in_check(white) {
            return false;
        }

        let (player, _) = self.get_players(white);

        for from in player.pieces() {
            let possible_moves = self.generate_moves_from(from);
            for m in possible_moves {{
                if self.make_move_unchecked(m) == Ok(()) {
                    let in_check = self.is_in_check(white);
                    self.undo_move();
                    if !in_check { return false; }
                }
            }}
        }
        true
    }
}