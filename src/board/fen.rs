use std::fmt;

use crate::{Board, Move, MoveList, Piece, Tile};

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Piece placement
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

        let castling = self.castling.to_fen();

        let en_passant = match self.en_passant {
            Some(tile) => tile.to_string(),
            None => "-".to_string(),
        };

        // let halfmove_clock = 0;
        // let fullmove_number = 0;

        format!(
            "{}{} {} {} {} {}",
            fen, turn, castling, en_passant, self.half_moves, self.full_move
        )
    }
    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();

        for (i, mv) in self.history.iter().enumerate() {
            if i % 2 == 0 {
                let move_number = i / 2 + 1;
                pgn.push_str(&format!("{}. ", move_number));
            }

            let san = self.move_to_san(mv);
            pgn.push_str(&san);
            pgn.push(' ');
        }

        pgn.push('*');
        pgn
    }
    pub fn get_move_history(&self) -> String {
        let mut string = String::new();

        let pairs = self.history.chunks(2);

        for (i, pair) in pairs.enumerate() {
            string.push_str(&format!("{}. {}", i + 1, self.move_to_san(&pair[0])));

            if pair.len() > 1 {
                string.push_str(&format!(",    {}    \n", self.move_to_san(&pair[1])));
            } else {
                string.push('\n');
            }
        }

        string
    }
    pub fn move_from_algebraic(&self, s: &str) -> Option<Move> {
        let s = s.trim();

        if s.to_uppercase() == "O-O" || s == "0-0" {
            let from = if self.white_turn { Tile::E1 } else { Tile::E8 };
            let to = if self.white_turn { Tile::G1 } else { Tile::G8 };
            return Some(self.create_move(from, to, Piece::King, None, None));
        }
        if s.to_uppercase() == "O-O-O" || s == "0-0-0" {
            let from = if self.white_turn { Tile::E1 } else { Tile::E8 };
            let to = if self.white_turn { Tile::C1 } else { Tile::C8 };
            return Some(self.create_move(from, to, Piece::King, None, None));
        }

        let mut chars = s.chars().peekable();

        let piece = if let Some(&c) = chars.peek() {
            match c {
                'N' | 'B' | 'R' | 'Q' | 'K' => {
                    chars.next();
                    match c {
                        'N' => Piece::Knight,
                        'B' => Piece::Bishop,
                        'R' => Piece::Rook,
                        'Q' => Piece::Queen,
                        'K' => Piece::King,
                        _ => unreachable!(),
                    }
                }
                // Implied pawn
                _ => Piece::Pawn,
            }
        } else {
            return None;
        };

        let mut disamb_file: Option<u8> = None;
        let mut disamb_rank: Option<u8> = None;

        if piece == Piece::Pawn {
            if let Some(&c) = chars.peek() {
                if c >= 'a' && c <= 'h' {
                    let mut clone_iter = chars.clone();
                    let file_char = clone_iter.next().unwrap();
                    if let Some(&'x') = clone_iter.peek() {
                        disamb_file = Some((file_char as u8) - b'a');
                        chars.next();
                    }
                }
            }
        } else {
            let mut clone_iter = chars.clone();

            if let Some(&c1) = clone_iter.peek() {
                if c1 >= 'a' && c1 <= 'h' {
                    let file = (c1 as u8) - b'a';
                    clone_iter.next();
                    if let Some(&c2) = clone_iter.peek() {
                        if c2 < '1' || c2 > '8' {
                            disamb_file = Some(file);
                            chars.next();
                            if let Some(&c2) = chars.peek() {
                                if c2 >= '1' && c2 <= '8' {
                                    disamb_rank = Some((c2 as u8) - b'1');
                                    chars.next();
                                }
                            }
                        }
                    } else {
                        disamb_file = Some(file);
                        chars.next(); 
                    }
                } else if c1 >= '1' && c1 <= '8' {
                    disamb_rank = Some((c1 as u8) - b'1');
                    chars.next();
                }
            }
        }

        let capture_flag = if let Some(&'x') = chars.peek() {
            chars.next();
            true
        } else {
            false
        };

        let dest_file = chars.next()?;
        let dest_rank = chars.next()?;
        let to = Tile::new_chars(dest_file, dest_rank)?;

        let promotion = if let Some(&'=') = chars.peek() {
            chars.next();
            match chars.next()? {
                'N' => Some(Piece::Knight),
                'B' => Some(Piece::Bishop),
                'R' => Some(Piece::Rook),
                'Q' => Some(Piece::Queen),
                _ => return None,
            }
        } else {
            None
        };

        let (player, _) = self.get_players(self.white_turn);

        let mut matching_from: Vec<Tile> = player.bb[piece as usize]
            .iter()
            .filter(|&from_tile| {
                let mut moves = MoveList::new();
                self.generate_psuedo_moves_from(from_tile, &mut moves);
                moves.contains_move(from_tile, to)
            })
            .collect();


        if let Some(file) = disamb_file {
            matching_from.retain(|&tile| {
                let (f, _) = tile.get_coords();
                f == file
            });
        }
        if let Some(rank) = disamb_rank {
            matching_from.retain(|&tile| {
                let (_, r) = tile.get_coords();
                r == rank
            });
        }

        let from = matching_from.get(0).copied()?;

        let captured = if capture_flag {
            if let Some((p, _)) = self.get_piece_at_tile(to) {
                Some(p)
            } else if piece == Piece::Pawn && Some(to) == self.en_passant {
                Some(Piece::Pawn)
            } else {
                return None;
            }
        } else {
            None
        };

        Some(self.create_move(from, to, piece, captured, promotion))
    }
    pub fn move_to_san(&self, mov: &Move) -> String
    {
        let piece = mov.piece();
        let from = mov.from();
        let to = mov.to();
        let capture = mov.capture();
        let promo = mov.promoted_to();

        if piece == Piece::King {
            if (from == Tile::E1 && to == Tile::G1) || (from == Tile::E8 && to == Tile::G8) {
                return "O-O".to_string()
            } else if (from == Tile::E1 && to == Tile::C1)
                || (from == Tile::E8 && to == Tile::C8)
            {
                return "O-O-O".to_string()
            }
        }

        let mut s = String::new();

        if let Some(d) = self.get_disambig(mov) {
            s.push_str(&d);
        }

        if capture.is_some() {
            if piece == Piece::Pawn {
                let (from_file, _) = from.get_coords();
                s.push((b'a' + from_file as u8) as char);
            }
            s.push('x');
        }

        s.push_str(&to.to_string());

        if let Some(p) = promo {
            s.push('=');
            s.push_str(&p.to_string());
        }

        // --- Check or checkmate suffix ---
        if self.is_checkmate(self.white_turn) {
            s.push('#');
        } else if self.is_in_check(self.white_turn) {
            s.push('+');
        }

        s
    }
    pub fn get_disambig(&self, mov: &Move) -> Option<String> {
        let mut s = String::new();

        let piece = mov.piece();
        let from = mov.from();
        let to = mov.to();
        
        if piece == Piece::Pawn || piece == Piece::King {
            return None
        }

        s.push_str(&piece.to_string());

        let (_, player) = self.current_players();

        if player.bb[piece as usize].count_ones() <= 1 {
            return None
        }

        let mut targets = self.generate_attacks_from_piece(from, piece, self.white_turn, None);
        targets &= player.bb[piece as usize];

        let (from_file, from_rank) = from.get_coords();
        let mut same_rank = false;
        let mut same_file = false;
        let mut moves = MoveList::new();
        for t in player.bb[piece as usize] {
            self.generate_legal_moves_from(t, &mut moves);
            if !moves.contains_move(from, to) {
                continue;
            }
            let (file, rank) = t.get_coords();
            if file == from_file {
                same_file = true;
            }
            if rank == from_rank {
                same_rank = true;
            }
            if same_file && same_rank {
                break;
            }
        }
        if same_file {
            s.push((b'a' + from_file as u8) as char);
        }
        if same_rank {
            s.push((b'1' + from_rank as u8) as char);
        }
        Some(s)
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