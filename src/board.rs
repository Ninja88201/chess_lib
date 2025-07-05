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

use crate::{CastlingRights, Move, Player, Tile};
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    pub white: Player,
    pub black: Player,
    pub castling: CastlingRights,

    pub white_turn: bool,
    pub en_passant: Option<Tile>,

    // Store move & its S.A.N string while we have context 
    pub history: Vec<(Move, String)>,
    repetition_history: Vec<u64>,

    pub half_moves: u8,
    pub full_move: u32,

    white_cache: Cell<Option<bool>>,
    black_cache: Cell<Option<bool>>,
}
