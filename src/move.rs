use crate::{CastlingRights, Piece, Tile};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub white_turn: bool,

    // Movement
    pub from: Tile,
    pub to: Tile,

    // Piece types
    pub piece: Piece,
    pub capture: Option<Piece>,
    
    // Castling Rights
    pub prev_white_castle: CastlingRights,
    pub prev_black_castle: CastlingRights,
    
    // Special cases
    pub promoted_to: Option<Piece>,
    pub en_passant: Option<Tile>,
}
impl Move {
    pub fn new(
        white_turn: bool,
        from: Tile,
        to: Tile,
        piece: Piece,
        capture: Option<Piece>,
        en_passant: Option<Tile>,
        prev_white_castle: CastlingRights,
        prev_black_castle: CastlingRights,
        promoted_to: Option<Piece>,
    ) -> Self {
        Self {
            white_turn,
            from,
            to,
            piece,
            capture,
            en_passant,
            prev_white_castle,
            prev_black_castle,
            promoted_to,
        }
    }
}
