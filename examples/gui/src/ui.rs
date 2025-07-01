use chess_lib::{Board, Piece, Tile};
use macroquad::prelude::*;

use crate::{utils::tile_to_screen, TILE_SIZE};

pub enum PromotionResult {
    Chosen(Piece),
    Cancelled,
    None,
}

pub fn render_promotion_popup(
    board: &Board,
    to: Tile,
    white_turn: bool,
    piece_atlas: &Texture2D,
) -> PromotionResult {
    let (x, y) = tile_to_screen(to, white_turn);
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE * 4.0, WHITE);

    for (i, &p) in Piece::PROMOTION_PIECES.iter().enumerate() {
        draw_texture_ex(
            piece_atlas,
            x,
            y + (TILE_SIZE * i as f32),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(crate::render::pieces::get_piece_sprite_rect(p, white_turn)),
                ..Default::default()
            },
        );
    }

    if is_mouse_button_released(MouseButton::Left) {
        let mouse = mouse_position().into();
        for (i, &piece) in Piece::PROMOTION_PIECES.iter().enumerate() {
            let rect = Rect::new(x, y + TILE_SIZE * i as f32, TILE_SIZE, TILE_SIZE);
            if rect.contains(mouse) {
                return PromotionResult::Chosen(piece);
            }
        }
        return PromotionResult::Cancelled;
    }

    PromotionResult::None
}