use std::fmt;

use crate::{Board, Move, MoveList, Piece, Tile};

impl Board {
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
        fen.push(if self.white_turn { 'w' } else { 'b' });

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
    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();

        for (i, (_, san)) in self.history.iter().enumerate() {
            if i % 2 == 0 {
                let move_number = i / 2 + 1;
                pgn.push_str(&format!("{}. ", move_number));
            }
            pgn.push_str(&format!("{} ", san));
        }

        pgn
    }
    pub fn move_from_algebraic(&self, s: &str) -> Option<Move> {
        let s = s.trim();

        // Castling
        if s.eq_ignore_ascii_case("O-O") || s == "0-0" {
            let (from, to) = if self.white_turn {
                (Tile::E1, Tile::G1)
            } else {
                (Tile::E8, Tile::G8)
            };
            return Some(self.create_move(from, to, Piece::King, None, None));
        }

        if s.eq_ignore_ascii_case("O-O-O") || s == "0-0-0" {
            let (from, to) = if self.white_turn {
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

        let (player, _) = self.get_players(self.white_turn);

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
    pub fn move_to_san(&self, mov: &Move) -> String {
        let piece = mov.piece();
        let from = mov.from();
        let to = mov.to();
        let capture = mov.capture();
        let promo = mov.promoted_to();

        // Castling
        if piece == Piece::King {
            if (from == Tile::E1 && to == Tile::G1) || (from == Tile::E8 && to == Tile::G8) {
                return "O-O".to_string();
            } else if (from == Tile::E1 && to == Tile::C1) || (from == Tile::E8 && to == Tile::C8) {
                return "O-O-O".to_string();
            }
        }

        let mut s = String::new();

        // Piece character
        if let Some(c) = piece.to_san_char() {
            s.push(c);
        }

        // Disambiguation
        if let Some(d) = self.get_disambig(mov) {
            s.push_str(&d);
        }

        // Capture
        if capture.is_some() {
            if piece == Piece::Pawn {
                let (from_file, _) = from.get_coords();
                s.push((b'a' + from_file) as char);
            }
            s.push('x');
        }

        // Destination
        s.push_str(&to.to_string());

        // Promotion
        if let Some(p) = promo {
            s.push('=');
            if let Some(pc) = p.to_san_char() {
                s.push(pc);
            }
        }

        // Check / Checkmate
        if self.is_checkmate(self.white_turn) {
            s.push('#');
        } else if self.is_in_check(self.white_turn) {
            s.push('+');
        }

        s
    }
    pub fn get_disambig(&self, mov: &Move) -> Option<String> {
        let piece = mov.piece();
        if piece == Piece::Pawn || piece == Piece::King {
            return None;
        }

        let from = mov.from();
        let to = mov.to();
        let (from_file, from_rank) = from.get_coords();

        let (player, _) = self.current_players();

        let candidates: Vec<_> = player.bb[piece as usize]
            .iter()
            .filter(|&t| {
                if t == from {
                    return false;
                }
                let mut moves = MoveList::new();
                self.generate_legal_moves_from(t, &mut moves);
                moves.contains_move(t, to)
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        let mut same_file = false;
        let mut same_rank = false;

        for t in &candidates {
            let (f, r) = t.get_coords();
            if f == from_file {
                same_file = true;
            }
            if r == from_rank {
                same_rank = true;
            }
        }

        let mut result = String::new();
        if !same_file {
            result.push((b'a' + from_file as u8) as char);
        } else if !same_rank {
            result.push((b'1' + from_rank as u8) as char);
        } else {
            result.push((b'a' + from_file as u8) as char);
            result.push((b'1' + from_rank as u8) as char);
        }

        Some(result)
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