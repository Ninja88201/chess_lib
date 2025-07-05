use std::io::stdin;

use chess_lib::{Board, MoveList, Piece, Tile};
use macroquad::{miniquad::date::now, prelude::*};

mod input;

use crate::{consts::TILE_SIZE, render::{history::{render_recent_history}, render_board}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    Promoting(Tile),
}

pub struct Game {
    pub board: Board,
    pub flipped: bool,
    pub selected_tile: Option<Tile>,
    pub state: GameState,
    pub grace: bool,
    pub rand: macroquad::rand::RandGenerator,
    pub piece_atlas: Texture2D,
}

impl Game {
    pub async fn new() -> Self {
        let board = Board::new_from_fen("2N1N3/1N3N2/8/1N3N2/2N1N3/8/8/k6K w - - 0 1")
        // let board = Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
        let piece_atlas = load_texture("assets/PieceAtlas.png").await.unwrap();
        piece_atlas.set_filter(FilterMode::Linear);

        let rand = macroquad::rand::RandGenerator::new();
        rand.srand(now() as u64);

        Self {
            board,
            flipped: false,
            selected_tile: None,
            state: GameState::Playing,
            grace: false,
            rand,
            piece_atlas,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLACK);

            if is_key_down(KeyCode::Escape) {
                return;
            }

            self.handle_input_keys();
            render_board(&self.piece_atlas, &self.board, self.selected_tile, self.flipped);
            let fen = self.board.to_fen();
            let size = measure_text(&fen, None, 30, 1.0);
            draw_text(
                &fen, 
                0.0, 
                (TILE_SIZE * 8.0) + size.offset_y,
                30.0,
                WHITE 
            );
            render_recent_history(&self.board);

            match self.state {
                GameState::Promoting(to) => {
                    self.handle_promotion_click(to);
                }
                GameState::Playing => {
                    self.handle_input_mouse();
                }
            }

            next_frame().await;
        }
    }

    pub fn promote(&mut self, to: Tile, piece: Piece) {
        if let Some(from) = self.selected_tile {
            let _ = self.board.try_move_piece(from, to, Some(piece));
        }
        self.selected_tile = None;
        self.state = GameState::Playing;
    }

    pub fn print_fen_input(&mut self) {
        println!("Input FEN string: ");
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
        match Board::new_from_fen(&buffer.trim()) {
            Ok(b) => self.board = b,
            Err(e) => println!("{}", e),
        }
    }

    pub fn print_manual_move_input(&mut self) {
        println!("Input move: ");
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
        let mov = self.board.move_from_algebraic(&buffer.trim());
        match mov {
            Some(m) => {
                let mut moves = MoveList::new();
                self.board.generate_legal_moves(self.board.white_turn, &mut moves);
                if moves.contains(&m) {
                    self.board.make_move_unchecked(m);
                }
            }
            None => println!("Not a valid move"),
        }
    }
}
