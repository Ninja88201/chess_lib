use crate::{Disambig, Piece, Tile};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(transparent)]
pub struct SanMove(u32);
impl SanMove {
    // 1 - 3    -> Piece    { Piece }
    const PIECE_SHIFT: u8 = 0;

    // 4 - 5    -> DKind    { Disambig }
    const D_KIND_SHIFT: u8 = 4;

    // 6 - 8   -> DFile    { u8 }
    const D_FILE_SHIFT: u8 = 6;

    // 9 - 11   -> DRank    { u8 }
    const D_RANK_SHIFT: u8 = 9;

    // 12       -> Capture  { bool }
    const CAP_SHIFT: u8 = 12;

    // 13 - 18  ->  Target  { Tile }
    const TO_SHIFT: u8 = 13;

    // 19 - 21  ->  Promote { ~Piece }
    const PROMO_SHIFT: u8 = 19;

    // 22 - 23  ->  Castling { u8 }
    const CASTLE_SHIFT: u8 = 22;

    // 24       -> Check    { bool }
    const CHECK_SHIFT: u8 = 24;
    
    // 25       -> Mate     { bool }
    const MATE_SHIFT: u8 = 25;
    
    pub fn new(
        piece: Piece, 
        disambig: Option<Disambig>, 
        capture: bool, 
        to: Tile, 
        promotion: Option<Piece>,
        kingside_castle: bool,
        queenside_castle: bool,
        is_check: bool,
        is_mate: bool,
    ) -> Self {
        let mut data = 0u32;

        data |= (piece as u32) << Self::PIECE_SHIFT;
        match disambig {
            Some(d) => {
                match d {
                    Disambig::File(f) => {
                        data |= 0b01 << Self::D_KIND_SHIFT;
                        data |= (f as u32) << Self::D_FILE_SHIFT;
                    },
                    Disambig::Rank(r) => {
                        data |= 0b10 << Self::D_KIND_SHIFT;
                        data |= (r as u32) << Self::D_RANK_SHIFT;
                    },
                    Disambig::FileRank(tile) => {
                        data |= 0b11 << Self::D_KIND_SHIFT;
                        let (file, rank) = tile.get_coords();
                        data |= (file as u32) << Self::D_FILE_SHIFT;
                        data |= (rank as u32) << Self::D_RANK_SHIFT;
                    },
                }
            },
            None => data |= 0u32 << Self::D_KIND_SHIFT,
        }
        data |= (capture as u32) << Self::CAP_SHIFT;
        data |= (to.to_u32()) << Self::TO_SHIFT;
        match promotion {
            Some(p) => {
                match p {
                    Piece::Pawn => panic!("Not a valid promotion piece"),
                    Piece::Knight => data |= 0b001 << Self::PROMO_SHIFT,
                    Piece::Bishop => data |= 0b010 << Self::PROMO_SHIFT,
                    Piece::Rook => data |= 0b011 << Self::PROMO_SHIFT,
                    Piece::Queen => data |= 0b100 << Self::PROMO_SHIFT,
                    Piece::King => panic!("Not a valid promotion piece"),
                }
            },
            None => data |= 0u32 << Self::PROMO_SHIFT,
        }
        match (kingside_castle, queenside_castle) {
            (true, true) => panic!("Cannot castle on both sides in the same turn"),
            (true, false) => data |= 0b10 << Self::CASTLE_SHIFT,
            (false, true) => data |= 0b01 << Self::CASTLE_SHIFT,
            (false, false) => data |= 0b00 << Self::CASTLE_SHIFT,
        }
        data |= (is_check as u32) << Self::CHECK_SHIFT;
        data |= (is_mate as u32) << Self::MATE_SHIFT;

        Self(data)
    }

    pub fn piece(&self) -> Piece {
        Piece::from_index(((self.0 >> Self::PIECE_SHIFT) & 0b111) as usize)
    }

    pub fn disambig(&self) -> Option<Disambig> {
        let kind = (self.0 >> Self::D_KIND_SHIFT) & 0b11;
        let file = ((self.0 >> Self::D_FILE_SHIFT) & 0b111) as u8;
        let rank = ((self.0 >> Self::D_RANK_SHIFT) & 0b111) as u8;

        match kind {
            0 => None,
            1 => Some(Disambig::File(file)),
            2 => Some(Disambig::Rank(rank)),
            3 => Some(Disambig::FileRank(Tile::new_xy(file, rank).unwrap())),
            _ => unreachable!(),
        }
    }

    pub fn capture(&self) -> bool {
        ((self.0 >> Self::CAP_SHIFT) & 1) != 0
    }

    pub fn to(&self) -> Tile {
        Tile::new_index(((self.0 >> Self::TO_SHIFT) & 0b11_1111) as u8).unwrap()
    }

    pub fn promotion(&self) -> Option<Piece> {
        match (self.0 >> Self::PROMO_SHIFT) & 0b111 {
            0 => None,
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            _ => panic!("Invalid promotion value"),
        }
    }

    pub fn is_kingside_castle(&self) -> bool {
        ((self.0 >> Self::CASTLE_SHIFT) & 0b11) == 0b10
    }

    pub fn is_queenside_castle(&self) -> bool {
        ((self.0 >> Self::CASTLE_SHIFT) & 0b11) == 0b01
    }

    pub fn is_check(&self) -> bool {
        ((self.0 >> Self::CHECK_SHIFT) & 1) != 0
    }

    pub fn is_mate(&self) -> bool {
        ((self.0 >> Self::MATE_SHIFT) & 1) != 0
    }

    pub fn set_mate(&mut self, value: bool) {
        self.0 |= (value as u32) << Self::MATE_SHIFT;
    }
    pub fn set_check(&mut self, value: bool) {
        self.0 |= (value as u32) << Self::CHECK_SHIFT;
    }
}
impl std::fmt::Display for SanMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_kingside_castle() {
            return write!(f, "O-O");
        } else if self.is_queenside_castle() {
            return write!(f, "O-O-O");
        }

        write!(f, "{}", self.piece().to_san_char())?;

        if let Some(d) = &self.disambig() {
            write!(f, "{d}")?;
        }

        if self.capture() {
            write!(f, "x")?;
        }

        write!(f, "{}", self.to())?;

        if let Some(p) = self.promotion() {
            write!(f, "={}", p.to_san_char())?;
        }

        if self.is_check() {
            write!(f, "+")?;
        } else if self.is_mate() {
            write!(f, "#")?;
        }

        Ok(())
    }
}