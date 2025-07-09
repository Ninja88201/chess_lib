use crate::{zobrist::consts::{CASTLING, EN_PASSANT, PIECE_SQUARE, SIDE_TO_MOVE}, Board, CastlingRights, Colour, History, Move, MoveError, MoveList, MoveResult, Piece, Tile};

impl Board {
    /// Attempts to create & make an internal move given the starting & ending tile
    /// returning whether the move is applied or needs further attention to select
    /// the promotion piece when necessary
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
        
        if self.is_in_checkmate(self.turn) {
            return Err(MoveError::Checkmate);
        }
        if self.is_stalemate(self.turn) {
            return Err(MoveError::Stalemate);
        }
        
        let result = self.get_piece_at_tile(from);
        if let Some((p, w)) = result {
            if w != self.turn {
                return Err(MoveError::WrongTurn);
            }
            
            let capture = match self.get_piece_at_tile(to) {
                Some((p, w)) => {
                    if w == self.turn {
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
            if p == Piece::Pawn && to.is_promotion(self.turn) && promotion.is_none() {
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

    /// Attempts to make a move not checking for llegality
    pub fn make_move_unchecked(&mut self, mov: Move) {
        let hash = self.zobrist_hash;
        let mut san = self.move_to_san(&mov);
        // let mut san = "".to_string();

        // Toggle side to move
        self.zobrist_hash ^= SIDE_TO_MOVE;

        let (player, opponent) = if self.turn.white() {
            (&mut self.white, &mut self.black)
        } else {
            self.full_move += 1;
            (&mut self.black, &mut self.white)
        };

        self.half_moves += 1;

        // Remove en passant hash if present
        if let Some(tile) = self.en_passant {
            let (file, _) = tile.get_coords();
            self.zobrist_hash ^= EN_PASSANT[file as usize];
        }
        self.en_passant = None;

        // Handle pawn move or capture (reset half-move clock)
        if mov.piece() == Piece::Pawn {
            self.half_moves = 0;
        }
        if let Some(p) = mov.capture() {
            self.half_moves = 0;

            let target_tile = if mov.en_passant() == Some(mov.to()) {
                mov.to().backward(self.turn).unwrap()
            } else {
                mov.to()
            };

            opponent.remove_piece_type(p, target_tile);
            self.zobrist_hash ^= PIECE_SQUARE[p.to_zobrist_index(!self.turn)][target_tile.to_usize()];

            // Remove opponent's castling rights if rook is captured
            if p == Piece::Rook {
                let rights = match mov.to() {
                    Tile::A1 => CastlingRights::WHITE_QUEENSIDE,
                    Tile::H1 => CastlingRights::WHITE_KINGSIDE,
                    Tile::A8 => CastlingRights::BLACK_QUEENSIDE,
                    Tile::H8 => CastlingRights::BLACK_KINGSIDE,
                    _ => CastlingRights::NONE,
                };
                if self.castling.contains(rights) {
                    self.zobrist_hash ^= CASTLING[rights.single_index()];
                    self.castling.remove(rights);
                }
            }
        }

        // Zobrist: remove piece from 'from'
        self.zobrist_hash ^= PIECE_SQUARE[mov.piece().to_zobrist_index(self.turn)][mov.from().to_usize()];
        // Move piece
        player.move_piece(mov.from(), mov.to());
        // Zobrist: add piece to 'to'
        self.zobrist_hash ^= PIECE_SQUARE[mov.piece().to_zobrist_index(self.turn)][mov.to().to_usize()];

        // Handle promotion
        if let Some(promo) = mov.promoted_to() {
            player.remove_piece(mov.to());
            self.zobrist_hash ^= PIECE_SQUARE[mov.piece().to_zobrist_index(self.turn)][mov.to().to_usize()];

            player.place_piece(promo, mov.to());
            self.zobrist_hash ^= PIECE_SQUARE[promo.to_zobrist_index(self.turn)][mov.to().to_usize()];
        }

        // Handle king move + castling
        if mov.piece() == Piece::King {
            let affected_rights = match self.turn {
                Colour::White => CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE,
                Colour::Black => CastlingRights::BLACK_KINGSIDE | CastlingRights::BLACK_QUEENSIDE,
            };

            for right in CastlingRights::ALL_RIGHTS {
                if affected_rights.contains(right) && self.castling.contains(right) {
                    self.zobrist_hash ^= CASTLING[right.single_index()];
                    self.castling.remove(right);
                }
            }

            match (self.turn, mov.from(), mov.to()) {
                (Colour::White, Tile::E1, Tile::G1) => {
                    player.move_piece(Tile::H1, Tile::F1);
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::H1.to_usize()];
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::F1.to_usize()];
                }
                (Colour::White, Tile::E1, Tile::C1) => {
                    player.move_piece(Tile::A1, Tile::D1);
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::A1.to_usize()];
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::D1.to_usize()];
                }
                (Colour::Black, Tile::E8, Tile::G8) => {
                    player.move_piece(Tile::H8, Tile::F8);
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::H8.to_usize()];
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::F8.to_usize()];
                }
                (Colour::Black, Tile::E8, Tile::C8) => {
                    player.move_piece(Tile::A8, Tile::D8);
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::A8.to_usize()];
                    self.zobrist_hash ^= PIECE_SQUARE[Piece::Rook.to_zobrist_index(self.turn)][Tile::D8.to_usize()];
                }
                _ => {}
            }
        }

        // Rook moved — remove castling rights
        if mov.piece() == Piece::Rook {
            let rights = match mov.from() {
                Tile::A1 => CastlingRights::WHITE_QUEENSIDE,
                Tile::H1 => CastlingRights::WHITE_KINGSIDE,
                Tile::A8 => CastlingRights::BLACK_QUEENSIDE,
                Tile::H8 => CastlingRights::BLACK_KINGSIDE,
                _ => CastlingRights::NONE,
            };
            if self.castling.contains(rights) {
                self.zobrist_hash ^= CASTLING[rights.single_index()];
                self.castling.remove(rights);
            }
        }

        // Handle en passant square creation
        if mov.piece() == Piece::Pawn
            && mov.from().get_coords().0 == mov.to().get_coords().0
            && (i8::abs(mov.from().get_coords().1 as i8 - mov.to().get_coords().1 as i8) == 2)
        {
            let ep_tile = mov.to().backward(self.turn).unwrap();
            let (file, _) = ep_tile.get_coords();
            self.en_passant = Some(ep_tile);
            self.zobrist_hash ^= EN_PASSANT[file as usize];
        }

        // Switch sides
        self.turn = !self.turn;

        self.white_cache.set(None);
        self.black_cache.set(None);

        // Update repetition history
        self.repetition_history.push(self.zobrist_hash);

        // Final SAN formatting
        if self.is_in_checkmate(self.turn) {
            san.set_mate(true);
        } else if self.is_in_check(self.turn) {
            san.set_check(true);
        }
        
        // Save to history (with original Zobrist hash)
        self.history.push(History::new(mov, hash, san));
    }
    pub fn undo_move(&mut self) {
        if let Some(h) = self.history.pop() {
            let last_move = h.last_move;
            let (player, opponent) = match !self.turn {
                Colour::White => (&mut self.white, &mut self.black),
                Colour::Black => {
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
                        last_move.to().backward(!self.turn).unwrap(),
                    );
                } else {
                    opponent.place_piece(captured, last_move.to());
                }
            }

            if last_move.piece() == Piece::King {
                match (!self.turn, last_move.from(), last_move.to()) {
                    (Colour::White, Tile::E1, Tile::G1) => {
                        player.move_piece(Tile::F1, Tile::H1);
                    }
                    (Colour::White, Tile::E1, Tile::C1) => {
                        player.move_piece(Tile::D1, Tile::A1);
                    }
                    (Colour::Black, Tile::E8, Tile::G8) => {
                        player.move_piece(Tile::F8, Tile::H8);
                    }
                    (Colour::Black, Tile::E8, Tile::C8) => {
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

            self.zobrist_hash = h.last_zobrist;

            self.turn = !self.turn;
        }
    }
}
