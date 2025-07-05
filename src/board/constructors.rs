use std::cell::Cell;

use crate::{Board, CastlingRights, Piece, Player, Tile};

impl Board {
    pub fn new() -> Self {
        Self {
            white: Player::new_white(),
            black: Player::new_black(),
            castling: CastlingRights::ALL,

            white_turn: true,
            history: Vec::new(),
            repetition_history: Vec::new(),
            en_passant: None,

            half_moves: 0,
            full_move: 1,

            white_cache: Cell::new(None),
            black_cache: Cell::new(None),
        }
    }
    pub fn new_empty() -> Self {
        Self {
            white: Player::new_empty(),
            black: Player::new_empty(),
            castling: CastlingRights::NONE,

            white_turn: true,
            history: Vec::new(),
            repetition_history: Vec::new(),
            en_passant: None,

            half_moves: 0,
            full_move: 1,

            white_cache: Cell::new(None),
            black_cache: Cell::new(None),
        }
    }
    pub fn new_from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Board::new_empty();
        let mut fields = fen.split_whitespace();

        let piece_placement = fields.next().ok_or("Missing piece placement in FEN")?;
        let active_color = fields.next().ok_or("Missing active color in FEN")?;
        let castling_rights = fields.next().ok_or("Missing castling rights in FEN")?;
        let en_passant = fields.next().ok_or("Missing en passant square in FEN")?;
        let half_move = fields.next().unwrap_or("0");
        let full_move = fields.next().unwrap_or("1");

        let ranks: Vec<&str> = piece_placement.split('/').collect();
        if ranks.len() != 8 {
            return Err("Invalid FEN: expected 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file = 0;
            for ch in rank_str.chars() {
                if let Some(skip) = ch.to_digit(10) {
                    file += skip as u8;
                    continue;
                }

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

                if let Some(tile) = Tile::new_index(square) {
                    if is_white {
                        board.white.place_piece(piece, tile);
                    } else {
                        board.black.place_piece(piece, tile);
                    }
                } else {
                    return Err(format!("Invalid square index: {}", square));
                }

                file += 1;
            }

            if file != 8 {
                return Err(format!("Incomplete rank {} in FEN", 8 - rank_idx));
            }
        }

        board.white_turn = active_color == "w";
        board.castling = CastlingRights::from_fen(castling_rights);

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
            Tile::new_xy(rank_idx, file_idx)
        } else {
            None
        };

        board.half_moves = half_move
            .parse()
            .map_err(|_| format!("Invalid half move count: {}", half_move))?;
        board.full_move = full_move
            .parse()
            .map_err(|_| format!("Invalid full move count: {}", full_move))?;

        Ok(board)
    }
}
