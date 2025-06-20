use crate::{board::{Board, Move, Piece}, tile::Tile};

impl Board {
    pub fn generate_legal_moves(&self, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let (player, _) = self.get_players(white);
        for t in player.pieces() {
            for m in self.generate_legal_moves_from(t) {
                moves.push(m);
            }
        }
        moves
    }
    pub fn generate_legal_moves_from(&self, from: Tile) -> Vec<Move> {
        let piece = self.get_piece_at_tile(from);
        if let Some((_, white)) = piece {
            let mut legal_moves = Vec::new();
            for m in self.generate_moves_from(from) {
                let mut copy = self.clone();

                copy.make_move_unchecked(m);

                if !copy.is_in_check(white) {
                    legal_moves.push(m);
                        
                }
            }
            legal_moves
        } else {
            Vec::new()
        }
    }

    pub fn generate_moves(&self, white: bool) -> Vec<Move> {
        let (player, _) = self.get_players(white);
        let mut moves = Vec::new();
        for s in player.pieces() {
            for m in self.generate_moves_from(s) {
                moves.push(m);
            }
        }
        moves
    }
    pub fn generate_moves_from(&self, square: Tile) -> Vec<Move> {
        let (piece, white) = match self.get_piece_at_tile(square) {
            Some(x) => x,
            None => return Vec::new(),
        };
        return self.generate_moves_from_piece(square, piece, white);
    }
    pub fn generate_moves_from_piece(&self, square: Tile, piece: Piece, white: bool) -> Vec<Move> {
        match piece {
            Piece::Pawn => self.generate_pawn_moves(square, white),
            Piece::Knight => self.generate_knight_moves(square, white),
            Piece::Bishop => self.generate_sliding_moves(square, white, false, true),
            Piece::Rook => self.generate_sliding_moves(square, white, true, false),
            Piece::Queen => self.generate_sliding_moves(square, white, true, true),
            Piece::King => self.generate_king_moves(square, white),
        }
    }
    fn generate_pawn_moves(&self, tile: Tile, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();

        let target = tile.forward(white).unwrap();
        if !self.occupied().get_bit(target) {
            if target.is_promotion(white) {
                for p in 1..5 {
                    moves.push(Move::new(
                        self.white_turn,
                        tile,
                        target,
                        Piece::Pawn,
                        None,
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        Some(Piece::from_index(p))
                    ));
                }
            }
            else {
                moves.push(
                    Move::new(
                        self.white_turn,
                        tile,
                        target,
                        Piece::Pawn,
                        None,
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        None
                    )
                );
            }
            
        }

        // Double push
        if tile.is_pawn_start(white) {
            let double_target = tile.forward(white).and_then(|t| t.forward(white)).unwrap();
            if !self.occupied().get_bit(target as Tile) && !self.occupied().get_bit(double_target as Tile)
            {
                moves.push(
                Move::new(
                    self.white_turn,
                    tile,
                    double_target,
                    Piece::Pawn,
                    None,
                    self.en_passant,
                    self.white.castling,
                    self.black.castling,
                    None
                )
            );
            }
        }

        // Captures
        for caps in [tile.forward(white).and_then(|t| t.left(white)), tile.forward(white).and_then(|t| t.right(white))] {
            if let Some(t) = caps {
                if let Some(ep) = self.en_passant {
                    if t == ep {
                        moves.push(Move::new(
                                self.white_turn,
                                tile,
                                t,
                                Piece::Pawn,
                                Some(Piece::Pawn),
                                self.en_passant,
                                self.white.castling,
                                self.black.castling,
                                None
                            ));
                    }
                }
                if self.is_square_occupied_by_enemy(t, white) {
                    if t.is_promotion(white) {
                        for p in 1..5 {
                            moves.push(Move::new(
                                self.white_turn,
                                tile,
                                t,
                                Piece::Pawn,
                                Some(self.get_piece_at_tile(t).unwrap().0),
                                self.en_passant,
                                self.white.castling,
                                self.black.castling,
                                Some(Piece::from_index(p))
                            ));
                        }
                    }
                    else {
                        moves.push(
                            Move::new(
                                self.white_turn,
                                tile,
                                t,
                                Piece::Pawn,
                                Some(self.get_piece_at_tile(t).unwrap().0),
                                self.en_passant,
                                self.white.castling,
                                self.black.castling,
                                None
                            )
                        );
                    }
                }
            }

            }
        moves
    }
    fn generate_knight_moves(&self, tile: Tile, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let (x, y) = tile.get_coords();
        for (dx, dy) in Board::KNIGHT_OFFSETS {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;
            if nx < 0 || ny < 0 { continue; }
            let dest = Tile::new_xy(nx as u8, ny as u8);
            if let Some(t) = dest {
                if !self.is_square_occupied_by_friendly(t, white) {
                    moves.push(
                        Move::new(
                            self.white_turn,
                            tile,
                            t,
                            Piece::Knight,
                            self.get_piece_at_tile(t).map(|(p, _)| p),
                            self.en_passant,
                            self.white.castling,
                            self.black.castling,
                            None
                        )
                    );
                }
            }
        }

        moves
    }
    fn generate_sliding_moves(
        &self,
        tile: Tile,
        white: bool,
        straight: bool,
        diagonal: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        const STRAIGHT_DELTAS: &[i8] = &[8, -8, 1, -1];      // up, down, right, left
        const DIAGONAL_DELTAS: &[i8] = &[9, -9, 7, -7];      // diag up-right, down-left, up-left, down-right

        if straight {
            for &delta in STRAIGHT_DELTAS {
                moves.extend(self.slide_in_direction_moves(tile, white, delta));
            }
        }

        if diagonal {
            for &delta in DIAGONAL_DELTAS {
                moves.extend(self.slide_in_direction_moves(tile, white, delta));
            }
        }

        moves
    }

    // Helper function to slide in a given direction
    fn slide_in_direction_moves(&self, tile: Tile, white: bool, delta: i8) -> Vec<Move> {
        let mut result = Vec::new();
        let mut current = tile;

        loop {
            let next = match delta {
                8 => current.forward(true),
                -8 => current.backward(true),
                1 => current.right(true),
                -1 => current.left(true),

                9 => current.forward(true).and_then(|t| t.right(true)),
                -9 => current.backward(true).and_then(|t| t.left(true)),
                7 => current.forward(true).and_then(|t| t.left(true)),
                -7 => current.backward(true).and_then(|t| t.right(true)),
                _ => panic!("Not a valid delta")
            };
            if let Some(t) = next {
                if self.is_square_occupied_by_friendly(t, white) {
                    break;
                }
    
                result.push(Move::new(
                    self.white_turn,
                    tile,
                    t,
                    self.get_piece_at_tile(tile).unwrap().0,
                    self.get_piece_at_tile(t).map(|(p, _)| p),
                    self.en_passant,
                    self.white.castling,
                    self.black.castling,
                    None,
                ));
    
                if self.is_square_occupied_by_enemy(t, white) {
                    break;
                }
                current = t;
            }
            else {
                break;
            }


        }

        result
    }
    fn generate_king_moves(&self, tile: Tile, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();

        let (player, _) = self.get_players(white);

        for t in tile.get_neighbours(){
            if !t.in_board() { continue; }
            if !self.is_square_occupied_by_friendly(t, white)
                && !self.tile_attacked(t, !white)
            {
                // moves.set_bit(dest as Tile, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        tile,
                        t,
                        Piece::King,
                        self.get_piece_at_tile(t).map(|(p, _)| p),
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        None
                    )
                );
            }
        }

        // Castling
        if white && tile == Board::E1 {
            // Short (g1)
            if player.castling.short_castle()
                && self.get_piece_at_tile(Board::F1).is_none()
                && self.get_piece_at_tile(Board::G1).is_none()
                && !self.tile_attacked(Board::F1, false)
                && !self.tile_attacked(Board::G1, false)
            {
                // moves.set_bit(Board::E1 + 2, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        tile,
                        Board::G1,
                        Piece::King,
                        match self.get_piece_at_tile(Board::G1) {
                            Some((p, _)) => Some(p),
                            None => None,
                        },
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        None
                    )
                );
            }
            // Long (c1)
            if player.castling.long_castle()
                && self.get_piece_at_tile(Board::D1).is_none()
                && self.get_piece_at_tile(Board::C1).is_none()
                && self.get_piece_at_tile(Board::B1).is_none()
                && !self.tile_attacked(Board::D1, false)
                && !self.tile_attacked(Board::C1, false)
            {
                // moves.set_bit(Board::C1, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        tile,
                        Board::C1,
                        Piece::King,
                        match self.get_piece_at_tile(Board::C1) {
                            Some((p, _)) => Some(p),
                            None => None,
                        },
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        None
                    )
                );
            }
        }

        if !white && tile == Board::E8 {
            // Short (g8)
            if player.castling.short_castle()
                && self.get_piece_at_tile(Board::F8).is_none()
                && self.get_piece_at_tile(Board::G8).is_none()
                && !self.tile_attacked(Board::F8, true)
                && !self.tile_attacked(Board::G8, true)
            {
                // moves.set_bit(Board::G8, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        tile,
                        Board::G8,
                        Piece::King,
                        match self.get_piece_at_tile(Board::G8) {
                            Some((p, _)) => Some(p),
                            None => None,
                        },
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        None
                    )
                );

            }
            // Long (c8)
            if player.castling.long_castle()
                && self.get_piece_at_tile(Board::D8).is_none()
                && self.get_piece_at_tile(Board::C8).is_none()
                && self.get_piece_at_tile(Board::B8).is_none()
                && !self.tile_attacked(Board::D8, true)
                && !self.tile_attacked(Board::C8, true)
            {
                // moves.set_bit(Board::C8, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        tile,
                        Board::C8,
                        Piece::King,
                        match self.get_piece_at_tile(Board::C8) {
                            Some((p, _)) => Some(p),
                            None => None,
                        },
                        self.en_passant,
                        self.white.castling,
                        self.black.castling,
                        None
                    )
                );
            }
        }

        moves
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