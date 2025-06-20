pub mod constants;
pub mod constructors;
pub mod movegen;
pub mod movement;
pub mod check_mate;
pub mod attackgen;
pub mod helper;

use std::fmt;

use crate::{player::Player, tile::Tile};
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    pub white: Player,
    pub black: Player,

    pub white_turn: bool,
    pub en_passant: Option<Tile>,
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
    pub from: Tile,
    pub to: Tile,
    pub piece: Piece,
    pub capture: Option<Piece>,
    pub en_passant: Option<Tile>,

    pub prev_white_castle: CastlingRights,
    pub prev_black_castle: CastlingRights,
    pub promoted_to: Option<Piece>,
}
impl Move {
    pub fn new(
        white_turn: bool,
        from: Tile,
        to: Tile,
        piece: Piece,
        capture: Option<Piece>,
        en_passant: Option<Tile>,
        prev_white_castle: CastlingRights,
        prev_black_castle: CastlingRights,
        promoted_to: Option<Piece>,
    ) -> Self {
        Self {
            white_turn,
            from,
            to,
            piece,
            capture,
            en_passant,
            prev_white_castle,
            prev_black_castle,
            promoted_to,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    NoPieceSelected,
    SameTile,
    FriendlyCapture,
    IllegalMove,
    WrongTurn,
    PiecePinned,
    Stalemate,
    Cancelled,
}
impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveError::IllegalMove => write!(f, "Illegal move"),
            MoveError::WrongTurn => write!(f, "It's the wrong player's turn"),
            MoveError::PiecePinned => write!(f, "Piece is pinned"),
            MoveError::Stalemate => write!(f, "The board is in a stalemate"),
            MoveError::NoPieceSelected => write!(f, "No piece is selected"),
            MoveError::SameTile => write!(f, "Same tile selected"),
            MoveError::FriendlyCapture => write!(f, "Cannot capture own piece"),
            MoveError::Cancelled => write!(f, "Cancelled move"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CastlingRights {
    None,
    KingSide,
    QueenSide,
    Both,
}
impl CastlingRights
{
    pub fn to_fen(&self, white: bool) -> String {
        let result = match self {
            CastlingRights::None => "",
            CastlingRights::KingSide => "k",
            CastlingRights::QueenSide => "q",
            CastlingRights::Both => "kq",
        };
        if white {
            result.to_uppercase()
        } else {
            result.to_string()
        }
    }
    pub fn short_castle(&self) -> bool {
        match self {
            CastlingRights::None => false,
            CastlingRights::KingSide => true,
            CastlingRights::QueenSide => false,
            CastlingRights::Both => true,
        }
    }
    pub fn long_castle(&self) -> bool {
        match self {
            CastlingRights::None => false,
            CastlingRights::KingSide => false,
            CastlingRights::QueenSide => true,
            CastlingRights::Both => true,
        }
    }
}