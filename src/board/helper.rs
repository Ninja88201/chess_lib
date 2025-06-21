use crate::{board::{Board, Move, Piece}, tile::Tile};

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
        )
    }

}