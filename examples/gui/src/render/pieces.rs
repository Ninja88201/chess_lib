use macroquad::prelude::*;
use chess_lib::{Board, Piece};
use crate::{TILE_SIZE, SPRITE_SIZE, utils::tile_to_screen};

pub fn render_all_pieces(board: &Board, flipped: bool, atlas: &Texture2D) {
    for (is_white, player) in [(true, &board.white), (false, &board.black)] {
        for (i, bb) in player.bb.iter().enumerate() {
            for tile in *bb {
                let (x, y) = tile_to_screen(tile, flipped);
                draw_texture_ex(
                    atlas,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        source: Some(get_piece_sprite_rect(Piece::from_index(i), is_white)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub fn get_piece_sprite_rect(piece: Piece, white: bool) -> Rect {
    use Piece::*;
    let (row, col) = match (piece, white) {
        (Pawn, true) => (0.0, 0.0),
        (Pawn, false) => (2.0, 0.0),
        (Knight, true) => (0.0, 1.0),
        (Knight, false) => (2.0, 1.0),
        (Bishop, true) => (0.0, 2.0),
        (Bishop, false) => (2.0, 2.0),
        (Rook, true) => (1.0, 0.0),
        (Rook, false) => (3.0, 0.0),
        (Queen, true) => (1.0, 1.0),
        (Queen, false) => (3.0, 1.0),
        (King, true) => (1.0, 2.0),
        (King, false) => (3.0, 2.0),
    };
    Rect::new(col * SPRITE_SIZE, row * SPRITE_SIZE, SPRITE_SIZE, SPRITE_SIZE)
}