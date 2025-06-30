use crate::Bitboard;

pub mod bishop_attacks;
pub mod bishop_magics;

pub mod rook_attacks;
pub mod rook_magics;

pub mod king_attacks;
pub mod knight_attacks;

pub mod between;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}
pub fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let b = blockers.to_u64() & entry.mask;
    let hashed = b.wrapping_mul(entry.magic);
    (entry.offset as usize) + ((hashed >> entry.shift) as usize)
}
