use crate::Bitboard;

impl Bitboard {
    // Ranks
    pub const RANK_1: Bitboard = Bitboard::new(0x00000000000000FF);
    pub const RANK_2: Bitboard = Bitboard::new(0x000000000000FF00);
    pub const RANK_3: Bitboard = Bitboard::new(0x0000000000FF0000);
    pub const RANK_4: Bitboard = Bitboard::new(0x00000000FF000000);
    pub const RANK_5: Bitboard = Bitboard::new(0x000000FF00000000);
    pub const RANK_6: Bitboard = Bitboard::new(0x0000FF0000000000);
    pub const RANK_7: Bitboard = Bitboard::new(0x00FF000000000000);
    pub const RANK_8: Bitboard = Bitboard::new(0xFF00000000000000);

    // Files
    pub const FILE_A: Bitboard = Bitboard::new(0x0101010101010101);
    pub const FILE_B: Bitboard = Bitboard::new(0x0202020202020202);
    pub const FILE_C: Bitboard = Bitboard::new(0x0404040404040404);
    pub const FILE_D: Bitboard = Bitboard::new(0x0808080808080808);
    pub const FILE_E: Bitboard = Bitboard::new(0x1010101010101010);
    pub const FILE_F: Bitboard = Bitboard::new(0x2020202020202020);
    pub const FILE_G: Bitboard = Bitboard::new(0x4040404040404040);
    pub const FILE_H: Bitboard = Bitboard::new(0x8080808080808080);
}