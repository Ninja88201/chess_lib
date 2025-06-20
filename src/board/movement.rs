use crate::bitboard::Bitboard;
use crate::board::{Board, CastlingRights, Move, MoveError, Piece};
use crate::player::Player;
use crate::tile::Tile;

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for y in (0..8).rev() {
            let mut empty = 0;

            for x in 0..8 {
                let tile = Tile::new_xy(x, y).unwrap();

                match self.get_piece_at_tile(tile) {
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

            if y != 0 {
                fen.push('/');
            }
        }

        let turn = if self.white_turn { " w" } else { " b" };
        format!("{}{}", fen, turn)
    }

    pub fn occupied(&self) -> Bitboard {
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
    pub fn get_piece_at_tile(&self, tile: Tile) -> Option<(Piece, bool)> {
        let white_piece = self.white.get_piece(tile);
        let black_piece = self.black.get_piece(tile);
        match (white_piece, black_piece) {
            (None, None) => return None,
            (None, Some(p)) => return Some((p, false)),
            (Some(p), None) => return Some((p, true)),
            (Some(_), Some(_)) => panic!("Two pieces are overlapping"),
        }
    }
    pub async fn try_move_piece<C, F: AsyncFn(Tile, C) -> Option<Piece>>
    (&mut self, from: Tile, to: Tile, promotion: F, context: C) -> Result<(), MoveError> {
        if from == to {
            return Err(MoveError::SameTile)
        }
        let result = self.get_piece_at_tile(from);
        if let Some((p, w)) = result {
            let mut promote = None;
            if to.is_promotion(w) && p == Piece::Pawn && from.get_neighbours().get_bit(to){
                let promotion = promotion(to, context);
                match promotion.await {
                    Some(p) => {
                        promote = Some(p);           
                    },
                    None => return Err(MoveError::Cancelled),
                }
            }
            let mut capture: Option<Piece> = None;
            capture = match self.get_piece_at_tile(to) {
                Some((p, w)) => {
                    if w == self.white_turn {
                        return Err(MoveError::FriendlyCapture);
                    }
                    Some(p)
                },
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
            
            let mov = Move::new(
                self.white_turn, 
                from, 
                to, 
                p, 
                capture,
                self.en_passant, 
                self.white.castling, 
                self.black.castling ,
                promote
            );
            let result = self.make_move_unchecked(mov);

            if self.is_in_check(!self.white_turn) {
                self.undo_move();
                return Err(MoveError::PiecePinned);
            }
            if self.generate_legal_moves(self.white_turn).len() == 0 && !self.is_in_check(self.white_turn){
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
        let legal_moves = self.generate_moves_from(mov.from);
        
        
        
        
        if !legal_moves.contains(&mov) {
            return Err(MoveError::IllegalMove);
        }
        if legal_moves.len() == 0 && !self.is_in_check(self.white_turn) {
            println!("Stalemate");
            return Err(MoveError::Stalemate);
        }
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
                if let Some(t) = mov.en_passant && t == mov.to {
                    opponent.remove_piece_type(p, t.backward(is_white).unwrap());
                }
                else {
                    opponent.remove_piece_type(p, mov.to);
                }
            }
            // return Err(MoveError::IllegalMove);
        };

        

        player.move_piece(mov.from, mov.to);
        self.en_passant = None;


        if let Some(p) = mov.promoted_to {
            player.remove_piece(mov.to);
            player.place_piece(p, mov.to);
        }

        if mov.piece == Piece::King {
            player.castling = CastlingRights::None;
            match (mov.white_turn, mov.from, mov.to) {
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
        if mov.piece == Piece::Rook && player.castling != CastlingRights::None {
            if mov.from == Board::A1 || mov.from == Board::A8 {
                player.castling = match player.castling {
                    CastlingRights::None => CastlingRights::None,
                    CastlingRights::KingSide => CastlingRights::KingSide,
                    CastlingRights::QueenSide => CastlingRights::None,
                    CastlingRights::Both => CastlingRights::KingSide,
                }
            }
            if mov.from ==  Board::H1 || mov.from == Board::H8 {
                player.castling = match player.castling {
                    CastlingRights::None => CastlingRights::None,
                    CastlingRights::KingSide => CastlingRights::None,
                    CastlingRights::QueenSide => CastlingRights::QueenSide,
                    CastlingRights::Both => CastlingRights::QueenSide,
                }
            }
        }
        if mov.piece == Piece::Pawn && !mov.from.get_neighbours().get_bit(mov.to) {
            self.en_passant = Some(mov.to.backward(self.white_turn).unwrap());
        }
        self.history.push(mov);

        self.white_turn = !self.white_turn;
        Ok(())
    }
    pub fn undo_move(&mut self) {
        let white = self.white_turn;
        if let Some(last_move) = self.history.pop() {
            let (player, opponent) = match last_move.white_turn {
                true => (&mut self.white, &mut self.black),
                false => (&mut self.black, &mut self.white),
            };
            if let Some(_) = last_move.promoted_to {
                player.remove_piece(last_move.to);
                player.place_piece(Piece::Pawn, last_move.to);
            }
            // Move piece back
            player.move_piece(last_move.to, last_move.from);
            
            // Restore captured piece if any
            if let Some(captured) = last_move.capture {
                if let Some(passant) = last_move.en_passant {
                    if last_move.to == passant {
                        opponent.place_piece(Piece::Pawn, passant.forward(white).unwrap());
                    }
                    else {
                        opponent.place_piece(captured, last_move.to);
                    }
                }
                else {
                    opponent.place_piece(captured, last_move.to);
                }
            }

            // Handle castling reversal
            if last_move.piece == Piece::King {
                match (last_move.white_turn, last_move.from, last_move.to) {
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
            self.white.castling = last_move.prev_white_castle;
            self.black.castling = last_move.prev_black_castle;
            self.en_passant = last_move.en_passant;

            self.white_turn = !self.white_turn;
        }
    }
}