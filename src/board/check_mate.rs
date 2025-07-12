use crate::{Board, Colour, GameState, MoveList, Piece, Tile};

impl Board {
    /// Returns whether or not a tile for a given colour is attacked
    /// e.g. tile_attacked(Tile::E1, Colour::White) return whether
    /// the tile E1 is attacked for white / by black
    pub fn tile_attacked(&self, tile: Tile, colour: Colour) -> bool {
        let (_, attacker) = self.get_players(colour);
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

        let pawn_mask = tile.pawn_attacks(colour);
        if (attacker.bb[Piece::Pawn as usize] & pawn_mask).some() {
            return true;
        }

        false
    }

    pub fn is_in_check(&self, colour: Colour) -> bool {
        if colour.white() {
            if let Some(cached) = self.white_cache.get() {
                return cached;
            }
        } else {
            if let Some(cached) = self.black_cache.get() {
                return cached;
            }
        }

        let (player, _) = self.get_players(colour);
        let is_checked =
            self.tile_attacked(player.king_tile(), colour);

        if colour.white() {
            self.white_cache.set(Some(is_checked));
        } else {
            self.black_cache.set(Some(is_checked));
        }

        is_checked
    }

    pub fn is_in_checkmate(&self, colour: Colour) -> bool {
        if !self.is_in_check(colour) {
            return false;
        }

        let mut moves = MoveList::new();
        self.generate_legal_moves(colour, &mut moves);
        moves.is_empty()
    }

    pub fn is_stalemate(&self, colour: Colour) -> bool {
        if self.is_in_check(colour) {
            return false;
        }
        let mut moves = MoveList::new();
        self.generate_legal_moves(colour, &mut moves);
        moves.is_empty()
    }

    /// Returns whether true if no pawn move or capture has been made
    /// in the past 50 moves
    pub fn fifty_move_rule(&self) -> bool {
        self.half_moves >= 100 
    }

    /// Returns whether or not the same position has been reached 3 times
    /// throughout the process of the game
    pub fn three_fold_rep(&self) -> bool {
        let current = self.to_zobrist_hash();
        self.repetition_history.iter()
            .filter(|&&hash| hash == current)
            .take(3)
            .count() == 3
    }

    /// Returns a draw if either opponent does not have the required material
    /// to perform a checkmate e.g. only a knight or same coloured bishops
    pub fn insufficient_material(&self) -> bool
    {
        let white_pieces = self.white.pieces.count_ones();
        let black_pieces = self.black.pieces.count_ones();

        let white_knights = self.white.bb[Piece::Knight as usize].count_ones();
        let white_bishops = self.white.bb[Piece::Bishop as usize].count_ones();

        let black_knights = self.black.bb[Piece::Knight as usize].count_ones();
        let black_bishops = self.black.bb[Piece::Bishop as usize].count_ones();

        match (white_pieces - 1, black_pieces - 1) {
            (1, 1) => true,
            (2, 1) | (1, 2) => {
                if white_knights > 0 || black_knights > 0 || white_bishops > 0 || black_bishops > 0 {
                    true
                } else {
                    false
                }
            }
            (2, 2) => {
                if white_bishops > 0 && black_bishops > 0 {
                    let white_colour = self.white.bb[Piece::Bishop as usize].to_bit().unwrap().is_light_square();
                    let black_colour = self.black.bb[Piece::Bishop as usize].to_bit().unwrap().is_light_square();
                    white_colour == black_colour
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Returns the current state of the board when the function is called
    pub fn get_state(&self) -> GameState
    {
        let white = self.turn;
        if self.is_in_checkmate(white) {
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
