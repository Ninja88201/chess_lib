use crate::{lookup_tables, Board, MoveList, Piece, Tile, CastlingRights};

impl Board {
    pub fn generate_legal_moves(&mut self, white: bool, moves: &mut MoveList) {
        let (player, _) = self.get_players(white);
        for t in player.pieces() {
            self.generate_legal_moves_from(t, moves);
        }
    }
    pub fn generate_legal_moves_from(&mut self, from: Tile, moves: &mut MoveList) {
        if let Some((_, white)) = self.get_piece_at_tile(from) {
            let mut pseudo_moves = MoveList::new();
                self.generate_moves_from(from, &mut pseudo_moves);
                for m in pseudo_moves.iter() {
                    if self.make_move_unchecked(*m).is_ok() {
                        if !self.is_in_check(white) {
                            moves.push(*m);
                        }
                        self.undo_move();
                    }
                }
        }
    }

    pub fn generate_moves(&self, white: bool, moves: &mut MoveList) {
        let (player, _) = self.get_players(white);
        for s in player.pieces() {
            self.generate_moves_from(s, moves);
        }
    }

    pub fn generate_moves_from(&self, tile: Tile, moves: &mut MoveList) {
        if let Some((piece, white)) = self.get_piece_at_tile(tile) {
            self.generate_moves_from_piece(tile, piece, white, moves);
        }
    }
    pub fn generate_moves_from_piece(&self, tile: Tile, piece: Piece, white: bool, moves: &mut MoveList) {
        match piece {
            Piece::Pawn => self.generate_pawn_moves(tile, white, moves),
            Piece::Knight => self.generate_knight_moves(tile, white, moves),
            Piece::Bishop => self.generate_sliding_moves(tile, white, false, true, moves),
            Piece::Rook => self.generate_sliding_moves(tile, white, true, false, moves),
            Piece::Queen => self.generate_sliding_moves(tile, white, true, true, moves),
            Piece::King => self.generate_king_moves(tile, white, moves),
        }
    }

    fn generate_pawn_moves(&self, tile: Tile, white: bool, moves: &mut MoveList) {
        // Single push
        if let Some(one_step) = tile.forward(white) {
            if !self.occupied().get_bit(one_step) {
                self.try_push_pawn_move(tile, one_step, white, None, moves);

                // Double push
                if tile.is_pawn_start(white) {
                    if let Some(two_step) = one_step.forward(white) {
                        if !self.occupied().get_bit(two_step) {
                            moves.push(self.create_move(tile, two_step, Piece::Pawn, None, None));
                        }
                    }
                }
            }
        }

        // Diagonal captures and en passant
        for maybe_target in [
            tile.forward(white).and_then(|t| t.left(white)),
            tile.forward(white).and_then(|t| t.right(white)),
        ] {
            if let Some(to) = maybe_target {
                if Some(to) == self.en_passant {
                    moves.push(self.create_move(tile, to, Piece::Pawn, Some(Piece::Pawn), None));
                    continue;
                }

                if self.is_square_occupied_by_enemy(to, white) {
                    let captured = self.get_piece_at_tile(to).map(|(p, _)| p);
                    self.try_push_pawn_move(tile, to, white, captured, moves);
                }
            }
        }
    }

    fn try_push_pawn_move(&self, from: Tile, to: Tile, white: bool, capture: Option<Piece>, moves: &mut MoveList) {
        if to.is_promotion(white) {
            for p in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
                moves.push(self.create_move(from, to, Piece::Pawn, capture, Some(p)));
            }
        } else {
            moves.push(self.create_move(from, to, Piece::Pawn, capture, None));
        }
    }
    fn generate_knight_moves(&self, tile: Tile, white: bool, moves: &mut MoveList) {
        let mask = lookup_tables::knight_attacks_on(Into::<usize>::into(tile));
        for to in mask {
            if !self.is_square_occupied_by_friendly(to, white) {
                moves.push(self.create_move(
                    tile,
                    to,
                    Piece::Knight,
                    self.get_piece_at_tile(to).map(|(p, _)| p),
                    None,
                ));
            }
        }
    }
    fn generate_sliding_moves(&self, tile: Tile, white: bool, straight: bool, diagonal: bool, moves: &mut MoveList) {
        let attacks = self.generate_sliding_attacks(tile, straight, diagonal);

        for to in attacks {
            if self.is_square_occupied_by_friendly(to, white) {
                continue;
            }
            moves.push(self.create_move(
                tile,
                to,
                self.get_piece_at_tile(tile).unwrap().0,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }
    }

    fn generate_king_moves(&self, tile: Tile, white: bool, moves: &mut MoveList) {
        // let (player, _) = self.get_players(white);

        let mask = lookup_tables::king_attacks_on(Into::<usize>::into(tile));
        for t in mask {
            if !self.is_square_occupied_by_friendly(t, white) && !self.tile_attacked(t, !white) {
                moves.push(self.create_move(
                    tile,
                    t,
                    Piece::King,
                    self.get_piece_at_tile(t).map(|(p, _)| p),
                    None,
                ));
            }
        }

        // Castling
        // Short (g1)
        if self.tile_attacked(tile, !white) {
            return;
        }
        if self.castling.contains(CastlingRights::WHITE_KINGSIDE) && self.white.king_tile == tile
            && self.get_piece_at_tile(Board::F1).is_none()
            && self.get_piece_at_tile(Board::G1).is_none()
            && !self.tile_attacked(Board::F1, false)
            && !self.tile_attacked(Board::G1, false)
        {
            moves.push(
                self.create_move(tile, 
                Board::G1, 
                Piece::King, 
                None,
                None)
            );
        }
        // Long (c1)
        if self.castling.contains(CastlingRights::WHITE_QUEENSIDE) && self.white.king_tile == tile
            && self.get_piece_at_tile(Board::D1).is_none()
            && self.get_piece_at_tile(Board::C1).is_none()
            && self.get_piece_at_tile(Board::B1).is_none()
            && !self.tile_attacked(Board::D1, false)
            && !self.tile_attacked(Board::C1, false)
        {
            moves.push(
                self.create_move(tile, 
                Board::C1, 
                Piece::King, 
                None,
                None)
            );
        }

        // Short (g8)
        if self.castling.contains(CastlingRights::BLACK_KINGSIDE) && self.black.king_tile == tile
            && self.get_piece_at_tile(Board::F8).is_none()
            && self.get_piece_at_tile(Board::G8).is_none()
            && !self.tile_attacked(Board::F8, true)
            && !self.tile_attacked(Board::G8, true)
        {
            moves.push(
                self.create_move(tile, 
                Board::G8, 
                Piece::King, 
                None,
                None)
            );

        }
        // Long (c8)
        if self.castling.contains(CastlingRights::BLACK_QUEENSIDE) && self.black.king_tile == tile
            && self.get_piece_at_tile(Board::D8).is_none()
            && self.get_piece_at_tile(Board::C8).is_none()
            && self.get_piece_at_tile(Board::B8).is_none()
            && !self.tile_attacked(Board::D8, true)
            && !self.tile_attacked(Board::C8, true)
        {
            moves.push(
                self.create_move(tile, 
                Board::C8, 
                Piece::King, 
                None,
                None)
            );
        }
    }
    fn is_square_occupied_by_enemy(&self, square: Tile, white: bool) -> bool {
        let (_, opponent) = self.get_players(white);
        opponent.pieces().get_bit(square)
    }

    fn is_square_occupied_by_friendly(&self, square: Tile, white: bool) -> bool {
        let (player, _) = self.get_players(white);
        player.pieces().get_bit(square)
    }
}
