use macroquad::prelude::*;
use chess_lib::{Board, MoveList, Tile};
use crate::{TILE_SIZE, utils::tile_to_screen};

pub fn render_moves(board: &Board, selected: Tile, flipped: bool) {
    let mut moves = MoveList::new();
    board.generate_legal_moves_from(selected, &mut moves);
    for m in moves.iter() {
        let (x, y) = tile_to_screen(m.to(), flipped);
        draw_circle(x + TILE_SIZE / 2.0, y + TILE_SIZE / 2.0, TILE_SIZE * 0.2, DARKGRAY);
    }
}
