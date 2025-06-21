use crate::{bitboard::Bitboard, tile::Tile};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LookupTables
{
    pub knight_table: [Bitboard; 64],
    pub king_table: [Bitboard; 64],
}

impl LookupTables {
    pub const KNIGHT_OFFSETS: [(i8, i8); 8] = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];

    pub fn new() -> Self {
        Self { knight_table: LookupTables::calculate_knights(), king_table: LookupTables::calculate_kings() }
    }
    fn calculate_knights() -> [Bitboard; 64] {
        let mut table = [Bitboard::EMPTY; 64];

        for i in 0..64 {
            table[i] = LookupTables::gen_knight_attacks(Tile::new_index(i as u8).unwrap())
        }

        table
    }
    fn calculate_kings() -> [Bitboard; 64] {
        let mut table = [Bitboard::EMPTY; 64];

        for i in 0..64 {
            table[i] = Tile::new_index(i as u8).unwrap().get_neighbours();
        }

        table
    }
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