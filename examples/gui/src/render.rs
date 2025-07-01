use chess_lib::{Board, Tile};
use macroquad::texture::Texture2D;

pub mod board;
pub mod moves;
pub mod pieces;
pub mod history;

pub fn render_board(atlas: &Texture2D, board: &Board, selected: Option<Tile>, flipped: bool) {
    board::render_board_squares(board, flipped);
    pieces::render_all_pieces(board, flipped, atlas);
    if let Some(t) = selected {
        moves::render_moves(board, t, flipped);
    }
}