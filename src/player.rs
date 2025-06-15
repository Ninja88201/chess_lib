use crate::board::Piece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub bb: [u64; 6],

    pub short_castle: bool,
    pub long_castle: bool,
    pub auto_queen: bool,
}

impl Player {
    /// Create a new player with an empty board (no pieces).
    pub fn new_empty() -> Self {
        Self {
            bb: [0; 6],
            short_castle: true,
            long_castle: true,
            auto_queen: true,
        }
    }

    /// Create a new player with the standard white piece positions.
    pub fn new_white() -> Self {
        Self {
            bb: [
                0x0000_0000_0000_FF00,
                0x0000_0000_0000_0042,
                0x0000_0000_0000_0024,
                0x0000_0000_0000_0081,
                0x0000_0000_0000_0008,
                0x0000_0000_0000_0010,
            ],
            short_castle: true,
            long_castle: true,
            auto_queen: true,
        }
    }

    /// Create a new player with the standard black piece positions.
    pub fn new_black() -> Self {
        Self {
            bb: [
                0x00FF_0000_0000_0000,
                0x4200_0000_0000_0000,
                0x2400_0000_0000_0000,
                0x8100_0000_0000_0000,
                0x0800_0000_0000_0000,
                0x1000_0000_0000_0000,
            ],
            short_castle: true,
            long_castle: true,
            auto_queen: true,
        }
    }

    pub fn pieces(&self) -> u64 {
        self.bb[Piece::Pawn as usize]
            | self.bb[Piece::Knight as usize]
            | self.bb[Piece::Bishop as usize]
            | self.bb[Piece::Rook as usize]
            | self.bb[Piece::Queen as usize]
            | self.bb[Piece::King as usize]
    }

    pub fn remove_piece(&mut self, square: u8) -> Option<Piece> {
        let mask = 1u64 << square;
        for i in 0..self.bb.len() {
            if self.bb[i] & mask != 0 {
                self.bb[i] &= !mask;
                return Some(Piece::from_index(i));
            }
        }
        None
    }
    pub fn remove_piece_type(&mut self, piece: Piece, square: u8) -> Option<Piece> {
        let mask = 1u64 << square;
        
        if self.bb[piece as usize] & mask != 0 {
            self.bb[piece as usize] &= !mask;
            return Some(piece);
        }

        None
    }

    pub fn place_piece(&mut self, piece: Piece, square: u8) {
        let mask = 1u64 << square;
        self.bb[piece as usize] |= mask;
    }
    pub fn get_piece(&self, square: u8) -> Option<Piece>
    {
        let mask = 1u64 << square;
        for i in 0..self.bb.len() {
            if self.bb[i] & mask != 0 {
                return Some(Piece::from_index(i));
            }
        }
        None
    }
    pub fn get_king_square(&self) -> u8 {
        let king_bb = self.bb[Piece::King as usize];
        debug_assert!(king_bb.count_ones() == 1, "King bitboard should have exactly one bit set.");
        king_bb.trailing_zeros() as u8
    }
}