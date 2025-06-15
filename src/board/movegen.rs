use crate::{bitboard::Bitboard, board::{Board, Piece}};

impl Board {
    pub fn generate_legal_moves(&self, white: bool) -> Vec<(u8, Bitboard)> {
        let mut moves = Vec::new();
        let (player, _) = self.get_players(white);
        for s in player.pieces() {
            moves.push((s, self.generate_legal_moves_from(s)));
        }
        moves
    }
    pub fn generate_legal_moves_from(&self, from: u8) -> Bitboard {
        let piece = self.get_piece_at_square(from);
        if let Some((_, white)) = piece {
            let mut legal_moves = Bitboard::EMPTY;
            for to in self.generate_moves_from(from) {
                let mut copy = self.clone();

                copy.make_move_unchecked(from, to);

                if !copy.is_in_check(white) {
                    legal_moves.set_bit(to, true);
                }
            }
            legal_moves
        } else {
            Bitboard::EMPTY
        }
    }

    pub fn generate_moves(&self, white: bool) -> Bitboard {
        let (player, _) = self.get_players(white);
        let mut moves = Bitboard::EMPTY;
        for s in player.pieces() {
            moves |= self.generate_moves_from(s);
        }
        moves
    }
    pub fn generate_moves_from(&self, square: u8) -> Bitboard {
        let (piece, white) = match self.get_piece_at_square(square) {
            Some(x) => x,
            None => return Bitboard::EMPTY,
        };
        return self.generate_moves_from_piece(square, piece, white);
    }
    pub fn generate_moves_from_piece(&self, square: u8, piece: Piece, white: bool) -> Bitboard {
        match piece {
            Piece::Pawn => self.generate_pawn_moves(square, white),
            Piece::Knight => self.generate_knight_moves(square, white),
            Piece::Bishop => self.generate_sliding_moves(square, white, false, true),
            Piece::Rook => self.generate_sliding_moves(square, white, true, false),
            Piece::Queen => self.generate_sliding_moves(square, white, true, true),
            Piece::King => self.generate_king_moves(square, white),
        }
    }
    fn generate_pawn_moves(&self, square: u8, white: bool) -> Bitboard {
        let mut moves = Bitboard::EMPTY;
        let direction: i8 = match white {
            true => 8,
            false => -8,
        };

        let target = square as i8 + direction;
        if target >= 0 && target < 64 && !self.occupied().get_bit(target as u8) {
            moves.set_bit(target as u8, true);
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
                moves.set_bit(double_target as u8, true);
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
                if self.is_square_occupied_by_enemy(cap_square as u8, white) {
                    moves.set_bit(cap_square as u8, true);
                }
            }
        }

        moves
    }
    fn generate_knight_moves(&self, square: u8, white: bool) -> Bitboard {
        let mut moves = Bitboard::EMPTY;
        let rank = square / 8;
        let file = square % 8;

        // List of all potential knight moves (dx, dy)

        for (dx, dy) in &Board::KNIGHT_OFFSETS {
            let new_file = file as i8 + dx;
            let new_rank = rank as i8 + dy;

            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                let dest = (new_rank * 8 + new_file) as u8;
                if !self.is_square_occupied_by_friendly(dest, white) {
                    moves.set_bit(dest, true);
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
    ) -> Bitboard {
        let mut moves = Bitboard::EMPTY;
        let start_file = square % 8;

        const STRAIGHT_DIRECTIONS: &[(i8, i8)] = &[
            (8, 0),   // up
            (-8, 0),  // down
            (1, 1),   // right
            (-1, -1), // left
        ];

        const DIAGONAL_DIRECTIONS: &[(i8, i8)] = &[
            (9, 1),   // up-right
            (-9, -1), // down-left
            (7, -1),  // up-left
            (-7, 1),  // down-right
        ];

        if straight {
            for &(delta, file_change) in STRAIGHT_DIRECTIONS {
                moves |= self.slide_in_direction_moves(square, white, delta, file_change, start_file);
            }
        }

        if diagonal {
            for &(delta, file_change) in DIAGONAL_DIRECTIONS {
                moves |= self.slide_in_direction_moves(square, white, delta, file_change, start_file);
            }
        }

        moves
    }

    // Helper function to slide in a given direction
    fn slide_in_direction_moves(
        &self,
        square: u8,
        white: bool,
        delta: i8,
        file_change: i8,
        start_file: u8,
    ) -> Bitboard {
        let mut result = Bitboard::EMPTY;
        let mut sq = square as i8;
        let mut current_file = start_file as i8;

        loop {
            sq += delta;
            current_file += file_change;

            if sq < 0 || sq >= 64 || current_file < 0 || current_file >= 8 {
                break;
            }

            let dest = sq as u8;

            if self.is_square_occupied_by_friendly(dest, white) {
                break;
            }

            result.set_bit(dest, true);

            if self.is_square_occupied_by_enemy(dest, white) {
                break;
            }
        }

        result
    }
    fn generate_king_moves(&self, square: u8, white: bool) -> Bitboard {
        let mut moves = Bitboard::EMPTY;

        let player = if white { &self.white } else { &self.black };

        for d in Board::KING_OFFSETS {
            let dest = square as i8 + d;
            
            // Ensure the king moves only one square, not more
            let dest_file = dest % 8;
            let from_file = square % 8;

            if (0..64).contains(&dest)
                // Ensure the king doesn't jump to a non-adjacent file
                && (from_file as i8 - dest_file as i8).abs() <= 1
                && !self.is_square_occupied_by_friendly(dest as u8, white)
                && !self.square_attacked(dest as u8, !white)
            {
                moves.set_bit(dest as u8, true);
            }
        }

        // Castling
        if white && square == Board::E1 {
            // Short (g1)
            if player.short_castle
                && self.get_piece_at_square(Board::E1 + 1).is_none()
                && self.get_piece_at_square(Board::E1 + 2).is_none()
                && !self.square_attacked(Board::E1 + 1, false)
                && !self.square_attacked(Board::E1 + 2, false)
            {
                moves.set_bit(Board::E1 + 2, true);
            }
            // Long (c1)
            if player.long_castle
                && self.get_piece_at_square(Board::E1 - 1).is_none()
                && self.get_piece_at_square(Board::E1 - 2).is_none()
                && self.get_piece_at_square(Board::E1 - 3).is_none()
                && !self.square_attacked(Board::E1 - 1, false)
                && !self.square_attacked(Board::E1 - 2, false)
            {
                moves.set_bit(Board::E1 - 2, true);
            }
        }

        if !white && square == Board::E8 {
            // Short (g8)
            if player.short_castle
                && self.get_piece_at_square(Board::E8 + 1).is_none()
                && self.get_piece_at_square(Board::E8 + 2).is_none()
                && !self.square_attacked(Board::E8 + 1, true)
                && !self.square_attacked(Board::E8 + 2, true)
            {
                moves.set_bit(Board::E8 + 2, true);

            }
            // Long (c8)
            if player.long_castle
                && self.get_piece_at_square(Board::E8 - 1).is_none()
                && self.get_piece_at_square(Board::E8 - 2).is_none()
                && self.get_piece_at_square(Board::E8 - 3).is_none()
                && !self.square_attacked(Board::E8 - 1, true)
                && !self.square_attacked(Board::E8 - 2, true)
            {
                moves.set_bit(Board::E8 - 2, true);

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