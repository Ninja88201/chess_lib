use macroquad::prelude::*;
use chess_lib::{Board, Tile};
use crate::{utils::{get_tile, tile_to_screen}, TILE_SIZE};

pub fn render_board_squares(board: &Board, flipped: bool) {
    let white_in_check = board.is_in_check(true);
    let black_in_check = board.is_in_check(false);
    
    let highlight = get_tile(mouse_position().into(), flipped);
    for file in 0..8 {
        for rank in 0..8 {
            let tile = Tile::new_xy(file, rank).unwrap();
            let (x, y) = tile_to_screen(tile, flipped);

            let is_light = (rank + file) % 2 == if flipped { 1 } else { 0 };
            let mut color = if is_light {
                Color::from_rgba(240, 217, 181, 255)
            } else {
                Color::from_rgba(181, 136, 99, 255)
            };

            if let Some(pos) = highlight {
                if Tile::new_xy(file, rank).unwrap() == pos {
                    color.a = 0.75;
                }
            }

            if white_in_check && tile == board.white.king_tile() {
                color.r = 1.0; color.g *= 0.5; color.b *= 0.5;
            }

            if black_in_check && tile == board.black.king_tile() {
                color.r = 1.0; color.g *= 0.5; color.b *= 0.5;
            }

            draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
        }
    }
}