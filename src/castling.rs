use std::ops::BitOr;

mod constants;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingRights(u8);

impl CastlingRights {
    // Constructors
    pub const fn new(rights: u8) -> Self {
        CastlingRights(rights)
    }
    pub fn from_fen(fen: &str) -> Self {
        let mut rights = CastlingRights::NONE;
        for c in fen.chars() {
            match c {
                'K' => rights.insert(Self::WHITE_KINGSIDE),
                'Q' => rights.insert(Self::WHITE_QUEENSIDE),
                'k' => rights.insert(Self::BLACK_KINGSIDE),
                'q' => rights.insert(Self::BLACK_QUEENSIDE),
                '-' => return Self::NONE,
                _ => {}
            }
        }
        rights
    }

    pub fn contains(&self, rights: CastlingRights) -> bool {
        self.0 & rights.0 != 0
    }

    // Modifications
    pub fn remove(&mut self, rights: CastlingRights) {
        self.0 &= !rights.0;
    }
    pub fn insert(&mut self, rights: CastlingRights) {
        self.0 |= rights.0;
    }

    // Conversions
    pub fn to_fen(&self) -> String {
        if self.0 == 0 {
            return "-".to_string();
        }

        let mut s = String::new();
        if self.contains(Self::WHITE_KINGSIDE) {
            s.push('K');
        }
        if self.contains(Self::WHITE_QUEENSIDE) {
            s.push('Q');
        }
        if self.contains(Self::BLACK_KINGSIDE) {
            s.push('k');
        }
        if self.contains(Self::BLACK_QUEENSIDE) {
            s.push('q');
        }

        s
    }
    pub fn to_u8(&self) -> u8 {
        self.0
    }
}
impl BitOr for CastlingRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        CastlingRights(self.0 | rhs.0)
    }
}