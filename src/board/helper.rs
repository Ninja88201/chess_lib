use crate::{castling_rights, Bitboard, Board, Move, Piece, Player, Tile};

impl Board
{
    pub fn create_move(
        &self,
        from: Tile,
        to: Tile,
        piece: Piece,
        captured: Option<Piece>,
        promotion: Option<Piece>,
    ) -> Move {
        Move::new(
            self.white_turn,
            from,
            to,
            piece,
            captured,
            self.en_passant,
            self.white.castling,
            self.black.castling,
            promotion,
            self.check_cached
        )
    }
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

        // Active color
        let turn = if self.white_turn { " w" } else { " b" };

        // Castling rights
        use castling_rights::CastlingRights as cr;
        let castling = match (self.white.castling, self.black.castling) {
            (cr::None, cr::None) => "-",
            (cr::None, cr::KingSide) => "k",
            (cr::None, cr::QueenSide) => "q",
            (cr::None, cr::Both) => "kq",
            (cr::KingSide, cr::None) => "K",
            (cr::KingSide, cr::KingSide) => "Kk",
            (cr::KingSide, cr::QueenSide) => "Kq",
            (cr::KingSide, cr::Both) => "Kkq",
            (cr::QueenSide, cr::None) => "Q",
            (cr::QueenSide, cr::KingSide) => "Qk",
            (cr::QueenSide, cr::QueenSide) => "Qq",
            (cr::QueenSide, cr::Both) => "Qkq",
            (cr::Both, cr::None) => "KQ",
            (cr::Both, cr::KingSide) => "KQk",
            (cr::Both, cr::QueenSide) => "KQq",
            (cr::Both, cr::Both) => "KQkq",
        };

        // En passant target
        let en_passant = match self.en_passant {
            Some(tile) => tile.to_string(),
            None => "-".to_string(),
        };

        let halfmove_clock = 0;
        let fullmove_number = self.history.iter().filter(|m| !m.white_turn == true).count() / 2 + 1;

        // Final FEN string
        format!(
            "{}{} {} {} {} {}",
            fen, turn, castling, en_passant, halfmove_clock, fullmove_number
        )
    }
    pub fn occupied(&self) -> Bitboard {
        self.white.pieces() | self.black.pieces()
    }
    #[inline(always)]
    pub fn get_players(&self, white: bool) -> (&Player, &Player) {
        if white {
            (&self.white, &self.black)
        } else {
            (&self.black, &self.white)
        }
    }
    pub fn get_players_mut(&mut self, white: bool) -> (&mut Player, &mut Player) {
        match white {
            true => (&mut self.white, &mut self.black),
            false => (&mut self.black, &mut self.white),
        }
    }

}