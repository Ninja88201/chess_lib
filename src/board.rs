pub mod constructors;

pub mod check_mate;
pub mod movegen;
pub mod movement;

pub mod attackgen;

pub mod debug;
pub mod fen;
pub mod helper;

#[cfg(test)]
mod tests;

use std::cell::Cell;

use crate::{CastlingRights, Colour, History, Player, Tile};
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    pub white: Player,
    pub black: Player,
    pub castling: CastlingRights,

    pub turn: Colour,
    pub en_passant: Option<Tile>,

    pub zobrist_hash: u64,

    // Store move & its S.A.N string & zobrist hash while we have context 
    pub history: Vec<History>,
    repetition_history: Vec<u64>,

    pub half_moves: u8,
    pub full_move: u32,

    white_cache: Cell<Option<bool>>,
    black_cache: Cell<Option<bool>>,
}
