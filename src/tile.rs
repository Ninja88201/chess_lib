use std::fmt::Display;

mod constants;
#[cfg(test)]
mod tests;

use crate::{
    BETWEEN, BISHOP_ATTACKS, BISHOP_MAGICS, Bitboard, KING_ATTACKS, KNIGHT_ATTACKS,
    ROOK_ATTACKS, ROOK_MAGICS, magics::magic_index,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile(u8);

impl Tile {
    // Constructors
    pub const fn new_index(index: u8) -> Option<Tile> {
        if index >= 64 {
            return None;
        }
        Some(Tile(index))
    }
    pub const fn new_xy(x: u8, y: u8) -> Option<Tile> {
        if x >= 8 || y >= 8 {
            return None;
        }
        Some(Tile((y * 8) + x))
    }
    pub fn from_str(s: &str) -> Option<Tile> {
        let mut chars = s.chars();
        let file = chars.next()?;
        let rank = chars.next()?;
        if chars.next().is_some() {
            return None; // too long
        }
        Tile::new_chars(file, rank)
    }
    pub const fn new_chars(file_char: char, rank_char: char) -> Option<Tile> {
        let file = match file_char {
            'a'..='h' => (file_char as u8) - b'a',
            _ => return None,
        };
        let rank = match rank_char {
            '1'..='8' => (rank_char as u8) - b'1',
            _ => return None,
        };
        Some(Tile::new_unchecked((rank << 3) | file))
    }
    pub const fn new_unchecked(index: u8) -> Tile {
        Tile(index)
    }

    // Conversions
    pub fn to_u8(&self) -> u8 {
        self.0
    }
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
    pub fn to_mask(&self) -> Bitboard {
        Bitboard::new(1u64 << self.0)
    }

    // Getting others
    pub fn get_coords(&self) -> (u8, u8) {
        (self.0 & 7, self.0 >> 3)
    }
    pub fn get_neighbours(&self) -> Bitboard {
        Bitboard::new(KING_ATTACKS[self.to_usize()])
    }

    pub fn offset(&self, dx: i8, dy: i8) -> Option<Self> {
        let (x, y) = self.get_coords();
        let nx = x as i8 + dx;
        let ny = y as i8 + dy;
        Tile::new_xy(nx as u8, ny as u8)
    }
    pub fn forward(&self, white: bool) -> Option<Self> {
        self.offset(0, if white { 1 } else { -1 })
    }
    pub fn backward(&self, white: bool) -> Option<Self> {
        self.offset(0, if white { -1 } else { 1 })
    }
    pub fn left(&self, white: bool) -> Option<Self> {
        self.offset(if white { -1 } else { 1 }, 0)
    }
    pub fn right(&self, white: bool) -> Option<Self> {
        self.offset(if white { 1 } else { -1 }, 0)
    }

    // Board rules
    pub fn is_promotion(&self, white: bool) -> bool {
        let y = self.get_coords().1;
        match white {
            true => y == 7,
            false => y == 0,
        }
    }
    pub fn is_pawn_start(&self, white: bool) -> bool {
        let y = self.get_coords().1;
        match white {
            true => y == 1,
            false => y == 6,
        }
    }

    // Attack Generation
    pub fn rook_attacks(&self, occ: Bitboard) -> Bitboard {
        let entry = &ROOK_MAGICS[self.to_usize()];
        let idx = magic_index(entry, occ);
        Bitboard::new(ROOK_ATTACKS[idx])
    }

    pub fn bishop_attacks(&self, occ: Bitboard) -> Bitboard {
        let entry = &BISHOP_MAGICS[self.to_usize()];
        let idx = magic_index(entry, occ);
        Bitboard::new(BISHOP_ATTACKS[idx])
    }

    pub fn queen_attacks(&self, occ: Bitboard) -> Bitboard {
        self.rook_attacks(occ) | self.bishop_attacks(occ)
    }

    pub fn knight_attacks(&self) -> Bitboard {
        Bitboard::new(KNIGHT_ATTACKS[self.to_usize()])
    }

    pub fn king_attacks(&self) -> Bitboard {
        Bitboard::new(KING_ATTACKS[self.to_usize()])
    }

    pub fn pawn_attacks(&self, white: bool) -> Bitboard {
        let mask = self.to_mask();
        match white {
            true => ((mask << 7) & !Bitboard::FILE_H) | ((mask << 9) & !Bitboard::FILE_A),
            false => ((mask >> 7) & !Bitboard::FILE_A) | ((mask >> 9) & !Bitboard::FILE_H),
        }
    }
    
    pub fn get_between(&self, to: Tile) -> Bitboard {
        Bitboard::new(BETWEEN[self.to_usize()][to.to_usize()])
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.get_coords();
        let file = (b'a' + x) as char;
        let rank = (y + 1).to_string();
        write!(f, "{}{}", file, rank)
    }
}
impl From<Tile> for usize {
    fn from(tile: Tile) -> Self {
        tile.0 as usize
    }
}