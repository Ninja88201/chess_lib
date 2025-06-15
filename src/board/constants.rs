use crate::board::Board;

impl Board {
    pub const KNIGHT_OFFSETS: [(i8, i8); 8] = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];

    pub const KING_OFFSETS: [i8; 8] = [1, -1, 8, -8, 9, -9, 7, -7];

    
    pub const A1: u8 = 0;
    pub const A8: u8 = 56;

    pub const C1: u8 = 2;
    pub const C8: u8 = 58;
    
    pub const D1: u8 = 3;
    pub const D8: u8 = 59;

    pub const E1: u8 = 4;
    pub const E8: u8 = 60;

    pub const F1: u8 = 5;
    pub const F8: u8 = 61;

    pub const G1: u8 = 6;
    pub const G8: u8 = 62;


    pub const H1: u8 = 7;
    pub const H8: u8 = 63;
}