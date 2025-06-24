use crate::{Bitboard, Board, MoveList, Tile};

impl Board {

    pub fn tile_attacked(&self, tile: Tile, by_white: bool) -> bool {
        let (opponent, _) = self.get_players(by_white);
        opponent.pieces()
            .find(|&from| {
                opponent.get_piece(from)
                    .map_or(false, |piece| self.generate_attacks_from_piece(from, piece, by_white).get_bit(tile))
            })
            .is_some()
    }

    pub fn is_in_check(&mut self, white: bool) -> bool {
        match white {
            true => {
                if let Some(c) = self.white_cache {
                    return c;
                }
            },
            false => {
                if let Some(c) = self.black_cache {
                    return c;
                }
            },
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
        match white {
            true => self.white_cache = Some(check),
            false => self.black_cache = Some(check),
        }
        return check;
    }
    pub fn is_checkmate(&mut self, white: bool) -> bool {
        if !self.is_in_check(white) {
            return false;
        }
        let mut moves = MoveList::new();
        self.generate_legal_moves(white, &mut moves);
        moves.is_empty()
        

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