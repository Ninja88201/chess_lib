use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);
impl BitAnd for Bitboard
{
    type Output = Self;

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
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let s = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Some(s)
        }
    }
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard { 0: 0 };
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }
    pub fn from_bit(bit: u8) -> Self {
        Self(1u64 << bit)
    }
    pub fn to_bit(&self) -> Option<u8> {
        if self.count_ones() == 1 {
            Some(self.0.trailing_zeros() as u8)
        } else {
            None
        }
    }
    pub fn set_bit(&mut self, bit: u8, value: bool) {
        let mask = 1u64 << bit;
        if value {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }
    pub fn get_bit(&self, bit: u8) -> bool {
        let mask = 1u64 << bit;
        return self.0 & mask != 0
    }
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
}