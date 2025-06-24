use crate::{Board, CastlingRights, Move, MoveError, MoveList, Piece, Tile};

impl Board {
    pub async fn try_move_piece<C, F: AsyncFn(Tile, C) -> Option<Piece>>(
        &mut self,
        from: Tile,
        to: Tile,
        promotion: F,
        context: C
    ) -> Result<(), MoveError> {
        if from == to {
            return Err(MoveError::SameTile);
        }

        if self.is_checkmate(self.white_turn) {
            return Err(MoveError::Checkmate);
        }

        let result = self.get_piece_at_tile(from);
        if let Some((p, w)) = result {
            if w != self.white_turn {
                return Err(MoveError::WrongTurn);
            }

            let mut promote = None;
            if to.is_promotion(w) && p == Piece::Pawn && from.get_neighbours().get_bit(to) {
                let promotion = promotion(to, context);
                match promotion.await {
                    Some(p) => promote = Some(p),
                    None => return Err(MoveError::Cancelled),
                }
            }

            let mut capture = match self.get_piece_at_tile(to) {
                Some((p, w)) => {
                    if w == self.white_turn {
                        return Err(MoveError::FriendlyCapture);
                    }
                    Some(p)
                }
                None => {
                    if let Some(ep) = self.en_passant {
                        if to == ep && p == Piece::Pawn {
                            Some(Piece::Pawn)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            };

            let mov = self.create_move(from, to, p, capture, promote);

            let mut legal = MoveList::new();
            self.generate_legal_moves_from(from, &mut legal);

            if !legal.iter().any(|m| *m == mov) {
                return Err(MoveError::IllegalMove);
            }

            let result = self.make_move_unchecked(mov);

            if self.is_in_check(!self.white_turn) {
                self.undo_move();
                return Err(MoveError::PiecePinned);
            }

            // Stalemate
            let mut remaining = MoveList::new();
            self.generate_legal_moves(self.white_turn, &mut remaining);
            if remaining.is_empty() && !self.is_in_check(self.white_turn) {
                println!("Stalemate");
            }

            if self.is_checkmate(self.white_turn) {
                println!("Checkmate");
            }

            return result;
        } else {
            return Err(MoveError::NoPieceSelected);
        }
    }
    pub fn make_move_unchecked(&mut self, mov: Move) -> Result<(), MoveError> {
        if mov.from == mov.to {
            return Err(MoveError::SameTile);
        }
        
        let piece_moved = self.get_piece_at_tile(mov.from);

        let (player, opponent) = if self.white_turn {
            (&mut self.white, &mut self.black)
        } else {
            (&mut self.black, &mut self.white)
        };

        if let Some((_, is_white)) = piece_moved {
            if is_white != self.white_turn {
                return Err(MoveError::WrongTurn);
            }
            if let Some(p) = mov.capture {
                let target_tile = if mov.en_passant == Some(mov.to) {
                    mov.to.backward(is_white).unwrap()
                } else {
                    mov.to
                };
                opponent.remove_piece_type(p, target_tile);

                if p == Piece::Rook {
                    let rights = match mov.to {
                        Board::A1 => CastlingRights::WHITE_QUEENSIDE,
                        Board::H1 => CastlingRights::WHITE_KINGSIDE,

                        Board::A8 => CastlingRights::BLACK_QUEENSIDE,
                        Board::H8 => CastlingRights::BLACK_KINGSIDE,
                        _ => CastlingRights::NONE,
                    };
                    self.castling.remove(rights);
                }
            }
        }
        else {
            return Err(MoveError::NoPieceSelected);
        }

        

        player.move_piece(mov.from, mov.to);
        self.en_passant = None;


        if let Some(p) = mov.promoted_to {
            player.remove_piece(mov.to);
            player.place_piece(p, mov.to);
        }

        if mov.piece == Piece::King {
            let rights = match self.white_turn {
                true => CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE,
                false => CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE,
            };
            self.castling.remove(rights);
            match (self.white_turn, mov.from, mov.to) {
                (true, Board::E1, Board::G1) => {
                    player.move_piece(Board::H1, Board::F1);
                }
                (true, Board::E1, Board::C1) => {
                    player.move_piece(Board::A1, Board::D1);
                }
                (false, Board::E8, Board::G8) => {
                    player.move_piece(Board::H8, Board::F8);
                }
                (false, Board::E8, Board::C8) => {
                    player.move_piece(Board::A8, Board::D8);
                }
                _ => {}
            }
        }
        if mov.piece == Piece::Rook {
            let rights = match mov.from {
                Board::A1 => CastlingRights::WHITE_QUEENSIDE,
                Board::H1 => CastlingRights::WHITE_KINGSIDE,

                Board::A8 => CastlingRights::BLACK_QUEENSIDE,
                Board::H8 => CastlingRights::BLACK_KINGSIDE,
                _ => CastlingRights::NONE,
            };
            self.castling.remove(rights);
        }
        if mov.piece == Piece::Pawn && (mov.from.get_coords().0 == mov.to.get_coords().0) && 
        (i8::abs(mov.from.get_coords().1 as i8 - mov.to.get_coords().1 as i8) == 2) {
            self.en_passant = Some(mov.to.backward(self.white_turn).unwrap());
        }
        self.history.push(mov);

        self.white_turn = !self.white_turn;
        self.white_cache = None;
        self.black_cache = None;

        Ok(())
    }
    pub fn undo_move(&mut self) {
        if let Some(last_move) = self.history.pop() {
            let (player, opponent) = match !self.white_turn {
                true => (&mut self.white, &mut self.black),
                false => (&mut self.black, &mut self.white),
            };
            if let Some(_) = last_move.promoted_to {
                player.remove_piece(last_move.to);
                player.place_piece(Piece::Pawn, last_move.to);
            }
            player.move_piece(last_move.to, last_move.from);
            
            if let Some(captured) = last_move.capture {
                if last_move.piece == Piece::Pawn && last_move.en_passant == Some(last_move.to) {
                    opponent.place_piece(Piece::Pawn, last_move.to.backward(!self.white_turn).unwrap());
                } else {
                    opponent.place_piece(captured, last_move.to);
                }
            }

            if last_move.piece == Piece::King {
                match (!self.white_turn, last_move.from, last_move.to) {
                    (true, Board::E1, Board::G1) => {
                        player.move_piece(Board::F1, Board::H1);
                    }
                    (true, Board::E1, Board::C1) => {
                        player.move_piece(Board::D1, Board::A1);
                    }
                    (false, Board::E8, Board::G8) => {
                        player.move_piece(Board::F8, Board::H8);
                    }
                    (false, Board::E8, Board::C8) => {
                        player.move_piece(Board::D8, Board::A8);
                    }
                    _ => {}
                }
            }
            self.castling = last_move.prev_castle;
            self.en_passant = last_move.en_passant;
            self.white_cache = last_move.white_cache;
            self.black_cache = last_move.black_cache;

            self.white_turn = !self.white_turn;

        }
    }
}