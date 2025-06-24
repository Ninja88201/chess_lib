use std::ops::{BitOr, BitOrAssign};

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum CastlingRights {
//     None,
//     KingSide,
//     QueenSide,
//     Both,
// }
// impl CastlingRights
// {
//     pub fn to_fen(&self, white: bool) -> String {
//         let result = match self {
//             CastlingRights::None => "",
//             CastlingRights::KingSide => "k",
//             CastlingRights::QueenSide => "q",
//             CastlingRights::Both => "kq",
//         };
//         if white {
//             result.to_uppercase()
//         } else {
//             result.to_string()
//         }
//     }
//     pub fn short_castle(&self) -> bool {
//         match self {
//             CastlingRights::None => false,
//             CastlingRights::KingSide => true,
//             CastlingRights::QueenSide => false,
//             CastlingRights::Both => true,
//         }
//     }
//     pub fn long_castle(&self) -> bool {
//         match self {
//             CastlingRights::None => false,
//             CastlingRights::KingSide => false,
//             CastlingRights::QueenSide => true,
//             CastlingRights::Both => true,
//         }
//     }
// }
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub const NONE: CastlingRights = CastlingRights(0b0000);
    pub const WHITE_KINGSIDE:  CastlingRights = CastlingRights(0b0001);
    pub const WHITE_QUEENSIDE: CastlingRights = CastlingRights(0b0010);
    pub const BLACK_KINGSIDE:  CastlingRights = CastlingRights(0b0100);
    pub const BLACK_QUEENSIDE: CastlingRights = CastlingRights(0b1000);
    pub const ALL: CastlingRights = CastlingRights(0b1111);

    pub fn empty() -> Self {
        CastlingRights(0)
    }

    pub fn contains(&self, rights: CastlingRights) -> bool {
        self.0 & rights.0 != 0
    }

    pub fn remove(&mut self, rights: CastlingRights) {
        self.0 &= !rights.0;
    }

    pub fn insert(&mut self, rights: CastlingRights) {
        self.0 |= rights.0;
    }
        pub fn from_fen(fen: &str) -> Self {
        let mut rights = CastlingRights::NONE;
        for c in fen.chars() {
            match c {
                'K' => rights |= Self::WHITE_KINGSIDE,
                'Q' => rights |= Self::WHITE_QUEENSIDE,
                'k' => rights |= Self::BLACK_KINGSIDE,
                'q' => rights |= Self::BLACK_QUEENSIDE,
                '-' => return Self::empty(),
                _ => {}
            }
        }
        rights
    }

    /// Converts the castling rights to a FEN string
    pub fn to_fen(&self) -> String {
        if self.0 == 0 {
            return "-".to_string();
        }

        let mut s = String::new();
        if self.contains(Self::WHITE_KINGSIDE)  { s.push('K'); }
        if self.contains(Self::WHITE_QUEENSIDE) { s.push('Q'); }
        if self.contains(Self::BLACK_KINGSIDE)  { s.push('k'); }
        if self.contains(Self::BLACK_QUEENSIDE) { s.push('q'); }

        s
    }
}
impl BitOr for CastlingRights
{
    type Output = CastlingRights;

    fn bitor(self, rhs: Self) -> Self::Output {
        CastlingRights(self.0 | rhs.0)
    }
}
impl BitOrAssign for CastlingRights
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}