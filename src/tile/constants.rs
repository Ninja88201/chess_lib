use crate::Tile;

impl Tile {
    pub const A1: Tile = Tile::new_unchecked(0);
    pub const B1: Tile = Tile::new_unchecked(1);
    pub const C1: Tile = Tile::new_unchecked(2);
    pub const D1: Tile = Tile::new_unchecked(3);
    pub const E1: Tile = Tile::new_unchecked(4);
    pub const F1: Tile = Tile::new_unchecked(5);
    pub const G1: Tile = Tile::new_unchecked(6);
    pub const H1: Tile = Tile::new_unchecked(7);

    pub const A2: Tile = Tile::new_unchecked(8);
    pub const B2: Tile = Tile::new_unchecked(9);
    pub const C2: Tile = Tile::new_unchecked(10);
    pub const D2: Tile = Tile::new_unchecked(11);
    pub const E2: Tile = Tile::new_unchecked(12);
    pub const F2: Tile = Tile::new_unchecked(13);
    pub const G2: Tile = Tile::new_unchecked(14);
    pub const H2: Tile = Tile::new_unchecked(15);

    pub const A3: Tile = Tile::new_unchecked(16);
    pub const B3: Tile = Tile::new_unchecked(17);
    pub const C3: Tile = Tile::new_unchecked(18);
    pub const D3: Tile = Tile::new_unchecked(19);
    pub const E3: Tile = Tile::new_unchecked(20);
    pub const F3: Tile = Tile::new_unchecked(21);
    pub const G3: Tile = Tile::new_unchecked(22);
    pub const H3: Tile = Tile::new_unchecked(23);

    pub const A4: Tile = Tile::new_unchecked(24);
    pub const B4: Tile = Tile::new_unchecked(25);
    pub const C4: Tile = Tile::new_unchecked(26);
    pub const D4: Tile = Tile::new_unchecked(27);
    pub const E4: Tile = Tile::new_unchecked(28);
    pub const F4: Tile = Tile::new_unchecked(29);
    pub const G4: Tile = Tile::new_unchecked(30);
    pub const H4: Tile = Tile::new_unchecked(31);

    pub const A5: Tile = Tile::new_unchecked(32);
    pub const B5: Tile = Tile::new_unchecked(33);
    pub const C5: Tile = Tile::new_unchecked(34);
    pub const D5: Tile = Tile::new_unchecked(35);
    pub const E5: Tile = Tile::new_unchecked(36);
    pub const F5: Tile = Tile::new_unchecked(37);
    pub const G5: Tile = Tile::new_unchecked(38);
    pub const H5: Tile = Tile::new_unchecked(39);

    pub const A6: Tile = Tile::new_unchecked(40);
    pub const B6: Tile = Tile::new_unchecked(41);
    pub const C6: Tile = Tile::new_unchecked(42);
    pub const D6: Tile = Tile::new_unchecked(43);
    pub const E6: Tile = Tile::new_unchecked(44);
    pub const F6: Tile = Tile::new_unchecked(45);
    pub const G6: Tile = Tile::new_unchecked(46);
    pub const H6: Tile = Tile::new_unchecked(47);

    pub const A7: Tile = Tile::new_unchecked(48);
    pub const B7: Tile = Tile::new_unchecked(49);
    pub const C7: Tile = Tile::new_unchecked(50);
    pub const D7: Tile = Tile::new_unchecked(51);
    pub const E7: Tile = Tile::new_unchecked(52);
    pub const F7: Tile = Tile::new_unchecked(53);
    pub const G7: Tile = Tile::new_unchecked(54);
    pub const H7: Tile = Tile::new_unchecked(55);

    pub const A8: Tile = Tile::new_unchecked(56);
    pub const B8: Tile = Tile::new_unchecked(57);
    pub const C8: Tile = Tile::new_unchecked(58);
    pub const D8: Tile = Tile::new_unchecked(59);
    pub const E8: Tile = Tile::new_unchecked(60);
    pub const F8: Tile = Tile::new_unchecked(61);
    pub const G8: Tile = Tile::new_unchecked(62);
    pub const H8: Tile = Tile::new_unchecked(63);

    pub const TILE_STRS: [&str; 64] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];
    pub const FILE_CHARS: [char; 8] = [
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
    ];
    pub const RANK_CHARS: [char; 8] = [
        '1',
        '2',
        '3',
        '4',
        '5',
        '6',
        '7',
        '8',
    ];
}