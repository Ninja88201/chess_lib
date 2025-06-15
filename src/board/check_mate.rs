use crate::board::{Board, Piece};

impl Board {

    fn attacks_square(&self, from: u8, piece: Piece, white: bool, target: u8) -> bool {
        self.generate_moves_from_piece(from, piece, white) & 1 << target != 0
    }
    pub fn square_attacked(&self, square: u8, by_white: bool) -> bool {
        let (opponent, _) = self.get_players(by_white);
        let mut p = opponent.pieces();
        while p != 0 {
            let s = p.trailing_zeros() as u8;
            if self.attacks_square(s, opponent.get_piece(s).unwrap(), by_white, square) {
                return true;
            }
            p &= p - 1;
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

        let mut b = player.pieces();
        while b != 0 {
            let from = b.trailing_zeros() as u8;
            let mut possible_moves = self.generate_moves_from(from);
            while possible_moves != 0 {
                let to = possible_moves.trailing_zeros() as u8;
                if self.try_move_piece(from, to) {
                    let in_check = self.is_in_check(white);
                    self.undo_move();
                    if !in_check { return false;}
                }
                possible_moves &= possible_moves - 1;
            }
            b &= b - 1;
        }
        true
    }
}