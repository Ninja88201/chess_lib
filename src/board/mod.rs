pub mod constants;
pub mod constructors;
pub mod movegen;
pub mod movement;
pub mod check_mate;
pub mod attackgen;

use std::fmt;

use crate::player::Player;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    pub white: Player,
    pub black: Player,

    pub white_turn: bool,
    pub history: Vec<Move>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}
impl Piece {
    pub fn from_index(index: usize) -> Self
    {
        match index {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece index"),
        }
    }
    pub fn to_fen_char(&self, white: bool) -> char {
        let c = match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };
        if white { c.to_ascii_uppercase() } else { c }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub white_turn: bool,
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub capture: Option<Piece>,

    // New fields for undoing special state
    pub prev_short_castle: (bool, bool), // (white, black)
    pub prev_long_castle: (bool, bool),  // (white, black)
    pub was_promotion: bool,
    pub promoted_to: Option<Piece>, // Only relevant if was_promotion = true
}
impl Move {
    pub fn new(
        white_turn: bool,
        from: u8,
        to: u8,
        piece: Piece,
        capture: Option<Piece>,
        prev_short_castle: (bool, bool),
        prev_long_castle: (bool, bool),
        was_promotion: bool,
        promoted_to: Option<Piece>,
    ) -> Self {
        Self {
            white_turn,
            from,
            to,
            piece,
            capture,
            prev_short_castle,
            prev_long_castle,
            was_promotion,
            promoted_to,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    IllegalMove,
    WrongTurn,
    PiecePinned,
    
}
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveError::IllegalMove => write!(f, "Illegal move"),
            MoveError::WrongTurn => write!(f, "It's the wrong player's turn"),
            MoveError::PiecePinned => write!(f, "Piece is pinned"),
        }
    }
}