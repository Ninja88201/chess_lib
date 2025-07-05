use crate::{Board, GameState, MoveList, Piece, Tile};

impl Board {
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

    pub fn is_in_check(&self, white: bool) -> bool {
        if white {
            if let Some(cached) = self.white_cache.get() {
                return cached;
            }
        } else {
            if let Some(cached) = self.black_cache.get() {
                return cached;
            }
        }

        let (player, _) = self.get_players(white);
        let is_checked =
            self.tile_attacked(player.king_tile(), !white);

        if white {
            self.white_cache.set(Some(is_checked));
        } else {
            self.black_cache.set(Some(is_checked));
        }

        is_checked
    }

    pub fn is_checkmate(&self, white: bool) -> bool {
        if !self.is_in_check(white) {
            return false;
        }

        let mut moves = MoveList::new();
        self.generate_legal_moves(white, &mut moves);
        moves.is_empty()
    }
    pub fn is_stalemate(&self, white: bool) -> bool {
        if self.is_in_check(white) {
            return false;
        }
        let mut moves = MoveList::new();
        self.generate_legal_moves(white, &mut moves);
        moves.is_empty()
    }
    pub fn fifty_move_rule(&self) -> bool {
        self.half_moves >= 100 
    }
    pub fn three_fold_rep(&self) -> bool {
        let current = self.to_zobrist_hash();
        self.repetition_history.iter()
            .filter(|&&hash| hash == current)
            .take(3)
            .count() == 3
    }
    pub fn insufficient_material(&self) -> bool
    {
        let white_pieces = self.white.get_all_attackers();
        let black_pieces = self.black.get_all_attackers();


        match (white_pieces.len(), black_pieces.len()) {
            (0, 0) => true,
            (1, 0) | (0, 1) => {
                matches!(white_pieces.first().or(black_pieces.first()).map(|x| x.0), Some(Piece::Bishop) | Some(Piece::Knight))
            }
            (1, 1) => {
                match (white_pieces[0].0, black_pieces[0].0) {
                    (Piece::Bishop, Piece::Bishop) => {
                        let white_color = white_pieces[0].1.is_light_square();
                        let black_color = black_pieces[0].1.is_light_square();
                        white_color == black_color
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn get_state(&self) -> GameState
    {
        let white = self.white_turn;
        if self.is_checkmate(white) {
            GameState::Checkmate(white)
        } else if self.is_stalemate(white) {
            GameState::Stalemate(white)
        } else if self.fifty_move_rule() {
            GameState::FiftyMoveRule
        } else if self.insufficient_material() {
            GameState::InsufficientMaterial
        } else if self.three_fold_rep() {
            GameState::ThreeRepetition
        } else {
            GameState::Playing
        }
    }
}
