pub mod constructors;

pub mod check_mate;
pub mod movegen;
pub mod movement;

pub mod attackgen;

pub mod debug;
pub mod fen;
pub mod helper;
pub mod history;

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
    pub history: Vec<Move>,

    pub half_moves: u8,
    pub full_move: u32,

    white_cache: Cell<Option<bool>>,
    black_cache: Cell<Option<bool>>,
}
