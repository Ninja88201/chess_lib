use crate::{board::Board, tile::Tile};

impl Board {
    pub const KNIGHT_OFFSETS: [(i8, i8); 8] = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];
    
    pub const A1: Tile = Tile(0);
    pub const A8: Tile = Tile(56);

    pub const B1: Tile = Tile(1);
    pub const B8: Tile = Tile(55);

    pub const C1: Tile = Tile(2);
    pub const C8: Tile = Tile(58);
    
    pub const D1: Tile = Tile(3);
    pub const D8: Tile = Tile(59);

    pub const E1: Tile = Tile(4);
    pub const E8: Tile = Tile(60);

    pub const F1: Tile = Tile(5);
    pub const F8: Tile = Tile(61);

    pub const G1: Tile = Tile(6);
    pub const G8: Tile = Tile(62);

    pub const H1: Tile = Tile(7);
    pub const H8: Tile = Tile(63);
}