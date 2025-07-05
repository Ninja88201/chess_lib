use macroquad::prelude::*;

use crate::game::{Game, GameState};
use crate::render::pieces::get_piece_sprite_rect;
use crate::utils::{get_tile, tile_to_screen};
use chess_lib::{Piece, Tile};
use crate::TILE_SIZE;

impl Game {
    pub fn handle_input_keys(&mut self) {
        if is_key_pressed(KeyCode::F) {
            self.flipped = !self.flipped;
        }
    
        if is_key_pressed(KeyCode::R) {
            self.board = chess_lib::Board::new();
        }
    
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
            self.board.undo_move();
            self.selected_tile = None;
            self.state = GameState::Playing;
        }
    
        if is_key_pressed(KeyCode::U) {
            self.print_fen_input();
        }
    
        if is_key_pressed(KeyCode::I) {
            self.print_manual_move_input();
        }
    
        if is_key_pressed(KeyCode::P) {
            println!("{}", self.board.to_fen());
        }
    
        if is_key_pressed(KeyCode::Space) {
            let mut moves = chess_lib::MoveList::new();
            self.board
                .generate_legal_moves(self.board.white_turn, &mut moves);
    
            if !moves.is_empty() {
                let random_index = self.rand.gen_range(0, moves.len());
                self.board.make_move_unchecked(moves[random_index]);
            }
        }
    }

    pub fn handle_input_mouse(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(clicked_tile) = get_tile(mouse_position().into(), self.flipped) {
                match self.selected_tile {
                    Some(from) => {
                        match self.board.try_move_piece(from, clicked_tile, None) {
                            Ok(result) => {
                                match result {
                                    chess_lib::MoveResult::MoveApplied(_) => {           
                                    },
                                    chess_lib::MoveResult::PromotionNeeded(tile) => {
                                        self.grace = true;
                                        self.state = GameState::Promoting(tile)
                                    },
                                }
                                self.selected_tile = None;
                            }
                            Err(e) => {
                                use chess_lib::move_enums::MoveError::*;
                                match e {
                                    IllegalMove => println!("Illegal move"),
                                    WrongTurn | NoPieceSelected | SameTile => self.selected_tile = None,
                                    FriendlyCapture => self.selected_tile = Some(clicked_tile),
                                    PiecePinned => println!("That piece is pinned"),
                                    Stalemate => println!("Stalemate"),
                                    Checkmate => println!("Checkmate"),
                                    Cancelled => println!("Move cancelled"),
                                }
                            }
                        }
                    }
                    None => {
                        if self.board.current_players().0.pieces.get_bit(clicked_tile) {
                            self.selected_tile = Some(clicked_tile);
                        } else {
                            self.selected_tile = None;
                        }
                    }
                }
            }
        }
    }
    pub fn handle_promotion_click(&mut self, to: Tile) {
        let (x, y) = tile_to_screen(to, self.flipped);
        draw_rectangle(x, y, TILE_SIZE, TILE_SIZE * 4.0, WHITE);
    
        for (i, &p) in Piece::PROMOTION_PIECES.iter().enumerate() {
            draw_texture_ex(
                &self.piece_atlas,
                x,
                y + (TILE_SIZE * i as f32),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    source: Some(get_piece_sprite_rect(p, self.board.white_turn)),
                    ..Default::default()
                },
            );
        }
    
        if is_mouse_button_released(MouseButton::Left) && !self.grace {
            let options = [
                (0, Piece::Queen),
                (1, Piece::Rook),
                (2, Piece::Bishop),
                (3, Piece::Knight),
            ];
    
            let mouse_pos: Vec2 = mouse_position().into();
    
            for (i, piece) in options {
                let rect = Rect::new(x, y + TILE_SIZE * i as f32, TILE_SIZE, TILE_SIZE);
                if rect.contains(mouse_pos) {
                    self.promote(to, piece);
                    break;
                }
            }
            self.selected_tile = None;
            self.state = GameState::Playing;
        }
    
        if is_mouse_button_released(MouseButton::Left) {
            self.grace = false;
        }
    }
}


