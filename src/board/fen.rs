use crate::{Board, Tile};

impl Board
{
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
        let castling = self.castling.to_fen();

        // En passant target
        let en_passant = match self.en_passant {
            Some(tile) => tile.to_string(),
            None => "-".to_string(),
        };

        let halfmove_clock = 0;
        let fullmove_number = 0;

        // Final FEN string
        format!(
            "{}{} {} {} {} {}",
            fen, turn, castling, en_passant, halfmove_clock, fullmove_number
        )
    }
}