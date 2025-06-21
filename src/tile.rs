use crate::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile(pub u8);

impl Tile {
    pub fn new_index(index: u8) -> Option<Tile> {
        if index >= 64 {
            return None;
        }
        Some(Tile(index))
    }
    pub fn new_xy(x: u8, y:u8) -> Option<Tile> {
        if x >= 8 || y >= 8{ 
            return None;
        }
        Some(Tile((y * 8) + x))
    }
    pub fn get_neighbours(&self) -> Bitboard {
        let (x, y) = self.get_coords();
        let mut result = Bitboard::EMPTY;

        const DIRS: [(i8, i8); 8] = [
            (0, 1), (1, 1), (1, 0), (1, -1),
            (0, -1), (-1, -1), (-1, 0), (-1, 1),
        ];

        for (dx, dy) in DIRS {
            let nx = x as i8 + dx;
            let ny = y as i8 + dy;
            if (0..8).contains(&nx) && (0..8).contains(&ny) {
                if let Some(t) = Tile::new_xy(nx as u8, ny as u8) {
                    result.set_bit(t, true);
                }
            }
        }

        result
    }
    pub fn as_mask(&self) -> Bitboard {
        if self.0 >= 64 { 
            Bitboard::EMPTY
        } else {
            Bitboard::new(1u64 << self.0)
        }
    }
    pub fn offset(&self, dx: i8, dy: i8) -> Option<Self> {
        let (x, y) = self.get_coords();
        let nx = x as i8 + dx;
        let ny = y as i8 + dy;
        if (0..8).contains(&nx) && (0..8).contains(&ny) {
            Tile::new_xy(nx as u8, ny as u8)
        } else {
            None
        }
    }
    pub fn get_coords(&self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }
    pub fn forward(&self, white: bool) -> Option<Self> {
        self.offset(0, if white { 1 } else { -1 })
    }

    pub fn backward(&self, white: bool) -> Option<Self> {
        self.offset(0, if white { -1 } else { 1 })
    }

    pub fn left(&self, white: bool) -> Option<Self> {
        self.offset(if white { -1 } else { 1 }, 0)
    }

    pub fn right(&self, white: bool) -> Option<Self> {
        self.offset(if white { 1 } else { -1 }, 0)
    }
    pub fn is_promotion(&self, white: bool) -> bool {
        let y = self.get_coords().1;
        match white {
            true => y == 7,
            false => y == 0,
        }
    }
    pub fn is_pawn_start(&self, white: bool) -> bool {
        let y = self.get_coords().1;
        match white {
            true => y == 1,
            false => y == 6,
        }
    }
}
impl Into<usize> for Tile
{
    fn into(self) -> usize {
        self.0 as usize
    }
}