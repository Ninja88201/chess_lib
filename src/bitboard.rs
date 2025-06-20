use std::ops::*;

use crate::tile::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);
impl BitAnd for Bitboard
{
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd for &Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<&Bitboard> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: &Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOr for &Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOr<&Bitboard> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Not for &Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Iterator for Bitboard {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let s = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Tile::new_index(s)
        }
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard { 0: 0 };
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }
    pub fn from_bit(bit: Tile) -> Self {
        Self(1u64 << bit.0)
    }
    pub fn to_bit(&self) -> Option<Tile> {
        if self.count_ones() == 1 {
            Tile::new_index(self.0.trailing_zeros() as u8)
        } else {
            None
        }
    }
    pub fn set_bit(&mut self, bit: Tile, value: bool) {
        let mask = bit.as_mask();
        if value {
            *self |= mask;
        } else {
            *self &= !mask;
        }
    }
    pub fn get_bit(&self, bit: Tile) -> bool {
        let mask = bit.as_mask();
        return self & &mask != Bitboard::EMPTY
    }
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
}