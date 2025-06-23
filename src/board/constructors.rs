use crate::{Tile, Player, Piece, CastlingRights, Board};

impl Board {
    pub fn new() -> Self {
        Self {
            white: Player::new_white(),
            black: Player::new_black(),

            white_turn: true,
            history: Vec::new(),
            en_passant: None,
            
            check_cached: None,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            white: Player::new_empty(),
            black: Player::new_empty(),

            white_turn: true,
            history: Vec::new(),
            en_passant: None,

            check_cached: None,
        }
    }
    pub fn new_from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Board::new_empty();
        let mut fields = fen.split_whitespace();

        let piece_placement = fields.next().ok_or("Missing piece placement in FEN")?;
        let active_color = fields.next().unwrap_or("w");
        let castling_rights = fields.next().unwrap_or("-");
        let en_passant = fields.next().unwrap_or("-");

        if piece_placement.split('/').count() != 8 {
            return Err("Invalid FEN: expected 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in piece_placement.split('/').enumerate() {
            let mut file = 0;
            for ch in rank_str.chars() {
                match ch {
                    '1'..='8' => {
                        file += ch.to_digit(10).unwrap() as u8;
                    }
                    'p' | 'P' | 'n' | 'N' | 'b' | 'B' | 'r' | 'R' | 'q' | 'Q' | 'k' | 'K' => {
                        if file >= 8 {
                            return Err(format!("Too many pieces in rank {}", 8 - rank_idx));
                        }

                        let is_white = ch.is_uppercase();
                        let piece = match ch.to_ascii_lowercase() {
                            'p' => Piece::Pawn,
                            'n' => Piece::Knight,
                            'b' => Piece::Bishop,
                            'r' => Piece::Rook,
                            'q' => Piece::Queen,
                            'k' => Piece::King,
                            _ => return Err(format!("Invalid piece character: {}", ch)),
                        };

                        let rank = 7 - rank_idx as u8;
                        let square = rank * 8 + file;
                        if is_white {
                            board.white.place_piece(piece, Tile(square));
                        } else {
                            board.black.place_piece(piece, Tile(square));
                        }

                        file += 1;
                    }
                    _ => return Err(format!("Invalid character in FEN: {}", ch)),
                }
            }

            if file != 8 {
                return Err(format!("Incomplete rank {} in FEN", 8 - rank_idx));
            }
        }

        board.white_turn = active_color == "w";
        board.white.castling = match (castling_rights.contains('K'), castling_rights.contains('Q')) {
            (true, true) => CastlingRights::Both,
            (true, false) => CastlingRights::KingSide,
            (false, true) => CastlingRights::QueenSide,
            (false, false) => CastlingRights::None,
        };
        board.black.castling = match (castling_rights.contains('k'), castling_rights.contains('q')) {
            (true, true) => CastlingRights::Both,
            (true, false) => CastlingRights::KingSide,
            (false, true) => CastlingRights::QueenSide,
            (false, false) => CastlingRights::None,
        };
        board.en_passant = if en_passant != "-" {
            let bytes = en_passant.as_bytes();
            if bytes.len() != 2 {
                return Err(format!("Invalid en passant square: {}", en_passant));
            }
            let file = bytes[0];
            let rank = bytes[1];
            if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
                return Err(format!("Invalid en passant square: {}", en_passant));
            }
            let file_idx = file - b'a';
            let rank_idx = rank - b'1';
            Some(Tile(rank_idx * 8 + file_idx))
        } else {
            None
        };
        Ok(board)
    }
}