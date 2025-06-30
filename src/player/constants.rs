use crate::{Bitboard, Player};

impl Player
{
    /// Standard starting bitboards for white pieces
    pub const WHITE_PAWNS: Bitboard  = Bitboard::new(0x0000_0000_0000_FF00);
    pub const WHITE_KNIGHTS: Bitboard = Bitboard::new(0x0000_0000_0000_0042);
    pub const WHITE_BISHOPS: Bitboard = Bitboard::new(0x0000_0000_0000_0024);
    pub const WHITE_ROOKS: Bitboard   = Bitboard::new(0x0000_0000_0000_0081);
    pub const WHITE_QUEEN: Bitboard   = Bitboard::new(0x0000_0000_0000_0008);
    pub const WHITE_KING: Bitboard    = Bitboard::new(0x0000_0000_0000_0010);

    pub const WHITE_PIECES: Bitboard = Bitboard::new(
        0x0000_0000_0000_FF00 |
        0x0000_0000_0000_0042 |
        0x0000_0000_0000_0024 |
        0x0000_0000_0000_0081 |
        0x0000_0000_0000_0008 |
        0x0000_0000_0000_0010
    );
    
    /// Standard starting bitboards for black pieces
    pub const BLACK_PAWNS: Bitboard   = Bitboard::new(0x00FF_0000_0000_0000);
    pub const BLACK_KNIGHTS: Bitboard = Bitboard::new(0x4200_0000_0000_0000);
    pub const BLACK_BISHOPS: Bitboard = Bitboard::new(0x2400_0000_0000_0000);
    pub const BLACK_ROOKS: Bitboard   = Bitboard::new(0x8100_0000_0000_0000);
    pub const BLACK_QUEEN: Bitboard   = Bitboard::new(0x0800_0000_0000_0000);
    pub const BLACK_KING: Bitboard    = Bitboard::new(0x1000_0000_0000_0000);

    pub const BLACK_PIECES: Bitboard = Bitboard::new(
        0x00FF_0000_0000_0000 |
        0x4200_0000_0000_0000 |
        0x2400_0000_0000_0000 |
        0x8100_0000_0000_0000 |
        0x0800_0000_0000_0000 |
        0x1000_0000_0000_0000
    );
}