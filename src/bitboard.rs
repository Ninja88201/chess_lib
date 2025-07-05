use crate::Tile;
use std::fmt;

mod constants;
#[cfg(test)]
mod tests;
mod bit_manip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(0xFFFF_FFFF_FFFF_FFFF);

    #[inline(always)]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    #[inline(always)]
    pub fn from_tile(tile: Tile) -> Self {
        Self(1u64 << tile.to_u8())
    }
    pub fn to_u64(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn to_bit(&self) -> Option<Tile> {
        if self.count_ones() == 1 {
            Tile::new_index(self.0.trailing_zeros() as u8)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn set_bit(&mut self, bit: Tile, value: bool) {
        let mask = 1u64 << bit.to_u8();
        if value {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }

    #[inline(always)]
    pub fn get_bit(&self, bit: Tile) -> bool {
        (self.0 >> bit.to_u8()) & 1 != 0
    }

    #[inline(always)]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    pub fn some(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub fn none(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn bits(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let bit = (self.0 >> idx) & 1;
                write!(f, "{} ", if bit == 1 { '1' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}