use std::fmt;

use crate::{Board, CastlingRights, Piece, Tile};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub white_turn: bool,

    // Movement
    pub from: Tile,
    pub to: Tile,

    // Piece types
    pub piece: Piece,
    pub capture: Option<Piece>,
    
    // Castling Rights
    pub prev_castle: CastlingRights,
    
    // Special cases
    pub promoted_to: Option<Piece>,
    pub en_passant: Option<Tile>,

    // Cached data
    pub check_cached: Option<bool>
}
impl Move {
    pub fn new(
        white_turn: bool,
        from: Tile,
        to: Tile,
        piece: Piece,
        capture: Option<Piece>,
        en_passant: Option<Tile>,
        prev_castle: CastlingRights,
        promoted_to: Option<Piece>,
        check_cached: Option<bool>,
    ) -> Self {
        Self {
            white_turn,
            from,
            to,
            piece,
            capture,
            en_passant,
            prev_castle,
            promoted_to,
            check_cached
        }
    }
}
impl Default for Move
{
    fn default() -> Self {
        Self { 
            white_turn: true, 
            from: Board::A1, 
            to: Board::A1, 
            piece: Piece::Pawn, 
            capture: None, 
            prev_castle: CastlingRights::ALL, 
            promoted_to: None, 
            en_passant: None, 
            check_cached: None }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Castling
        if self.piece == Piece::King {
            if self.from == Board::E1 && self.to == Board::G1 || self.from == Board::E8 && self.to == Board::G8 {
                return write!(f, "O-O");
            } else if self.from == Board::E1 && self.to == Board::C1 || self.from == Board::E8 && self.to == Board::C8 {
                return write!(f, "O-O-O");
            }
        }

        let mut s = String::new();

        if self.piece != Piece::Pawn {
            s.push_str(&self.piece.to_string());
        }

        if self.capture.is_some() {
            if self.piece == Piece::Pawn {
                // For pawn captures, include file
                let (from_file, _) = self.from.get_coords();
                s.push((b'a' + from_file as u8) as char);
            }
            s.push('x');
        }

        s.push_str(&self.to.to_string());

        if let Some(promo) = self.promoted_to {
            s.push('=');
            s.push_str(&promo.to_string());
        }

        if self.en_passant.is_some() {
            s.push_str(" e.p.");
        }

        write!(f, "{}", s)
    }
}