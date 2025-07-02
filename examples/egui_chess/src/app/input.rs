use chess_lib::{Board, MoveList, Tile};
use egui::{Context, InputState, Key, Pos2, Response};
use rand::Rng;

use crate::app::{helper::UIState, ChessApp};

impl ChessApp
{
    pub fn move_input(&mut self, response: Response, origin: Pos2) {
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let (x, y) = self.screen_to_tile(pos, origin);
                let player = self.board.current_players().0;

                if let Some(t) = Tile::new_xy(x as u8, y as u8) {
                    if player.pieces.get_bit(t) {
                        self.selected = Some(t);
                    }
                    else {
                        if let Some(s) = self.selected {
                            let result = self.board.try_move_piece(
                                s, 
                                t, 
                                None,
                            );
                            match result {
                                Ok(p) => {
                                    if let Some(to) = p {
                                        // Handle promotion popup & recall try_move_piece with the 
                                        // selected promotion piece
                                        self.ui_state = UIState::Promotion(to);
                                    }
                                    else {
                                        self.selected = None;
                                    }
                                },
                                Err(e) => {
                                    use chess_lib::MoveError as me;
                                    match e {
                                        me::NoPieceSelected => println!("a"),
                                        // Unselect Tile
                                        me::SameTile => self.selected = None,
                                        me::FriendlyCapture => unreachable!(),
                                        me::IllegalMove => println!("That move is illegal"),
                                        me::WrongTurn => unreachable!(),
                                        me::PiecePinned => (), // Flash king square red,
                                        me::Stalemate => println!("Stalemate"),
                                        me::Checkmate => println!("You are in checkmate"),
                                        me::Cancelled => (),
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn utility_input(&mut self, input: &InputState, _: &Context) {
        if input.key_pressed(Key::R) {
            self.board = Board::new();
        }
        if input.key_pressed(Key::F) {
            self.flipped = !self.flipped;
        }
        if input.modifiers.ctrl && input.key_pressed(Key::Z) {
            self.board.undo_move();
        }
        if input.key_pressed(Key::Space) {
            let mut moves = MoveList::new();
            self.board.generate_legal_moves(self.board.white_turn, &mut moves);
            if !moves.is_empty() {
                let random_index = self.rand.random_range(0..moves.len());
                self.board.make_move_unchecked(moves[random_index]);
            }
        }
        if input.key_pressed(Key::Escape) {
            self.should_close = true;
        }
    }
}