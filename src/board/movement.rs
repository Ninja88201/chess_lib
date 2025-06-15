use crate::board::{Board, Piece, Move};
use crate::player::Player;

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let square = rank * 8 + file;

                match self.get_piece_at_square(square) {
                    Some((piece, is_white)) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(piece.to_fen_char(is_white));
                    }
                    None => {
                        empty += 1;
                    }
                }
            }

            if empty > 0 {
                fen.push_str(&empty.to_string());
            }

            if rank != 0 {
                fen.push('/');
            }
        }

        let turn = if self.white_turn { " w" } else { " b" };
        format!("{}{}", fen, turn)
    }

    pub fn occupied(&self) -> u64 {
        self.white.pieces() | self.black.pieces()
    }
    pub fn get_players(&self, white: bool) -> (&Player, &Player) {
        match white {
            true => (&self.white, &self.black),
            false => (&self.black, &self.white),
        }
    }
    pub fn get_players_mut(&mut self, white: bool) -> (&mut Player, &mut Player) {
        match white {
            true => (&mut self.white, &mut self.black),
            false => (&mut self.black, &mut self.white),
        }
    }
    pub fn get_piece_at_square(&self, square: u8) -> Option<(Piece, bool)> {
        let white_piece = self.white.get_piece(square);
        let black_piece = self.black.get_piece(square);
        match (white_piece, black_piece) {
            (None, None) => return None,
            (None, Some(p)) => return Some((p, false)),
            (Some(p), None) => return Some((p, true)),
            (Some(_), Some(_)) => panic!("Two pieces are overlapping"),
        }
    }
    pub fn try_move_piece(&mut self, from: u8, to: u8) -> bool {
        // Can't move to same square
        if from == to {
            return false;
        }

        let piece_moved = self.get_piece_at_square(from);
        let target_piece = self.get_piece_at_square(to);
        let legal_moves = self.generate_moves_from(from);
        // let legal_moves = self.generate_legal_moves(from);

        // If no piece at source or not player's piece, fail
        let Some((moved_piece, is_white)) = piece_moved else {
            return false;
        };
        if is_white != self.white_turn {
            return false;
        }

        // Move not Pseudo-legal
        if (legal_moves & (1u64 << to)) == 0 {
            return false;
        }

        // Snapshot castling rights
        let prev_short_castle = (self.white.short_castle, self.black.short_castle);
        let prev_long_castle = (self.white.long_castle, self.black.long_castle);

        let (player, opponent) = if self.white_turn {
            (&mut self.white, &mut self.black)
        } else {
            (&mut self.black, &mut self.white)
        };

        let capture = if let Some((captured_piece, captured_is_white)) = target_piece {
            if captured_is_white == self.white_turn {
                return false;
            }
            opponent.remove_piece_type(captured_piece, to)
        } else {
            None
        };

        player.remove_piece_type(moved_piece, from);
        player.place_piece(moved_piece, to);

        if moved_piece == Piece::King {
            player.short_castle = false;
            player.long_castle = false;

            match (is_white, from, to) {
                (true, Board::E1, Board::G1) => {
                    player.remove_piece_type(Piece::Rook, Board::H1);
                    player.place_piece(Piece::Rook, Board::F1);
                }
                (true, Board::E1, Board::C1) => {
                    player.remove_piece_type(Piece::Rook, Board::A1);
                    player.place_piece(Piece::Rook, Board::D1);
                }
                (false, Board::E8, Board::G8) => {
                    player.remove_piece_type(Piece::Rook, Board::H8);
                    player.place_piece(Piece::Rook, Board::F8);
                }
                (false, Board::E8, Board::C8) => {
                    player.remove_piece_type(Piece::Rook, Board::A8);
                    player.place_piece(Piece::Rook, Board::D8);
                }
                _ => {}
            }
        }

        // Save move to history
        self.history.push(Move::new(
            self.white_turn,
            from,
            to,
            moved_piece,
            capture,
            prev_short_castle,
            prev_long_castle,
            false,
            None,
        ));
        
        self.white_turn = !self.white_turn;

        if self.is_in_check(!self.white_turn) {
            self.undo_move();
            return false;
        }

        // Optional: print if checkmate
        if self.is_checkmate(self.white_turn) {
            println!("Checkmate");
        }

        true
    }
    pub fn make_move_unchecked(&mut self, from: u8, to: u8) -> bool {
        // Can't move to same square
        if from == to {
            return false;
        }

        let piece_moved = self.get_piece_at_square(from);
        let target_piece = self.get_piece_at_square(to);
        let legal_moves = self.generate_moves_from(from);
        // let legal_moves = self.generate_legal_moves(from);

        // If no piece at source or not player's piece, fail
        let Some((moved_piece, is_white)) = piece_moved else {
            return false;
        };
        if is_white != self.white_turn {
            return false;
        }

        // Move not Pseudo-legal
        if (legal_moves & (1u64 << to)) == 0 {
            return false;
        }

        // Snapshot castling rights
        let prev_short_castle = (self.white.short_castle, self.black.short_castle);
        let prev_long_castle = (self.white.long_castle, self.black.long_castle);

        let (player, opponent) = if self.white_turn {
            (&mut self.white, &mut self.black)
        } else {
            (&mut self.black, &mut self.white)
        };

        let capture = if let Some((captured_piece, captured_is_white)) = target_piece {
            if captured_is_white == self.white_turn {
                return false;
            }
            opponent.remove_piece_type(captured_piece, to)
        } else {
            None
        };

        player.remove_piece_type(moved_piece, from);
        player.place_piece(moved_piece, to);

        if moved_piece == Piece::King {
            player.short_castle = false;
            player.long_castle = false;

            match (is_white, from, to) {
                (true, Board::E1, Board::G1) => {
                    player.remove_piece_type(Piece::Rook, Board::H1);
                    player.place_piece(Piece::Rook, Board::F1);
                }
                (true, Board::E1, Board::C1) => {
                    player.remove_piece_type(Piece::Rook, Board::A1);
                    player.place_piece(Piece::Rook, Board::D1);
                }
                (false, Board::E8, Board::G8) => {
                    player.remove_piece_type(Piece::Rook, Board::H8);
                    player.place_piece(Piece::Rook, Board::F8);
                }
                (false, Board::E8, Board::C8) => {
                    player.remove_piece_type(Piece::Rook, Board::A8);
                    player.place_piece(Piece::Rook, Board::D8);
                }
                _ => {}
            }
        }

        // Save move to history
        self.history.push(Move::new(
            self.white_turn,
            from,
            to,
            moved_piece,
            capture,
            prev_short_castle,
            prev_long_castle,
            false,
            None,
        ));
        
        self.white_turn = !self.white_turn;

        true
    }
    pub fn undo_move(&mut self) {
        if let Some(last_move) = self.history.pop() {
            let (player, opponent) = self.get_players_mut(last_move.white_turn);

            // Move piece back
            player.remove_piece_type(last_move.piece, last_move.to);
            player.place_piece(last_move.piece, last_move.from);

            // Restore captured piece if any
            if let Some(captured) = last_move.capture {
                opponent.place_piece(captured, last_move.to);
            }

            // Handle castling reversal
            if last_move.piece == Piece::King {
                match (last_move.from, last_move.to) {
                    (4, 6) => {
                        // White kingside
                        player.remove_piece_type(Piece::Rook, 5);
                        player.place_piece(Piece::Rook, 7);
                    }
                    (4, 2) => {
                        // White queenside
                        player.remove_piece_type(Piece::Rook, 3);
                        player.place_piece(Piece::Rook, 0);
                    }
                    (60, 62) => {
                        // Black kingside
                        player.remove_piece_type(Piece::Rook, 61);
                        player.place_piece(Piece::Rook, 63);
                    }
                    (60, 58) => {
                        // Black queenside
                        player.remove_piece_type(Piece::Rook, 59);
                        player.place_piece(Piece::Rook, 56);
                    }
                    _ => {}
                }
            }

            self.white.short_castle = last_move.prev_short_castle.0;
            self.black.short_castle = last_move.prev_short_castle.1;
            self.white.long_castle = last_move.prev_long_castle.0;
            self.black.long_castle = last_move.prev_long_castle.1;
            self.white_turn = !self.white_turn;
        }
    }
}