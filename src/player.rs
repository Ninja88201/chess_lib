use crate::{Bitboard, Board, CastlingRights, Piece, Tile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub bb: [Bitboard; 5],
    pub king_tile: Tile,

    pub selected_tile: Option<Tile>,
}

impl Player {
    pub fn new_empty() -> Self {
        Self {
            bb: [Bitboard::EMPTY; 5],
            king_tile: Board::E1,
            selected_tile: None,
        }
    }
    /// Create a new player with the standard white piece positions.
    pub fn new_white() -> Self {
        Self {
            bb: [
                Bitboard::new(0x0000_0000_0000_FF00),
                Bitboard::new(0x0000_0000_0000_0042),
                Bitboard::new(0x0000_0000_0000_0024),
                Bitboard::new(0x0000_0000_0000_0081),
                Bitboard::new(0x0000_0000_0000_0008),
                // Bitboard::new(0x0000_0000_0000_0010),
            ],
            king_tile: Board::E1,
            selected_tile: None,
        }
    }

    /// Create a new player with the standard black piece positions.
    pub fn new_black() -> Self {
        Self {
            bb: [
                Bitboard::new(0x00FF_0000_0000_0000),
                Bitboard::new(0x4200_0000_0000_0000),
                Bitboard::new(0x2400_0000_0000_0000),
                Bitboard::new(0x8100_0000_0000_0000),
                Bitboard::new(0x0800_0000_0000_0000),
                // Bitboard::new(0x1000_0000_0000_0000),
            ],
            king_tile: Board::E8,
            selected_tile: None,
        }
    }

    pub fn pieces(&self) -> Bitboard {
        self.bb[Piece::Pawn as usize]
            | self.bb[Piece::Knight as usize]
            | self.bb[Piece::Bishop as usize]
            | self.bb[Piece::Rook as usize]
            | self.bb[Piece::Queen as usize]
            | self.king_tile.as_mask()
    }
    pub fn attackers(&self) -> Bitboard {
        self.bb[Piece::Pawn as usize]
            | self.bb[Piece::Knight as usize]
            | self.bb[Piece::Bishop as usize]
            | self.bb[Piece::Rook as usize]
            | self.bb[Piece::Queen as usize]
    }

    pub fn remove_piece(&mut self, tile: Tile) -> Option<Piece> {
        if tile == self.king_tile {
            return Some(Piece::King);
        }
        for i in 0..self.bb.len() {
            if let Some(p) = self.remove_piece_type(Piece::from_index(i), tile) {
                return Some(p);
            }
        }
        None
    }
    pub fn remove_piece_type(&mut self, piece: Piece, tile: Tile) -> Option<Piece> {
        if piece == Piece::King {
            return Some(Piece::King);
        }
        if self.bb[piece as usize].get_bit(tile) {
            self.bb[piece as usize].set_bit(tile, false);
            return Some(piece);
        }

        None
    }

    pub fn place_piece(&mut self, piece: Piece, tile: Tile) {
        if piece == Piece::King {
            self.king_tile = tile;
            return;
        }
        self.bb[piece as usize].set_bit(tile, true);
    }
    pub fn move_piece(&mut self, from: Tile, to: Tile) {
        if let Some(p) = self.remove_piece(from) {
            self.place_piece(p, to);
        }
    }
    pub fn get_piece(&self, tile: Tile) -> Option<Piece>
    {
        if tile == self.king_tile {
            return Some(Piece::King);
        }
        for i in 0..self.bb.len() {
            if self.bb[i].get_bit(tile) {
                return Some(Piece::from_index(i));
            }
        }
        None
    }

}