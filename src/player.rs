use crate::{bitboard::Bitboard, board::{CastlingRights, Piece}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub bb: [Bitboard; 6],

    pub castling: CastlingRights,
}

impl Player {
    /// Create a new player with an empty board (no pieces).
    pub fn new_empty() -> Self {
        Self {
            bb: [Bitboard::EMPTY; 6],
            castling: CastlingRights::Both,
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
                Bitboard::new(0x0000_0000_0000_0010),
            ],
            castling: CastlingRights::Both,
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
                Bitboard::new(0x1000_0000_0000_0000),
            ],
            castling: CastlingRights::Both,
        }
    }

    pub fn pieces(&self) -> Bitboard {
        self.bb[Piece::Pawn as usize]
            | self.bb[Piece::Knight as usize]
            | self.bb[Piece::Bishop as usize]
            | self.bb[Piece::Rook as usize]
            | self.bb[Piece::Queen as usize]
            | self.bb[Piece::King as usize]
    }

    pub fn remove_piece(&mut self, square: u8) -> Option<Piece> {
        for i in 0..self.bb.len() {
            if self.bb[i].get_bit(square) {
                self.bb[i].set_bit(square, false);
                return Some(Piece::from_index(i));
            }
        }
        None
    }
    pub fn remove_piece_type(&mut self, piece: Piece, square: u8) -> Option<Piece> {
        if self.bb[piece as usize].get_bit(square) {
            self.bb[piece as usize].set_bit(square, false);
            return Some(piece);
        }

        None
    }

    pub fn place_piece(&mut self, piece: Piece, square: u8) {
        self.bb[piece as usize].set_bit(square, true);
    }
    pub fn move_piece(&mut self, from: u8, to: u8) {
        let p = self.remove_piece(from).unwrap();
        self.place_piece(p, to);
    }
    pub fn get_piece(&self, square: u8) -> Option<Piece>
    {
        for i in 0..self.bb.len() {
            if self.bb[i].get_bit(square) {
                return Some(Piece::from_index(i));
            }
        }
        None
    }
    pub fn get_king_square(&self) -> u8 {
        match self.bb[Piece::King as usize].to_bit() {
            Some(s) => s,
            None => panic!("Can only have 1 king"),
        }
    }
}