use std::fmt;

use crate::{Bitboard, Board, Disambig, GameState, Move, MoveList, Piece, SanMove, Tile};

impl Board {
    /// Converts the current board position into Forsyth–Edwards Notation
    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(100);

        // Piece placement
        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let tile = Tile::new_xy(file, rank)
                    .expect("Invalid tile index in to_fen (this should never happen)");

                match self.get_piece_at_tile(tile) {
                    Some((piece, is_white)) => {
                        if empty > 0 {
                            fen.push((b'0' + empty as u8) as char);
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
                fen.push((b'0' + empty as u8) as char);
            }

            if rank != 0 {
                fen.push('/');
            }
        }

        // Active color
        fen.push(' ');
        fen.push(if self.turn.white() { 'w' } else { 'b' });

        // Castling rights
        let castling = self.castling.to_fen();
        fen.push(' ');
        fen.push_str(&castling);

        // En passant
        fen.push(' ');
        fen.push_str(
            &self
                .en_passant
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-".to_string()),
        );

        // Halfmove and fullmove
        use std::fmt::Write;
        write!(fen, " {} {}", self.half_moves, self.full_move).unwrap();

        fen
    }

    /// Converts the board into Portable Game Notation
    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();

        for (i, h) in self.history.iter().enumerate() {
            if i % 2 == 0 {
                let move_number = i / 2 + 1;
                pgn.push_str(&format!("{}. ", move_number));
            }
            pgn.push_str(&format!("{} ", h.san_string));
        }

        let state = self.get_state();
        if let GameState::Checkmate(c) = state {
            if c.white() {
                pgn.push_str("0-1");
            } else {
                pgn.push_str("1-0");
            }
        } else if matches!(state, GameState::Stalemate(_)) 
            || state == GameState::FiftyMoveRule 
            || state == GameState::InsufficientMaterial 
            || state == GameState::ThreeRepetition {
                pgn.push_str("1/2-1/2");
        } else {
            pgn.push('*');
        }
        pgn
    }

    /// Attempts to create an internal move from a string in Standard Algebraic Notation ( SAN )
    pub fn move_from_algebraic(&self, s: &str) -> Option<Move> {
        let s = s.trim();

        // Castling
        if s.eq_ignore_ascii_case("O-O") || s == "0-0" {
            let (from, to) = if self.turn.white() {
                (Tile::E1, Tile::G1)
            } else {
                (Tile::E8, Tile::G8)
            };
            return Some(self.create_move(from, to, Piece::King, None, None));
        }

        if s.eq_ignore_ascii_case("O-O-O") || s == "0-0-0" {
            let (from, to) = if self.turn.white() {
                (Tile::E1, Tile::C1)
            } else {
                (Tile::E8, Tile::C8)
            };
            return Some(self.create_move(from, to, Piece::King, None, None));
        }

        let mut chars = s.chars().peekable();

        // Piece type
        let piece = match chars.peek() {
            Some('N') => { chars.next(); Piece::Knight }
            Some('B') => { chars.next(); Piece::Bishop }
            Some('R') => { chars.next(); Piece::Rook }
            Some('Q') => { chars.next(); Piece::Queen }
            Some('K') => { chars.next(); Piece::King }
            _ => Piece::Pawn,
        };

        let mut disamb_file = None;
        let mut disamb_rank = None;

        // Disambiguation
        while let Some(&c) = chars.peek() {
            if c == 'x' {
                break;
            } else if c.is_ascii_digit() {
                disamb_rank = Some(c as u8 - b'1');
                chars.next();
            } else if c.is_ascii_alphabetic() {
                disamb_file = Some(c as u8 - b'a');
                chars.next();
            } else {
                break;
            }
        }

        let is_capture = matches!(chars.peek(), Some('x'));
        if is_capture {
            chars.next();
        }

        // Destination
        let dest_file = chars.next()? as u8 - b'a';
        let dest_rank = chars.next()? as u8 - b'1';
        let to = Tile::new_xy(dest_file, dest_rank)?;

        // Promotion
        let promotion = if chars.peek() == Some(&'=') {
            chars.next();
            match chars.next()? {
                'Q' => Some(Piece::Queen),
                'R' => Some(Piece::Rook),
                'B' => Some(Piece::Bishop),
                'N' => Some(Piece::Knight),
                _ => return None,
            }
        } else {
            None
        };

        let (player, _) = self.get_players(self.turn);

        // Filter matching pieces that can move to destination
        let mut candidates: Vec<Tile> = player.bb[piece as usize]
            .iter()
            .filter(|&from_tile| {
                let mut moves = MoveList::new();
                self.generate_psuedo_moves_from(from_tile, &mut moves);
                moves.contains_move(from_tile, to)
            })
            .collect();

        // Apply disambiguation
        if let Some(f) = disamb_file {
            candidates.retain(|&t| t.get_coords().0 == f);
        }
        if let Some(r) = disamb_rank {
            candidates.retain(|&t| t.get_coords().1 == r);
        }

        // Ambiguity check
        if candidates.len() != 1 {
            return None;
        }

        let from = candidates[0];

        // Determine captured piece
        let captured = if is_capture {
            self.get_piece_at_tile(to)
                .map(|(p, _)| p)
                .or_else(|| {
                    if piece == Piece::Pawn && Some(to) == self.en_passant {
                        Some(Piece::Pawn)
                    } else {
                        None
                    }
                })
        } else {
            None
        };

        Some(self.create_move(from, to, piece, captured, promotion))
    }

    /// Converts a move into Standard Algebraic Notation using the current board
    /// position as context for disambiguation
    pub fn move_to_san(&self, mov: &Move) -> SanMove {
        let piece = mov.piece();
        let from = mov.from();
        let to = mov.to();
        let capture = mov.capture().is_some();
        let promo = mov.promoted_to();

        let (is_kingside_castle, is_queenside_castle) = match (from, to) {
            (Tile::E1, Tile::G1) | (Tile::E8, Tile::G8) => (true, false),
            (Tile::E1, Tile::C1) | (Tile::E8, Tile::C8) => (false, true),
            _ => (false, false),
        };

        let disambig = self.get_disambig(mov);

        SanMove::new(
            piece, 
            disambig, 
            capture, 
            to, 
            promo, 
            is_kingside_castle, 
            is_queenside_castle, 
            false, 
            false
        )
    }

    /// Returns a string representing the optional characters needed
    /// to differentiate which piece is being moved
    fn get_disambig(&self, mov: &Move) -> Option<Disambig> {
        let piece = mov.piece();
        let from = mov.from();
        let to = mov.to();
        let (from_file, from_rank) = from.get_coords();

        // Skip disambig for kings (can never have ambiguous destination)
        if piece == Piece::King {
            return None;
        }
        
        if piece == Piece::Pawn {
            if mov.capture().is_none() {
                return None;
            } else {
                return Some(Disambig::File(from_file))
            }
        }
        let (player, _) = self.current_players();
        let pinned = self.get_pinned_mask(self.turn);

        // Find other same-piece attackers (excluding the current one)
        let attackers = self.get_attackers_to(to, piece, self.turn)
            & player.bb[piece as usize]
            & !pinned
            & !Bitboard::from_tile(from);

        if attackers.is_empty() {
            return None;
        }

        // For pawns, only disambiguate in the case of a capture

        // Disambiguation logic for other pieces (knights, rooks, etc.)
        let mut same_file = false;
        let mut same_rank = false;

        for t in attackers {
            let (f, r) = t.get_coords();
            if f == from_file {
                same_file = true;
            }
            if r == from_rank {
                same_rank = true;
            }
        }

        Some(if !same_file {
            Disambig::File(from_file)
        } else if !same_rank {
            Disambig::Rank(from_rank)
        } else {
            Disambig::FileRank(from)
        })
    }

}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  +------------------------+")?;
        for y in (0..8).rev() {
            write!(f, "{} |", y + 1)?;
            for x in 0..8 {
                let tile = Tile::new_xy(x, y).unwrap();
                match self.get_piece_at_tile(tile) {
                    Some((piece, is_white)) => {
                        write!(f, " {} ", piece.to_fen_char(is_white))?;
                    }
                    None => {
                        write!(f, " . ")?;
                    }
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "  +------------------------+")?;
        writeln!(f, "    a  b  c  d  e  f  g  h")
    }
}