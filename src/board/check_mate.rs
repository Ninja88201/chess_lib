use crate::{Bitboard, Board, Tile};

impl Board {

    pub fn tile_attacked(&self, tile: Tile, by_white: bool) -> bool {
        let (opponent, _) = self.get_players(by_white);
        let mut attacks = Bitboard::EMPTY;
        for from in opponent.pieces() {
            if let Some(piece) = opponent.get_piece(from) {
                attacks |= self.generate_attacks_from_piece(from, piece, by_white);
            }
        }
        return attacks.get_bit(tile);
    }

    pub fn is_in_check(&self, white: bool) -> bool {
        let (player, _) = self.get_players(white);
        return self.tile_attacked(player.king_tile, !white);
    }
    pub fn is_checkmate(&mut self, white: bool) -> bool {
        if !self.is_in_check(white) {
            return false;
        }

        let (player, _) = self.get_players(white);

        for from in player.pieces() {
            let possible_moves = self.generate_moves_from(from);
            for m in possible_moves {{
                if self.make_move_unchecked(m).is_ok() {
                    let in_check = self.is_in_check(white);
                    self.undo_move();
                    if !in_check { return false; }
                }
            }}
        }
        true
    }
}