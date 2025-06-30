use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

use crate::{Bitboard, Tile};

impl BitAnd for Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 & rhs.0)
    }
}

impl BitAnd<&Bitboard> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: &Self) -> Self::Output {
        Bitboard::new(self.0 & rhs.0)
    }
}

impl BitAnd for &Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 | rhs.0)
    }
}

impl BitOr for &Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 | rhs.0)
    }
}

impl BitOr<&Bitboard> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: &Self) -> Self::Output {
        Bitboard::new(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard::new(!self.0)
    }
}

impl Not for &Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard::new(!self.0)
    }
}

impl Shl<i32> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self::Output {
        Bitboard::new(self.0 << rhs)
    }
}

impl Shr<i32> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: i32) -> Self::Output {
        Bitboard::new(self.0 >> rhs)
    }
}

impl From<u64> for Bitboard {
    #[inline(always)]
    fn from(val: u64) -> Self {
        Bitboard::new(val)
    }
}

impl Iterator for Bitboard {
    type Item = Tile;

    #[inline]
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
    pub fn iter(&self) -> BitIter {
        BitIter(self.0)
    }
}

pub struct BitIter(u64);

impl Iterator for BitIter {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let lsb = self.0.trailing_zeros();
        self.0 &= self.0 - 1;
        Tile::new_index(lsb as u8)
    }
}