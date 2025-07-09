use macroquad::prelude::*;
use chess_lib::{Board, Colour, Piece};
use crate::{TILE_SIZE, SPRITE_SIZE, utils::tile_to_screen};

pub fn render_all_pieces(board: &Board, flipped: bool, atlas: &Texture2D) {
    for (colour, player) in [(Colour::White, &board.white), (Colour::Black, &board.black)] {
        for (i, bb) in player.bb.iter().enumerate() {
            for tile in bb.iter() {
                let (x, y) = tile_to_screen(tile, flipped);
                draw_texture_ex(
                    atlas,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        source: Some(get_piece_sprite_rect(Piece::from_index(i), colour)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub fn get_piece_sprite_rect(piece: Piece, colour: Colour) -> Rect {
    use Piece::*;
    let (row, col) = match (piece, colour) {
        (Pawn, Colour::White) => (0.0, 0.0),
        (Pawn, Colour::Black) => (2.0, 0.0),
        (Knight, Colour::White) => (0.0, 1.0),
        (Knight, Colour::Black) => (2.0, 1.0),
        (Bishop, Colour::White) => (0.0, 2.0),
        (Bishop, Colour::Black) => (2.0, 2.0),
        (Rook, Colour::White) => (1.0, 0.0),
        (Rook, Colour::Black) => (3.0, 0.0),
        (Queen, Colour::White) => (1.0, 1.0),
        (Queen, Colour::Black) => (3.0, 1.0),
        (King, Colour::White) => (1.0, 2.0),
        (King, Colour::Black) => (3.0, 2.0),
    };
    Rect::new(col * SPRITE_SIZE, row * SPRITE_SIZE, SPRITE_SIZE, SPRITE_SIZE)
}