use crate::{Board, MoveList, Piece, Tile};

impl Board {
    #[inline]
    pub fn tile_attacked(&self, tile: Tile, by_white: bool) -> bool {
        let (attacker, _) = self.get_players(by_white);
        let occ = self.occupied();
        let straight_mask = tile.rook_attacks(occ);
        if (attacker.bb[Piece::Rook as usize] & straight_mask).some()
            || (attacker.bb[Piece::Queen as usize] & straight_mask).some()
        {
            return true;
        }

        let diag_mask = tile.bishop_attacks(occ);
        if (attacker.bb[Piece::Bishop as usize] & diag_mask).some()
            || (attacker.bb[Piece::Queen as usize] & diag_mask).some()
        {
            return true;
        }

        let knight_mask = tile.knight_attacks();
        if (attacker.bb[Piece::Knight as usize] & knight_mask).some() {
            return true;
        }

        let pawn_mask = tile.pawn_attacks(!by_white);
        if (attacker.bb[Piece::Pawn as usize] & pawn_mask).some() {
            return true;
        }

        false
    }

    pub fn is_in_check(&mut self, white: bool) -> bool {
        if white {
            if let Some(cached) = self.white_cache {
                return cached;
            }
        } else {
            if let Some(cached) = self.black_cache {
                return cached;
            }
        }

        let (player, _) = self.get_players(white);
        let is_checked =
            self.tile_attacked(player.bb[Piece::King as usize].to_bit().unwrap(), !white);

        if white {
            self.white_cache = Some(is_checked);
        } else {
            self.black_cache = Some(is_checked);
        }

        is_checked
    }

    pub fn is_checkmate(&mut self, white: bool) -> bool {
        if !self.is_in_check(white) {
            return false;
        }

        let mut moves = MoveList::new();
        self.generate_legal_moves(white, &mut moves);
        moves.is_empty()
    }
    pub fn is_stalemate(&mut self, white: bool) -> bool {
        if self.is_in_check(white) {
            return false;
        }
        let mut moves = MoveList::new();
        self.generate_legal_moves(white, &mut moves);
        moves.is_empty()
    }
}
