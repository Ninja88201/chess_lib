use std::fmt;

use crate::{Bitboard, Board, Colour, Disambig, GameState, Move, Piece, SanMove, Tile};

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
        if s.is_empty() {
            return None
        }

        let s = s.trim().trim_end_matches(['+', '#', '?', '!']);
        let len = s.len();
        let is_capture = s.contains('x');
        let (player, _) = self.get_players(self.turn);

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

        // Promotion
        if s.contains('=') {
            let first = s.chars().nth(0)?;
            let last = s.chars().last()?;
            let promotion = Piece::from_san(last);
            let dest = Tile::from_str(&s[len-4..len-2])?;


            let source = if !is_capture {
                (Bitboard::from_tile(dest.backward(self.turn)?) & player.bb[Piece::Pawn as usize]).to_bit()?
            } else {
                (dest.pawn_attacks(!self.turn) & 
                    player.bb[Piece::Pawn as usize] & 
                    Bitboard::from_tile(Tile::new_chars(first, if self.turn == Colour::White {'7'} else {'2'})?)).to_bit()?
            };

            return Some(self.create_move(
                source, 
                dest, 
                Piece::Pawn, 
                self.get_piece_at_tile(dest).map(|(p, _)| p), 
                Some(promotion)
            ))
        }

        match len {
            2 => {
                // Pawn move
                // e4, d4
                let dest = Tile::from_str(s)?; 
                let mut bb = Bitboard::EMPTY;
                if let Some(t1) = dest.backward(self.turn) {
                    bb.set_bit(t1, true);
                    if let Some(t2) = t1.backward(self.turn) {
                        bb.set_bit(t2, true);
                    }
                }
                bb &= player.bb[Piece::Pawn as usize];
                Some(self.create_move(
                    bb.to_bit().unwrap(), 
                    dest,
                    Piece::Pawn, 
                    None, 
                    None
                ))
            },
            3 => {
                // Normal move
                // Nc3, Bc4
                let first = s.chars().nth(0)?;
                let piece = Piece::from_san(first);
                let dest = Tile::from_str(&s[1..])?;

                Some(self.create_move(
                    (player.bb[piece as usize] & self.generate_attacks_from_piece(dest, piece, !self.turn, None)).to_bit().unwrap(), 
                    dest, 
                    piece, 
                    None, 
                    None
                ))
            },
            4 => {
                // Captures or Single disambiguation
                // exd4, Bxd4, Nfd4
                let first = s.chars().nth(0)?;
                let piece = Piece::from_san(first);
                let dest = Tile::from_str(&s[len-2..])?;
                let from = if is_capture {
                    let mut bb = player.bb[piece as usize] & 
                        self.generate_attacks_from_piece(dest, piece, !self.turn, None);

                    if piece == Piece::Pawn {
                        bb &= Bitboard::file_from_char(first)
                    }
                    bb.to_bit()?
                } else {
                    let piece = Piece::from_san(first);
                    (player.bb[piece as usize] & 
                        self.generate_attacks_from_piece(dest, piece, !self.turn, None) &
                        Bitboard::file_from_char(s.chars().nth(1)?))
                        .to_bit()?
                };
                Some(self.create_move(
                    from, 
                    dest, 
                    piece, 
                    self.get_piece_at_tile(dest).map(|(p, _)| p), 
                    None
                ))
            },
            5 => {
                // Double disambiguation, Single disambiguation capture
                // Nd2e4, Nfxe4

                let first = s.chars().nth(0)?;
                let piece = Piece::from_san(first);
                let dest = Tile::from_str(&s[len-2..])?;
                let from = if is_capture {
                    (player.bb[piece as usize] & 
                        self.generate_attacks_from_piece(dest, piece, !self.turn, None) &
                        Bitboard::file_from_char(s.chars().nth(1)?))
                        .to_bit()?
                } else {
                    Tile::from_str(&s[1..3])?
                };
                Some(self.create_move(
                    from, 
                    dest, 
                    piece, 
                    self.get_piece_at_tile(dest).map(|(p, _)| p), 
                    None
                ))
            },
            6 => {
                // Double disambiguation capture
                // Nd2xe4

                let first = s.chars().nth(0)?;
                let piece = Piece::from_san(first);
                let dest = Tile::from_str(&s[len-2..])?;
                let from = Tile::from_str(&s[1..3])?;
                Some(self.create_move(
                    from, 
                    dest, 
                    piece, 
                    self.get_piece_at_tile(dest).map(|(p, _)| p), 
                    None
                ))
            }
            _ => None,
        }
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
    pub fn to_ascii(&self) -> String {
        let mut output = "  +------------------------+\n".to_string();
        for y in (0..8).rev() {
            output.push_str(&format!("{} |", y + 1));
            for x in 0..8 {
                let tile = Tile::new_xy(x, y).unwrap();
                match self.get_piece_at_tile(tile) {
                    Some((piece, is_white)) => {
                        output.push_str(&format!(" {} ", piece.to_fen_char(is_white)));
                    }
                    None => {
                        output.push_str(" . ");
                    }
                }
            }
            output.push_str("|");
        }
        output.push_str("  +------------------------+\n");
        output.push_str("    a  b  c  d  e  f  g  h\n");
        output
    }
    pub fn to_unicode(&self) -> String {
        let mut output = "  +------------------------+\n".to_string();
        for y in (0..8).rev() {
            output.push_str(&format!("{} |", y + 1));
            for x in 0..8 {
                let tile = Tile::new_xy(x, y).unwrap();
                match self.get_piece_at_tile(tile) {
                    Some((piece, is_white)) => {
                        output.push_str(&format!(" {} ", piece.to_unicode(is_white)));
                    }
                    None => {
                        output.push_str(" . ");
                    }
                }
            }
            output.push_str("|\n");
        }
        output.push_str("  +------------------------+\n");
        output.push_str("    a  b  c  d  e  f  g  h\n");
        output
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