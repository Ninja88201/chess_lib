use crate::{Board, CastlingRights, Piece, Tile};

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

    // Cached data
    pub check_cached: Option<bool>
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
        check_cached: Option<bool>,
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
            check_cached
        }
    }
}
impl Default for Move
{
    fn default() -> Self {
        Self { 
            white_turn: true, 
            from: Board::A1, 
            to: Board::A1, 
            piece: Piece::Pawn, 
            capture: None, 
            prev_white_castle: CastlingRights::Both, 
            prev_black_castle: CastlingRights::Both, 
            promoted_to: None, 
            en_passant: None, 
            check_cached: None }
    }
}