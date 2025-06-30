use crate::CastlingRights;

impl CastlingRights
{
    pub const NONE: CastlingRights = CastlingRights::new(0b0000);
    pub const WHITE_KINGSIDE: CastlingRights = CastlingRights::new(0b0001);
    pub const WHITE_QUEENSIDE: CastlingRights = CastlingRights::new(0b0010);
    pub const BLACK_KINGSIDE: CastlingRights = CastlingRights::new(0b0100);
    pub const BLACK_QUEENSIDE: CastlingRights = CastlingRights::new(0b1000);
    pub const ALL: CastlingRights = CastlingRights::new(0b1111);
}