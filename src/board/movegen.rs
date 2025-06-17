use crate::board::{Board, Piece, Move};

impl Board {
    pub fn generate_legal_moves(&self, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let (player, _) = self.get_players(white);
        for s in player.pieces() {
            for m in self.generate_legal_moves_from(s) {
                moves.push(m);
            }
        }
        moves
    }
    pub fn generate_legal_moves_from(&self, from: u8) -> Vec<Move> {
        let piece = self.get_piece_at_square(from);
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
    pub fn generate_moves_from(&self, square: u8) -> Vec<Move> {
        let (piece, white) = match self.get_piece_at_square(square) {
            Some(x) => x,
            None => return Vec::new(),
        };
        return self.generate_moves_from_piece(square, piece, white);
    }
    pub fn generate_moves_from_piece(&self, square: u8, piece: Piece, white: bool) -> Vec<Move> {
        match piece {
            Piece::Pawn => self.generate_pawn_moves(square, white),
            Piece::Knight => self.generate_knight_moves(square, white),
            Piece::Bishop => self.generate_sliding_moves(square, white, false, true),
            Piece::Rook => self.generate_sliding_moves(square, white, true, false),
            Piece::Queen => self.generate_sliding_moves(square, white, true, true),
            Piece::King => self.generate_king_moves(square, white),
        }
    }
    fn generate_pawn_moves(&self, square: u8, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        // let (_, _) = self.get_players(white);
        let direction: i8 = match white {
            true => 8,
            false => -8,
        };
        let promotion: u8 = match white {
            true => 7,
            false => 0,
        };

        let target = square as i8 + direction;
        if target >= 0 && target < 64 && !self.occupied().get_bit(target as u8) {
            let new_rank = (target / 8) as u8;

            if new_rank == promotion {
                moves.push(Move::new(
                    self.white_turn,
                    square,
                    target as u8,
                    Piece::Pawn,
                    None,
                    self.en_passant,
                    self.white.castling,
                    self.black.castling,
                    Some(Piece::Queen)
                ));
                for p in 1..5 {
                    moves.push(Move::new(
                        self.white_turn,
                        square,
                        target as u8,
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
                        square,
                        target as u8,
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
        let start_rank = match white {
            true => 1,
            false => 6,
        };
        if square / 8 == start_rank {
            let double_target = square as i8 + 2 * direction;
            if (0..64).contains(&double_target)
                && !self.occupied().get_bit(target as u8) && !self.occupied().get_bit(double_target as u8)
            {
                moves.push(
                Move::new(
                    self.white_turn,
                    square,
                    double_target as u8,
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
        for side in [-1, 1] {
            let cap_square = square as i8 + direction + side;
            let file = square % 8;
            if cap_square >= 0
                && cap_square < 64
                && (file != 0 || side != -1)
                && (file != 7 || side != 1)
            {
                if let Some(ep) = self.en_passant {
                    if cap_square as u8 == ep {
                        moves.push(Move::new(
                                self.white_turn,
                                square,
                                cap_square as u8,
                                Piece::Pawn,
                                Some(Piece::Pawn),
                                self.en_passant,
                                self.white.castling,
                                self.black.castling,
                                None
                            ));
                    }
                }
                if self.is_square_occupied_by_enemy(cap_square as u8, white) {
                    let new_rank = (cap_square / 8) as u8;

                    if new_rank == promotion {
                        for p in 1..5 {
                            moves.push(Move::new(
                                self.white_turn,
                                square,
                                cap_square as u8,
                                Piece::Pawn,
                                Some(self.get_piece_at_square(cap_square as u8).unwrap().0),
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
                                square,
                                cap_square as u8,
                                Piece::Pawn,
                                Some(self.get_piece_at_square(cap_square as u8).unwrap().0),
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
    fn generate_knight_moves(&self, square: u8, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let rank = square / 8;
        let file = square % 8;

        for (dx, dy) in &Board::KNIGHT_OFFSETS {
            let new_file = file as i8 + dx;
            let new_rank = rank as i8 + dy;

            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                let dest = (new_rank * 8 + new_file) as u8;
                if !self.is_square_occupied_by_friendly(dest, white) {
                    moves.push(
                        Move::new(
                            self.white_turn,
                            square,
                            dest,
                            Piece::Knight,
                            match self.get_piece_at_square(dest) {
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
        }

        moves
    }
    fn generate_sliding_moves(
        &self,
        square: u8,
        white: bool,
        straight: bool,
        diagonal: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        const STRAIGHT_DELTAS: &[i8] = &[8, -8, 1, -1];      // up, down, right, left
        const DIAGONAL_DELTAS: &[i8] = &[9, -9, 7, -7];      // diag up-right, down-left, up-left, down-right

        if straight {
            for &delta in STRAIGHT_DELTAS {
                moves.extend(self.slide_in_direction_moves(square, white, delta));
            }
        }

        if diagonal {
            for &delta in DIAGONAL_DELTAS {
                moves.extend(self.slide_in_direction_moves(square, white, delta));
            }
        }

        moves
    }

    // Helper function to slide in a given direction
    fn slide_in_direction_moves(&self, square: u8, white: bool, delta: i8) -> Vec<Move> {
        let mut result = Vec::new();
        let mut current = square as i8;
        let mut prev_file = current % 8;

        loop {
            let next = current + delta;

            if next < 0 || next >= 64 {
                break;
            }

            let next_file = next % 8;

            // Prevent wrapping around left/right edges
            if (delta == 1 || delta == -1 || delta == 9 || delta == -9 || delta == 7 || delta == -7)
                && (next_file - prev_file).abs() != 1
            {
                break;
            }

            let dest = next as u8;

            if self.is_square_occupied_by_friendly(dest, white) {
                break;
            }

            result.push(Move::new(
                self.white_turn,
                square,
                dest,
                self.get_piece_at_square(square).unwrap().0,
                self.get_piece_at_square(dest).map(|(p, _)| p),
                self.en_passant,
                self.white.castling,
                self.black.castling,
                None,
            ));

            if self.is_square_occupied_by_enemy(dest, white) {
                break;
            }

            current = next;
            prev_file = next_file;
        }

        result
    }
    fn generate_king_moves(&self, square: u8, white: bool) -> Vec<Move> {
        let mut moves = Vec::new();

        let (player, _) = self.get_players(white);

        for d in Board::KING_OFFSETS {
            let dest = square as i8 + d;
            
            // Ensure the king moves only one square, not more
            let dest_file = dest % 8;
            let from_file = square % 8;

            if (0..64).contains(&dest)
                && (from_file as i8 - dest_file as i8).abs() <= 1
                && !self.is_square_occupied_by_friendly(dest as u8, white)
                && !self.square_attacked(dest as u8, !white)
            {
                // moves.set_bit(dest as u8, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        square,
                        dest as u8,
                        Piece::King,
                        match self.get_piece_at_square(dest as u8) {
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

        // Castling
        if white && square == Board::E1 {
            // Short (g1)
            if player.castling.short_castle()
                && self.get_piece_at_square(Board::E1 + 1).is_none()
                && self.get_piece_at_square(Board::E1 + 2).is_none()
                && !self.square_attacked(Board::E1 + 1, false)
                && !self.square_attacked(Board::E1 + 2, false)
            {
                // moves.set_bit(Board::E1 + 2, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        square,
                        Board::E1 + 2,
                        Piece::King,
                        match self.get_piece_at_square(Board::E1 + 2) {
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
                && self.get_piece_at_square(Board::E1 - 1).is_none()
                && self.get_piece_at_square(Board::E1 - 2).is_none()
                && self.get_piece_at_square(Board::E1 - 3).is_none()
                && !self.square_attacked(Board::E1 - 1, false)
                && !self.square_attacked(Board::E1 - 2, false)
            {
                // moves.set_bit(Board::E1 - 2, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        square,
                        Board::E1 - 2,
                        Piece::King,
                        match self.get_piece_at_square(Board::E1 - 2) {
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

        if !white && square == Board::E8 {
            // Short (g8)
            if player.castling.short_castle()
                && self.get_piece_at_square(Board::E8 + 1).is_none()
                && self.get_piece_at_square(Board::E8 + 2).is_none()
                && !self.square_attacked(Board::E8 + 1, true)
                && !self.square_attacked(Board::E8 + 2, true)
            {
                // moves.set_bit(Board::E8 + 2, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        square,
                        Board::E8 + 2,
                        Piece::King,
                        match self.get_piece_at_square(Board::E8 + 2) {
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
                && self.get_piece_at_square(Board::E8 - 1).is_none()
                && self.get_piece_at_square(Board::E8 - 2).is_none()
                && self.get_piece_at_square(Board::E8 - 3).is_none()
                && !self.square_attacked(Board::E8 - 1, true)
                && !self.square_attacked(Board::E8 - 2, true)
            {
                // moves.set_bit(Board::E8 - 2, true);
                moves.push(
                    Move::new(
                        self.white_turn,
                        square,
                        Board::E8 - 2,
                        Piece::King,
                        match self.get_piece_at_square(Board::E8 - 2) {
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
    fn is_square_occupied_by_enemy(&self, square: u8, white: bool) -> bool {
        let (_, opponent) = self.get_players(white);
        opponent.pieces().get_bit(square)
    }

    fn is_square_occupied_by_friendly(&self, square: u8, white: bool) -> bool {
        let (player, _) = self.get_players(white);
        player.pieces().get_bit(square)
    }
}