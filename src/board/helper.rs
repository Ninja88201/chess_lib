use crate::{Bitboard, Board, Move, Piece, Player, Tile};

impl Board {
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
    pub fn occupied(&self) -> Bitboard {
        self.white.pieces | self.black.pieces
    }
    pub fn occupied_kingless(&self) -> Bitboard {
        self.white.attackers() | self.black.attackers()
    }
    pub fn current_players(&self) -> (&Player, &Player) {
        self.get_players(self.white_turn)
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
            (None, None) => None,
            (None, Some(p)) => Some((p, false)),
            (Some(p), None) => Some((p, true)),
            (Some(_), Some(_)) => panic!("Two pieces are overlapping"),
        }
    }
}
