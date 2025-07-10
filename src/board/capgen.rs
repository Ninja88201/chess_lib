use crate::{Bitboard, Board, Colour, MoveList, Piece, Tile};

impl Board
{
    pub fn generate_legal_captures(&self, colour: Colour, moves: &mut MoveList) {
        let (player, _) = self.get_players(colour);
        let checkers = self.get_checkers(colour);
        let checkers_count = checkers.count_ones();
        let king_tile = player.king_tile();

        self.generate_king_capture_moves(king_tile, colour, moves);

        // Double check → only king can move
        if checkers_count > 1 {
            return;
        }

        // Determine valid capture targets (checker or interposing square)
        let targets = if checkers_count == 1 {
            let checker_tile = checkers.to_bit().expect("Expected exactly one checker");
            let checker_piece = self.get_piece_at_tile(checker_tile).unwrap();

            let between = if matches!(checker_piece.0, Piece::Bishop | Piece::Rook | Piece::Queen) {
                checker_tile.get_between(king_tile)
            } else {
                Bitboard::EMPTY
            };

            checker_tile.to_mask() | between
        } else {
            Bitboard::ALL
        };

        let pinned = self.get_pinned_pieces(colour);

        // Pawns (capture directions only)
        for pawn_tile in player.bb[Piece::Pawn as usize] {
            let pin_mask = pinned.get(&pawn_tile).copied().unwrap_or(Bitboard::ALL);
            let move_mask = pin_mask & targets;
            self.generate_pawn_capture_moves(pawn_tile, colour, Some(move_mask), moves);
        }

        // Knights
        for knight_tile in player.bb[Piece::Knight as usize] {
            if pinned.contains_key(&knight_tile) {
                continue;
            }
            self.generate_knight_capture_moves(knight_tile, colour, Some(targets), moves);
        }

        // Bishops
        for bishop_tile in player.bb[Piece::Bishop as usize] {
            let pin_mask = pinned.get(&bishop_tile).copied().unwrap_or(Bitboard::ALL);
            let move_mask = pin_mask & targets;
            self.generate_sliding_capture_moves(bishop_tile, colour, false, true, Some(move_mask), moves);
        }

        // Rooks
        for rook_tile in player.bb[Piece::Rook as usize] {
            let pin_mask = pinned.get(&rook_tile).copied().unwrap_or(Bitboard::ALL);
            let move_mask = pin_mask & targets;
            self.generate_sliding_capture_moves(rook_tile, colour, true, false, Some(move_mask), moves);
        }

        // Queens
        for queen_tile in player.bb[Piece::Queen as usize] {
            let pin_mask = pinned.get(&queen_tile).copied().unwrap_or(Bitboard::ALL);
            let move_mask = pin_mask & targets;
            self.generate_sliding_capture_moves(queen_tile, colour, true, true, Some(move_mask), moves);
        }
    }
    pub fn generate_king_capture_moves(&self, tile: Tile, colour: Colour, moves: &mut MoveList) {
        let mut attacks = tile.king_attacks();
        let (friendly_mask, opponent_mask) = if colour.white() {
            (self.white.pieces, self.black.pieces)
        } else {
            (self.black.pieces, self.white.pieces)
        };

        let attack_mask = self.generate_king_danger(colour);

        attacks &= !friendly_mask & !attack_mask & opponent_mask;

        for to in attacks {
            moves.push(self.create_move(
                tile,
                to,
                Piece::King,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }
    }  
    pub fn generate_pawn_capture_moves(&self, tile: Tile, colour: Colour, mask: Option<Bitboard>, moves: &mut MoveList) {
        for maybe_target in [
            tile.left(colour).and_then(|t| t.forward(colour)),
            tile.right(colour).and_then(|t| t.forward(colour)),
        ] {
            let to = match maybe_target {
                Some(t) => t,
                None => continue,
            };

            // --- En Passant Capture ---
            if Some(to) == self.en_passant && mask.map_or(true, |mask| mask.get_bit(to.backward(colour).unwrap())) {
                let king_tile = if colour.white() {
                    self.white.bb[Piece::King as usize].to_bit().unwrap()
                } else {
                    self.black.bb[Piece::King as usize].to_bit().unwrap()
                };
                let occupied = self.occupied();
                let (_, opponent) = self.get_players(colour);
                let enemy_sliders = opponent.bb[Piece::Rook as usize] | opponent.bb[Piece::Queen as usize];

                if !Self::is_illegal_en_passant_discovery(tile, to, king_tile, occupied, enemy_sliders) {
                    moves.push(self.create_move(tile, to, Piece::Pawn, Some(Piece::Pawn), None));
                }
                continue;
            }

            // --- Normal Capture ---
            if self.is_square_occupied_by_enemy(to, colour) && mask.map_or(true, |mask| mask.get_bit(to)) {
                let captured = self.get_piece_at_tile(to).map(|(p, _)| p);
                self.try_push_pawn_move(tile, to, colour, captured, moves);
            }
        }
    }
    pub fn generate_knight_capture_moves(&self, tile: Tile, colour: Colour, mask: Option<Bitboard>, moves: &mut MoveList) {
        let mut attacks = tile.knight_attacks();
        if let Some(m) = mask {
            attacks &= m;
        }
        let (friendly_mask, opponent_mask) = if colour.white() {
            (self.white.pieces, self.black.pieces)
        } else {
            (self.black.pieces, self.white.pieces)
        };

        attacks &= !friendly_mask & opponent_mask;
        for to in attacks {
            moves.push(self.create_move(
                tile,
                to,
                Piece::Knight,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }
    }
    pub fn generate_sliding_capture_moves(&self, tile: Tile, colour: Colour, straight: bool, diagonal: bool, mask: Option<Bitboard>, moves: &mut MoveList) {
        let mut attacks = self.generate_sliding_attacks(tile, straight, diagonal, None);
        if let Some(m) = mask {
            attacks &= m;
        }
        let (friendly_mask, opponent_mask) = if colour.white() {
            (self.white.pieces, self.black.pieces)
        } else {
            (self.black.pieces, self.white.pieces)
        };
        attacks &= !friendly_mask & opponent_mask;
        let piece = self.get_piece_at_tile(tile).unwrap().0;
        for to in attacks {
            moves.push(self.create_move(
                tile,
                to,
                piece,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }
    }
}