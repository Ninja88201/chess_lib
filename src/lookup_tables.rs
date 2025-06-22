use crate::{bitboard::Bitboard, tile::Tile};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LookupTables {
    pub rook_magics: [MagicEntry; 64],
    pub bishop_magics: [MagicEntry; 64],

    pub rook_attacks: &'static [u64],
    pub bishop_attacks: &'static [u64], 

    pub knight_table: [Bitboard; 64],
    pub king_table: [Bitboard; 64],
}

impl LookupTables {
    pub const KNIGHT_OFFSETS: [(i8, i8); 8] = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];

    pub fn new() -> Self {
        LookupTables {
            rook_magics: ROOK_MAGICS,
            bishop_magics: BISHOP_MAGICS,
            rook_attacks: ROOK_ATTACKS,
            bishop_attacks: BISHOP_ATTACKS,
            knight_table: crate::lookup_tables::init_knight_table(),
            king_table: crate::lookup_tables::init_king_table(),
        }
    }
    // fn calculate_knights() -> [Bitboard; 64] {
    //     let mut table = [Bitboard::EMPTY; 64];

    //     for i in 0..64 {
    //         table[i] = LookupTables::gen_knight_attacks(Tile::new_index(i as u8).unwrap())
    //     }

    //     table
    // }
    // fn calculate_kings() -> [Bitboard; 64] {
    //     let mut table = [Bitboard::EMPTY; 64];

    //     for i in 0..64 {
    //         table[i] = Tile::new_index(i as u8).unwrap().get_neighbours();
    //     }

    //     table
    // }
    pub fn gen_knight_attacks(tile: Tile) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let (x, y) = tile.get_coords();

        for &(dx, dy) in &Self::KNIGHT_OFFSETS {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;

            if let Some(t) = Tile::new_xy(nx as u8, ny as u8) {
                attacks.set_bit(t, true);
            }
        }

        attacks
    }
}
impl LookupTables {
    /// Compute table index: hashed blockers → offset
    #[inline]
    fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
        let b = blockers.0 & entry.mask;
        let hashed = b.wrapping_mul(entry.magic);
        (entry.offset as usize) + ((hashed >> entry.shift) as usize)
    }

    /// Rook attacks for square `sq` under occupied bitboard `occ`.
    pub fn rook_attacks_on(&self, sq: usize, occ: Bitboard) -> Bitboard {
        let entry = &self.rook_magics[sq];
        let idx = Self::magic_index(entry, occ);
        Bitboard(self.rook_attacks[idx])
    }

    /// Bishop attacks for square `sq` under occupied bitboard `occ`.
    pub fn bishop_attacks_on(&self, sq: usize, occ: Bitboard) -> Bitboard {
        let entry = &self.bishop_magics[sq];
        let idx = Self::magic_index(entry, occ);
        Bitboard(self.bishop_attacks[idx])
    }

    /// Queen attacks = rook + bishop attacks.
    pub fn queen_attacks_on(&self, sq: usize, occ: Bitboard) -> Bitboard {
        self.rook_attacks_on(sq, occ) | self.bishop_attacks_on(sq, occ)
    }
}
// Magic
include!("magics/rook_magics.rs");
include!("magics/bishop_magics.rs");

include!("magics/rook_attacks.rs");
include!("magics/bishop_attacks.rs");

impl Default for LookupTables {
    fn default() -> Self {
        LookupTables {
            rook_magics: ROOK_MAGICS,
            bishop_magics: BISHOP_MAGICS,
            rook_attacks: ROOK_ATTACKS,
            bishop_attacks: BISHOP_ATTACKS,
            knight_table: crate::lookup_tables::init_knight_table(),
            king_table: crate::lookup_tables::init_king_table(),
        }
    }
}
pub fn init_knight_table() -> [Bitboard; 64] {
    let mut arr = [Bitboard::EMPTY; 64];
    for sq in 0..64 {
        let t = Tile::new_index(sq as u8).unwrap();
        let mut bb = Bitboard::EMPTY;
        for &d in &[(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)] {
            if let Some(tt) = t.offset(d.0, d.1) {
                bb |= tt.as_mask();
            }
        }
        arr[sq] = bb;
    }
    arr
}

pub fn init_king_table() -> [Bitboard; 64] {
    let mut arr = [Bitboard::EMPTY; 64];
    for sq in 0..64 {
        let t = Tile::new_index(sq as u8).unwrap();
        let mut bb = Bitboard::EMPTY;
        for &d in &[(1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)] {
            if let Some(tt) = t.offset(d.0, d.1) {
                bb |= tt.as_mask();
            }
        }
        arr[sq] = bb;
    }
    arr
}
