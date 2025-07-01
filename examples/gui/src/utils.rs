use chess_lib::Tile;
use macroquad::prelude::*;
use crate::TILE_SIZE;

pub fn tile_to_screen(tile: Tile, flipped: bool) -> (f32, f32) {
    let (file, rank) = tile.get_coords();
    let x = if flipped { (7 - file) as f32 } else { file as f32 } * TILE_SIZE;
    let y = if flipped { rank as f32 } else { (7 - rank) as f32 } * TILE_SIZE;
    (x, y)
}

pub fn get_tile(pos: Vec2, flipped: bool) -> Option<Tile> {
    let file = (pos.x / TILE_SIZE).floor() as u8;
    let rank = (pos.y / TILE_SIZE).floor() as u8;
    if file >= 8 || rank >= 8 {
        return None;
    }
    let (file, rank) = if flipped {
        (7 - file, rank)
    } else {
        (file, 7 - rank)
    };
    Tile::new_xy(file, rank)
}
