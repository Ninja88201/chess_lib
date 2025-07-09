use crate::{Bitboard, Board, Colour, Move, Piece, Player, Tile};

impl Board {
    /// Creates an internal move using the current board position to add context
    /// i.e. the current castling rights & en passant tiles
    pub fn create_move(
        &self,
        from: Tile,
        to: Tile,
        piece: Piece,
        captured: Option<Piece>,
        promotion: Option<Piece>,
    ) -> Move {
        Move::new(
            from,
            to,
            piece,
            captured,
            self.en_passant,
            self.castling,
            promotion,
            self.white_cache.get(),
            self.black_cache.get(),
            self.half_moves,
        )
    }

    /// Returns a bitboard representing all tiles with a piece on it
    pub fn occupied(&self) -> Bitboard {
        self.white.pieces | self.black.pieces
    }

    pub fn current_players(&self) -> (&Player, &Player) {
        self.get_players(self.turn)
    }

    pub fn get_players(&self, colour: Colour) -> (&Player, &Player) {
        match colour {
            Colour::White => (&self.white, &self.black),
            Colour::Black => (&self.black, &self.white),
        }
    }

    pub fn get_players_mut(&mut self, colour: Colour) -> (&mut Player, &mut Player) {
        match colour {
            Colour::White => (&mut self.white, &mut self.black),
            Colour::Black => (&mut self.black, &mut self.white),
        }
    }

    /// Returns whether or not a piece is at a given tile & it's colour
    pub fn get_piece_at_tile(&self, tile: Tile) -> Option<(Piece, Colour)> {
        let white_piece = self.white.get_piece(tile);
        let black_piece = self.black.get_piece(tile);
        match (white_piece, black_piece) {
            (None, None) => None,
            (None, Some(p)) => Some((p, Colour::Black)),
            (Some(p), None) => Some((p, Colour::White)),
            (Some(_), Some(_)) => panic!("Two pieces are overlapping"),
        }
    }
}
