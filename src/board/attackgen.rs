use crate::{bitboard::Bitboard, board::{Board, Piece}, tile::Tile};

impl Board {

    pub fn generate_attacks(&self, white: bool) -> Bitboard {
        let (player, _) = self.get_players(white);
        let mut attacks = Bitboard::EMPTY;
        for t in player.pieces() {
            attacks |= self.generate_attacks_from(t);
        }
        attacks
    }
    pub fn generate_attacks_from(&self, tile: Tile) -> Bitboard {
        match self.get_piece_at_tile(tile) {
            Some((p, w)) => self.generate_attacks_from_piece(tile, p, w),
            None => Bitboard::EMPTY,
        }
    }
    pub fn generate_attacks_from_piece(&self, tile: Tile, piece: Piece, white: bool) -> Bitboard {
        match piece {
            Piece::Pawn => self.generate_pawn_attacks(tile, white),
            Piece::Knight => self.tables.knight_table[Into::<usize>::into(tile)],
            Piece::Bishop => self.generate_sliding_attacks(tile, false, true),
            Piece::Rook => self.generate_sliding_attacks(tile, true, false),
            Piece::Queen => self.generate_sliding_attacks(tile, true, true),
            Piece::King => self.tables.king_table[Into::<usize>::into(tile)],
        }
    }
    fn generate_pawn_attacks(&self, tile: Tile, white: bool) -> Bitboard {
        let bb = tile.as_mask();
        match white {
            true => ((bb << 7) & !Self::FILE_H) | ((bb << 9) & !Self::FILE_A),
            false => ((bb >> 7) & !Self::FILE_A) | ((bb >> 9) & !Self::FILE_H),
        }
    }

    fn generate_sliding_attacks(
        &self,
        tile: Tile,
        straight: bool,
        diagonal: bool,
    ) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        const STRAIGHT_DELTAS: &[i8] = &[8, -8, 1, -1];      // up, down, right, left
        const DIAGONAL_DELTAS: &[i8] = &[9, -9, 7, -7];      // diag up-right, down-left, up-left, down-right

        if straight {
            for &delta in STRAIGHT_DELTAS {
                attacks |= self.slide_in_direction_attack(tile, delta);
            }
        }

        if diagonal {
            for &delta in DIAGONAL_DELTAS {
                attacks |= self.slide_in_direction_attack(tile, delta);
            }
        }

        attacks
    }
    fn slide_in_direction_attack(&self, start: Tile, delta: i8) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let mut current = Some(start);

        while let Some(tile) = current {
            current = match delta {
                8 => tile.forward(true),
                -8 => tile.backward(true),
                1 => tile.right(true),
                -1 => tile.left(true),

                9 => tile.offset(1, 1),
                -9 => tile.offset(-1, -1),
                7 => tile.offset(-1, 1),
                -7 => tile.offset(1, -1),
                _ => panic!("Invalid delta"),
            };

            if let Some(next_tile) = current {
                attacks.set_bit(next_tile, true);
                if self.occupied().get_bit(next_tile) {
                    break;
                }
            }
        }

        attacks
    }

}