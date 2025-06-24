use crate::{Bitboard, Board, MoveList, Tile};

impl Board {

    pub fn tile_attacked(&self, tile: Tile, by_white: bool) -> bool {
        let (opponent, _) = self.get_players(by_white);
        for from in opponent.pieces() {
            if let Some(piece) = opponent.get_piece(from) {
                let attacks = self.generate_attacks_from_piece(from, piece, by_white);
                if attacks.get_bit(tile) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_in_check(&mut self, white: bool) -> bool {
        if let Some(c) = self.check_cached {
            if self.white_turn == white {
                return c
            } else {
                return false;
            }
        }
        let (player, opponent) = if white {
            (&self.white, &self.black)
        } else {
            (&self.black, &self.white)
        };
        let mut attacks = Bitboard::EMPTY;
        for from in opponent.attackers() {
            if let Some(piece) = opponent.get_piece(from) {
                attacks |= self.generate_attacks_from_piece(from, piece, !white)
            }
        }
        let check = attacks.get_bit(player.king_tile);
        self.check_cached = Some(check);
        return check;
    }
    pub fn is_checkmate(&mut self, white: bool) -> bool {
        if !self.is_in_check(white) {
            return false;
        }
        let mut moves = MoveList::new();
        self.generate_legal_moves(white, &mut moves);
        if moves.len() <= 0 {
            return true;
        }
        else {
            return false;
        }
        

        // let (player, _) = self.get_players(white);

        // for from in player.pieces() {
        //     let mut moves = MoveList::new();
        //     self.generate_moves_from(from, &mut moves);
        //     for m in moves.iter() {
        //         if self.make_move_unchecked(*m).is_ok() {
        //             let in_check = self.is_in_check(white);
        //             self.undo_move();
        //             if !in_check {
        //                 return false;
        //             }
        //         }
        //     }
        // }

        // true
    }
}