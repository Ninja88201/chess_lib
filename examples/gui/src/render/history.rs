use chess_lib::Board;
use macroquad::{color::WHITE, text::draw_multiline_text};
use macroquad::prelude::*;

use crate::consts::TILE_SIZE;


pub fn render_recent_history(board: &Board) {
    let moves = board.get_move_history();

    let all_moves: Vec<&str> = moves.lines().collect();

    let recent_moves = if all_moves.len() > 20 {
        &all_moves[all_moves.len() - 20..]
    } else {
        &all_moves[..]
    };

    let recent_moves_text = recent_moves.join("\n");

    let font_size = 30;
    let x = TILE_SIZE * 8.0;
    let y = measure_text(&recent_moves_text, None, font_size, 1.0).offset_y; 

    draw_multiline_text(
        &recent_moves_text,
        x,
        y,
        font_size as f32,
        None,
        WHITE,
    );
}

