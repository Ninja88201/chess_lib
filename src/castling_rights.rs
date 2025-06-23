#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CastlingRights {
    None,
    KingSide,
    QueenSide,
    Both,
}
impl CastlingRights
{
    pub fn to_fen(&self, white: bool) -> String {
        let result = match self {
            CastlingRights::None => "",
            CastlingRights::KingSide => "k",
            CastlingRights::QueenSide => "q",
            CastlingRights::Both => "kq",
        };
        if white {
            result.to_uppercase()
        } else {
            result.to_string()
        }
    }
    pub fn short_castle(&self) -> bool {
        match self {
            CastlingRights::None => false,
            CastlingRights::KingSide => true,
            CastlingRights::QueenSide => false,
            CastlingRights::Both => true,
        }
    }
    pub fn long_castle(&self) -> bool {
        match self {
            CastlingRights::None => false,
            CastlingRights::KingSide => false,
            CastlingRights::QueenSide => true,
            CastlingRights::Both => true,
        }
    }
}