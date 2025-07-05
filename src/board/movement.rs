use crate::{MoveResult, Board, CastlingRights, Move, MoveError, MoveList, Piece, Tile};

impl Board {
    pub fn try_move_piece(
        &mut self,
        from: Tile,
        to: Tile,
        promotion: Option<Piece>
    ) -> Result<MoveResult, MoveError> {
        if let Some(p) = promotion {
            let mov = self.create_move(
                from, 
                to, 
                self.get_piece_at_tile(from).unwrap().0, 
                self.get_piece_at_tile(to).map(|(p, _)| p), 
                Some(p)
            );
            self.make_move_unchecked(mov);
            return Ok(MoveResult::MoveApplied(self.get_state()))
        }
        if from == to {
            return Err(MoveError::SameTile);
        }
        
        if self.is_checkmate(self.white_turn) {
            return Err(MoveError::Checkmate);
        }
        if self.is_stalemate(self.white_turn) {
            return Err(MoveError::Stalemate);
        }
        
        let result = self.get_piece_at_tile(from);
        if let Some((p, w)) = result {
            if w != self.white_turn {
                return Err(MoveError::WrongTurn);
            }
            
            let capture = match self.get_piece_at_tile(to) {
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

            let mut promote = promotion;
            if p == Piece::Pawn && to.is_promotion(self.white_turn) && promotion.is_none() {
                promote = Some(Piece::Queen);
            }
            let mov = self.create_move(from, to, p, capture, promote);

            let mut legal = MoveList::new();
            self.generate_legal_moves_from(from, &mut legal);

            if !legal.iter().any(|m| *m == mov) {
                return Err(MoveError::IllegalMove);
            }

            if promote == Some(Piece::Queen) && promotion.is_none() {
                return Ok(MoveResult::PromotionNeeded(to))
            }
            self.make_move_unchecked(mov);

            return Ok(MoveResult::MoveApplied(self.get_state()));
        } else {
            return Err(MoveError::NoPieceSelected);
        }
    }
    pub fn make_move_unchecked(&mut self, mov: Move) {
        let san = self.move_to_san(&mov);
        let (player, opponent) = if self.white_turn {
            (&mut self.white, &mut self.black)
        } else {
            self.full_move += 1;
            (&mut self.black, &mut self.white)
        };
        self.half_moves += 1;

        if mov.piece() == Piece::Pawn
        {
            self.half_moves = 0;
        }
        if let Some(p) = mov.capture() {
            self.half_moves = 0;
            let target_tile = if mov.en_passant() == Some(mov.to()) {
                mov.to().backward(self.white_turn).unwrap()
            } else {
                mov.to()
            };
            opponent.remove_piece_type(p, target_tile);

            if p == Piece::Rook {
                let rights = match mov.to() {
                    Tile::A1 => CastlingRights::WHITE_QUEENSIDE,
                    Tile::H1 => CastlingRights::WHITE_KINGSIDE,

                    Tile::A8 => CastlingRights::BLACK_QUEENSIDE,
                    Tile::H8 => CastlingRights::BLACK_KINGSIDE,
                    _ => CastlingRights::NONE,
                };
                self.castling.remove(rights);
            }
        }

        player.move_piece(mov.from(), mov.to());
        self.en_passant = None;

        if let Some(p) = mov.promoted_to() {
            player.remove_piece(mov.to());
            player.place_piece(p, mov.to());
        }

        if mov.piece() == Piece::King {
            let rights = match self.white_turn {
                true => CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE,
                false => CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE,
            };
            self.castling.remove(rights);
            match (self.white_turn, mov.from(), mov.to()) {
                (true, Tile::E1, Tile::G1) => {
                    player.move_piece(Tile::H1, Tile::F1);
                }
                (true, Tile::E1, Tile::C1) => {
                    player.move_piece(Tile::A1, Tile::D1);
                }
                (false, Tile::E8, Tile::G8) => {
                    player.move_piece(Tile::H8, Tile::F8);
                }
                (false, Tile::E8, Tile::C8) => {
                    player.move_piece(Tile::A8, Tile::D8);
                }
                _ => {}
            }
        }
        if mov.piece() == Piece::Rook {
            let rights = match mov.from() {
                Tile::A1 => CastlingRights::WHITE_QUEENSIDE,
                Tile::H1 => CastlingRights::WHITE_KINGSIDE,

                Tile::A8 => CastlingRights::BLACK_QUEENSIDE,
                Tile::H8 => CastlingRights::BLACK_KINGSIDE,
                _ => CastlingRights::NONE,
            };
            self.castling.remove(rights);
        }
        if mov.piece() == Piece::Pawn
            && (mov.from().get_coords().0 == mov.to().get_coords().0)
            && (i8::abs(mov.from().get_coords().1 as i8 - mov.to().get_coords().1 as i8) == 2)
        {
            self.en_passant = Some(mov.to().backward(self.white_turn).unwrap());
        }
        self.history.push((mov, san));

        self.white_turn = !self.white_turn;
        self.white_cache.set(None);
        self.black_cache.set(None);
        self.repetition_history.push(self.to_zobrist_hash());
    }
    pub fn undo_move(&mut self) {
        if let Some((last_move, _)) = self.history.pop() {
            let (player, opponent) = match !self.white_turn {
                true => (&mut self.white, &mut self.black),
                false => {
                    self.full_move -= 1;
                    (&mut self.black, &mut self.white)
                },
            };
            if last_move.promoted_to().is_some() {
                player.remove_piece(last_move.to());
                player.place_piece(Piece::Pawn, last_move.to());
            }
            player.move_piece(last_move.to(), last_move.from());

            if let Some(captured) = last_move.capture() {
                if last_move.piece() == Piece::Pawn
                    && last_move.en_passant() == Some(last_move.to())
                {
                    opponent.place_piece(
                        Piece::Pawn,
                        last_move.to().backward(!self.white_turn).unwrap(),
                    );
                } else {
                    opponent.place_piece(captured, last_move.to());
                }
            }

            if last_move.piece() == Piece::King {
                match (!self.white_turn, last_move.from(), last_move.to()) {
                    (true, Tile::E1, Tile::G1) => {
                        player.move_piece(Tile::F1, Tile::H1);
                    }
                    (true, Tile::E1, Tile::C1) => {
                        player.move_piece(Tile::D1, Tile::A1);
                    }
                    (false, Tile::E8, Tile::G8) => {
                        player.move_piece(Tile::F8, Tile::H8);
                    }
                    (false, Tile::E8, Tile::C8) => {
                        player.move_piece(Tile::D8, Tile::A8);
                    }
                    _ => {}
                }
            }
            self.castling = last_move.prev_castle();
            self.en_passant = last_move.en_passant();
            self.white_cache.set(last_move.white_cache());
            self.black_cache.set(last_move.black_cache());
            self.half_moves = last_move.prev_half_moves();
            self.repetition_history.pop();

            self.white_turn = !self.white_turn;
        }
    }
}
