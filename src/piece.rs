use crate::Colour;

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
        Piece::Knight,
        Piece::Rook,
        Piece::Bishop,
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
    pub fn from_san(san: char) -> Self {
        match san {
            'N' => Piece::Knight,
            'B' => Piece::Bishop,
            'R' => Piece::Rook,
            'Q' => Piece::Queen,
            'K' => Piece::King,
            _ => Piece::Pawn,
        }
    }
    pub fn to_zobrist_index(&self, colour: Colour) -> usize {
        match (self, colour) {
            (Piece::Pawn, Colour::White) => 0,
            (Piece::Knight, Colour::White) => 1,
            (Piece::Bishop, Colour::White) => 2,
            (Piece::Rook, Colour::White) => 3,
            (Piece::Queen, Colour::White) => 4,
            (Piece::King, Colour::White) => 5,

            (Piece::Pawn, Colour::Black) => 6,
            (Piece::Knight, Colour::Black) => 7,
            (Piece::Bishop, Colour::Black) => 8,
            (Piece::Rook, Colour::Black) => 9,
            (Piece::Queen, Colour::Black) => 10,
            (Piece::King, Colour::Black) => 11,
        }
    }
    pub fn to_fen_char(&self, colour: Colour) -> char {
        let c = match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };
        if colour.white() { c.to_ascii_uppercase() } else { c }
    }
    pub fn to_san_char(&self) -> char {
        match self {
            Piece::Pawn => '\0',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }
    pub fn to_unicode(&self, colour: Colour) -> char {
        match (self, colour) {
            (Piece::King, Colour::White) => '♔',
            (Piece::Queen, Colour::White) => '♕',
            (Piece::Rook, Colour::White) => '♖',
            (Piece::Bishop, Colour::White) => '♗',
            (Piece::Knight, Colour::White) => '♘',
            (Piece::Pawn, Colour::White) => '♙',
            
            (Piece::King, Colour::Black) => '♚',
            (Piece::Queen, Colour::Black) => '♛',
            (Piece::Rook, Colour::Black) => '♜',
            (Piece::Bishop, Colour::Black) => '♝',
            (Piece::Knight, Colour::Black) => '♞',
            (Piece::Pawn, Colour::Black) => '♟',
        }
    }
}

impl From<Piece> for usize {
    fn from(value: Piece) -> Self {
        value as usize
    }
}
