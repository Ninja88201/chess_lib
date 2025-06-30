use crate::{Bitboard, Piece, Tile};

mod constants;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub bb: [Bitboard; 6],
    pub pieces: Bitboard,

    pub selected_tile: Option<Tile>,
}

impl Player {
    pub fn new_empty() -> Self {
        Self {
            bb: [Bitboard::EMPTY; 6],
            pieces: Bitboard::EMPTY,
            selected_tile: None,
        }
    }
    /// Create a new player with the standard white piece positions.
    pub fn new_white() -> Self {
        Self {
            bb: [
                Player::WHITE_PAWNS,
                Player::WHITE_KNIGHTS,
                Player::WHITE_BISHOPS,
                Player::WHITE_ROOKS,
                Player::WHITE_QUEEN,
                Player::WHITE_KING,
            ],
            pieces: Player::WHITE_PIECES,
            selected_tile: None,
        }
    }

    /// Create a new player with the standard black piece positions.
    pub fn new_black() -> Self {
        Self {
            bb: [
                Player::BLACK_PAWNS,
                Player::BLACK_KNIGHTS,
                Player::BLACK_BISHOPS,
                Player::BLACK_ROOKS,
                Player::BLACK_QUEEN,
                Player::BLACK_KING,
            ],
            pieces: Player::BLACK_PIECES,
            selected_tile: None,
        }
    }
    pub fn attackers(&self) -> Bitboard {
        self.pieces & !self.bb[Piece::King as usize]
    }
    pub fn king_tile(&self) -> Tile {
        self.bb[Piece::King as usize].to_bit().unwrap()
    }

    pub fn remove_piece(&mut self, tile: Tile) -> Option<Piece> {
        for piece in Piece::ALL_PIECES {
            if self.bb[piece as usize].get_bit(tile) {
                self.bb[piece as usize].set_bit(tile, false);
                self.pieces.set_bit(tile, false);
                return Some(piece);
            }
        }
        None
    }
    pub fn remove_piece_type(&mut self, piece: Piece, tile: Tile) -> Option<Piece> {
        if self.bb[piece as usize].get_bit(tile) {
            self.bb[piece as usize].set_bit(tile, false);
            self.pieces.set_bit(tile, false);
            return Some(piece);
        }

        None
    }

    pub fn place_piece(&mut self, piece: Piece, tile: Tile) {
        self.bb[piece as usize].set_bit(tile, true);
        self.pieces.set_bit(tile, true);
    }
    pub fn move_piece(&mut self, from: Tile, to: Tile) {
        if let Some(p) = self.remove_piece(from) {
            self.place_piece(p, to);
        }
    }
    pub fn get_piece(&self, tile: Tile) -> Option<Piece> {
        if (self.pieces & tile.to_mask()).none() {
            return None;
        }
        for i in 0..self.bb.len() {
            if self.bb[i].get_bit(tile) {
                return Some(Piece::from_index(i));
            }
        }
        None
    }
}
