use crate::Bitboard;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}
// Transposition tables
include!("magics/knight_attacks.rs");
include!("magics/king_attacks.rs");
// Magic
include!("magics/rook_magics.rs");
include!("magics/bishop_magics.rs");

include!("magics/rook_attacks.rs");
include!("magics/bishop_attacks.rs");

/// Compute table index: hashed blockers -> offset
#[inline]
fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let b = blockers.0 & entry.mask;
    let hashed = b.wrapping_mul(entry.magic);
    (entry.offset as usize) + ((hashed >> entry.shift) as usize)
}

pub fn rook_attacks_on(t: usize, occ: Bitboard) -> Bitboard {
    let entry = &ROOK_MAGICS[t];
    let idx = magic_index(entry, occ);
    Bitboard(ROOK_ATTACKS[idx])
}

pub fn bishop_attacks_on(t: usize, occ: Bitboard) -> Bitboard {
    let entry = &BISHOP_MAGICS[t];
    let idx = magic_index(entry, occ);
    Bitboard(BISHOP_ATTACKS[idx])
}

pub fn queen_attacks_on(t: usize, occ: Bitboard) -> Bitboard {
    rook_attacks_on(t, occ) | bishop_attacks_on(t, occ)
}
pub fn knight_attacks_on(t: usize) -> Bitboard {
    Bitboard::new(KNIGHT_ATTACKS[t])
}
pub fn king_attacks_on(t: usize) -> Bitboard {
    Bitboard::new(KING_ATTACKS[t])
}
