#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}
impl Piece {
    pub const ALL_PIECES: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
    pub const PROMOTION_PIECES: [Piece; 4] = [
        Piece::Queen,
        Piece::Rook,
        Piece::Bishop,
        Piece::Knight
    ];
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece index"),
        }
    }
    pub fn to_fen_char(&self, white: bool) -> char {
        let c = match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };
        if white { c.to_ascii_uppercase() } else { c }
    }
    pub fn to_unicode(&self, is_white: bool) -> char {
        match (self, is_white) {
            (Piece::King, true) => '♔',
            (Piece::Queen, true) => '♕',
            (Piece::Rook, true) => '♖',
            (Piece::Bishop, true) => '♗',
            (Piece::Knight, true) => '♘',
            (Piece::Pawn, true) => '♙',
            (Piece::King, false) => '♚',
            (Piece::Queen, false) => '♛',
            (Piece::Rook, false) => '♜',
            (Piece::Bishop, false) => '♝',
            (Piece::Knight, false) => '♞',
            (Piece::Pawn, false) => '♟',
        }
    }
}
impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Piece::Pawn => "",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
        };
        write!(f, "{}", c)
    }
}

impl From<Piece> for usize {
    fn from(value: Piece) -> Self {
        value as usize
    }
}
