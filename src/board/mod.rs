pub mod constants;
pub mod constructors;

pub mod movegen;
pub mod movement;
pub mod check_mate;

pub mod attackgen;

pub mod helper;
pub mod debug;

use crate::{Move, Player, Tile};
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Board {
    pub white: Player,
    pub black: Player,

    pub white_turn: bool,
    pub en_passant: Option<Tile>,
    pub history: Vec<Move>,

    check_cached: Option<bool>,
}