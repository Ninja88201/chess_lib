use crate::{CastlingRights, Piece, Tile};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Move(u64);

impl Move {
    // 0 - 5    ->  From    ( Tile )
    const FROM_SHIFT: u8 = 0;

    // 6 - 11   ->  To      ( Tile )
    const TO_SHIFT: u8 = 6;

    // 12 - 15  ->  Piece   ( Piece )
    const PIECE_SHIFT: u8 = 12;

    // 16 - 19  ->  Capture ( Option<Piece> )
    const CAPTURE_SHIFT: u8 = 16;

    // 20 - 23  ->  Promte  ( Option<Piece> )
    const PROMO_SHIFT: u8 = 20;

    // 24 - 29  ->  EnPass  ( Option<Piece> )
    const EP_SHIFT: u8 = 24;

    // 30 - 33  -> Castling ( CastlingRights (u8) )
    const CASTLE_SHIFT: u8 = 30;

    // 34 - 35  -> WCache   ( Option<bool> )
    const WHITE_CACHE_SHIFT: u8 = 34;
    // 36 - 37  -> BCache   ( Option<bool> )
    const BLACK_CACHE_SHIFT: u8 = 36;

    // 38 - 45 -> PrevHalfMoves (u8)
    const PREV_HALF_MOVE_SHIFT: u8 = 38;

    fn encode_option_bool(val: Option<bool>) -> u64 {
        match val {
            None => 0b00,
            Some(false) => 0b01,
            Some(true) => 0b10,
        }
    }

    fn decode_option_bool(bits: u64) -> Option<bool> {
        match bits & 0b11 {
            0b00 => None,
            0b01 => Some(false),
            0b10 => Some(true),
            _ => None,
        }
    }

    pub fn new(
        from: Tile,
        to: Tile,

        piece: Piece,
        capture: Option<Piece>,

        en_passant: Option<Tile>,
        prev_castle: CastlingRights,
        promoted_to: Option<Piece>,

        white_cache: Option<bool>,
        black_cache: Option<bool>,

        prev_half_moves: u8,
    ) -> Self {
        let mut data = 0u64;
        data |= (from.to_u8() as u64) << Self::FROM_SHIFT;
        data |= (to.to_u8() as u64) << Self::TO_SHIFT;
        data |= (piece as u64) << Self::PIECE_SHIFT;
        data |= (capture.map(|p| p as u64 + 1).unwrap_or(0)) << Self::CAPTURE_SHIFT;
        data |= (promoted_to.map(|p| p as u64 + 1).unwrap_or(0)) << Self::PROMO_SHIFT;
        data |= (en_passant.map(|t| t.to_u8()).unwrap_or(0x3F) as u64) << Self::EP_SHIFT;
        data |= (prev_castle.to_u8() as u64) << Self::CASTLE_SHIFT;
        data |= Self::encode_option_bool(white_cache) << Self::WHITE_CACHE_SHIFT;
        data |= Self::encode_option_bool(black_cache) << Self::BLACK_CACHE_SHIFT;
        data |= (prev_half_moves as u64) << Self::PREV_HALF_MOVE_SHIFT;
        Self(data)
    }

    pub fn from(&self) -> Tile {
        Tile::new_unchecked(((self.0 >> Self::FROM_SHIFT) & 0x3F) as u8)
    }
    pub fn to(&self) -> Tile {
        Tile::new_unchecked(((self.0 >> Self::TO_SHIFT) & 0x3F) as u8)
    }
    pub fn piece(&self) -> Piece {
        unsafe { std::mem::transmute(((self.0 >> Self::PIECE_SHIFT) & 0xF) as u8) }
    }
    pub fn capture(&self) -> Option<Piece> {
        let v = (self.0 >> Self::CAPTURE_SHIFT) & 0xF;
        (v != 0).then(|| unsafe { std::mem::transmute((v - 1) as u8) })
    }
    pub fn promoted_to(&self) -> Option<Piece> {
        let v = (self.0 >> Self::PROMO_SHIFT) & 0xF;
        (v != 0).then(|| unsafe { std::mem::transmute((v - 1) as u8) })
    }
    pub fn en_passant(&self) -> Option<Tile> {
        let v = (self.0 >> Self::EP_SHIFT) & 0x3F;
        (v != 0x3F).then(|| Tile::new_unchecked(v as u8))
    }
    pub fn prev_castle(&self) -> CastlingRights {
        CastlingRights::new(((self.0 >> Self::CASTLE_SHIFT) & 0xF) as u8)
    }
    pub fn white_cache(&self) -> Option<bool> {
        Self::decode_option_bool((self.0 >> Self::WHITE_CACHE_SHIFT) & 0b11)
    }
    pub fn black_cache(&self) -> Option<bool> {
        Self::decode_option_bool((self.0 >> Self::BLACK_CACHE_SHIFT) & 0b11)
    }
    pub fn prev_half_moves(&self) -> u8 {
        ((self.0 >> Self::PREV_HALF_MOVE_SHIFT) & 0xFF) as u8
    }
}

impl Default for Move {
    fn default() -> Self {
        Self(0)
    }
}