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
        let (piece, white) = match self.get_piece_at_tile(tile) {
            Some(x) => x,
            None => return Bitboard::EMPTY,
        };
        return self.generate_attacks_from_piece(tile, piece, white);
    }
    pub fn generate_attacks_from_piece(&self, tile: Tile, piece: Piece, white: bool) -> Bitboard {
        match piece {
            Piece::Pawn => self.generate_pawn_attacks(tile, white),
            Piece::Knight => self.generate_knight_attacks(tile),
            Piece::Bishop => self.generate_sliding_attacks(tile, false, true),
            Piece::Rook => self.generate_sliding_attacks(tile, true, false),
            Piece::Queen => self.generate_sliding_attacks(tile, true, true),
            Piece::King => self.generate_king_attacks(tile),
        }
    }
    fn generate_pawn_attacks(&self, tile: Tile, white: bool) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        if let Some(t) = tile.forward(white).and_then(|t| t.left(white)) {
            attacks.set_bit(t, true);
        }
        if let Some(t) = tile.forward(white).and_then(|t| t.right(white)) {
            attacks.set_bit(t, true);
        }

        attacks
    }
    fn generate_knight_attacks(&self, tile: Tile) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let (x, y) = tile.get_coords();
        for (dx, dy) in &Board::KNIGHT_OFFSETS {
            let nx = (x as i8) + dx;
            let ny = y as i8 + dy;
            if nx < 0 || ny < 0 { continue; }
            let dest = Tile::new_xy(nx as u8, ny as u8);
            
            if let Some(t) = dest {
                attacks.set_bit(t, true);
            }
        }

        attacks
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
    fn slide_in_direction_attack(
        &self,
        tile: Tile,
        delta: i8,
    ) -> Bitboard {
        let mut result = Bitboard::EMPTY;
        let mut curr = Some(tile);

        loop {
            if let Some(c) = curr {
                curr = match delta {
                    8 => c.forward(true),
                    -8 => c.backward(true),
                    1 => c.right(true),
                    -1 => c.left(true),
    
                    9 => c.forward(true).and_then(|t| t.right(true)),
                    -9 => c.backward(true).and_then(|t| t.left(true)),
                    7 => c.forward(true).and_then(|t| t.left(true)),
                    -7 => c.backward(true).and_then(|t| t.right(true)),
                    _ => panic!("Not a valid delta")
                };
            }
            else {
                break;
            }
            if let Some(c) = curr {
                result.set_bit(c, true);
    
                if self.occupied().get_bit(c) {
                    break;
                }    
            }
            else {
                break;
            }
        }

        result
    }

    fn generate_king_attacks(&self, tile: Tile) -> Bitboard {
        tile.get_neighbours()
    }
}