use crate::{bitboard::Bitboard, board::{Board, Piece}};

impl Board {

    pub fn generate_attacks(&self, white: bool) -> Bitboard {
        let (player, _) = self.get_players(white);
        let mut attacks = Bitboard::EMPTY;
        for s in player.pieces() {
            attacks |= self.generate_attacks_from(s);
        }
        attacks
    }
    pub fn generate_attacks_from(&self, square: u8) -> Bitboard {
        let (piece, white) = match self.get_piece_at_square(square) {
            Some(x) => x,
            None => return Bitboard::EMPTY,
        };
        return self.generate_attacks_from_piece(square, piece, white);
    }
    pub fn generate_attacks_from_piece(&self, square: u8, piece: Piece, white: bool) -> Bitboard {
        match piece {
            Piece::Pawn => self.generate_pawn_attacks(square, white),
            Piece::Knight => self.generate_knight_attacks(square),
            Piece::Bishop => self.generate_sliding_attacks(square, false, true),
            Piece::Rook => self.generate_sliding_attacks(square, true, false),
            Piece::Queen => self.generate_sliding_attacks(square, true, true),
            Piece::King => self.generate_king_attacks(square),
        }
    }
    fn generate_pawn_attacks(&self, square: u8, white: bool) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let direction: i8 = match white {
            true => 8,
            false => -8,
        };

        for side in [-1, 1] {
            let cap_square = square as i8 + direction + side;
            let file = square % 8;
            if cap_square >= 0
                && cap_square < 64
                && (file != 0 || side != -1)
                && (file != 7 || side != 1)
            {
                attacks.set_bit(cap_square as u8, true);
            }
        }

        attacks
    }
    fn generate_knight_attacks(&self, square: u8) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let rank = square / 8;
        let file = square % 8;

        for (dx, dy) in &Board::KNIGHT_OFFSETS {
            let new_file = file as i8 + dx;
            let new_rank = rank as i8 + dy;

            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                let dest = (new_rank * 8 + new_file) as u8;
                attacks.set_bit(dest, true);
            }
        }

        attacks
    }
    fn generate_sliding_attacks(
        &self,
        square: u8,
        straight: bool,
        diagonal: bool,
    ) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
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
                attacks |= self.slide_in_direction_attack(square, delta, file_change, start_file);
            }
        }

        if diagonal {
            for &(delta, file_change) in DIAGONAL_DIRECTIONS {
                attacks |= self.slide_in_direction_attack(square, delta, file_change, start_file);
            }
        }

        attacks
    }
    fn slide_in_direction_attack(
        &self,
        square: u8,
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

            result.set_bit(dest, true);

            if self.occupied().get_bit(dest) {
                break;
            }
        }

        result
    }

    fn generate_king_attacks(&self, square: u8) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        for d in Board::KING_OFFSETS {
            let dest = square as i8 + d;
            if (0..64).contains(&dest) {
                attacks.set_bit(dest as u8, true);
            }
        }

        attacks
    }
}