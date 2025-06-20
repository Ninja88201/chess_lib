use std::ops::Add;

use crate::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile(pub u8);

impl Tile {
    // pub fn new_index<T: Into<u8>>
    // (index: T) -> Tile {
    //     Tile(index.into())
    // }
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
    // pub fn new_xy<T: TryInto<u8>>(x: T, y: T) -> Result<Tile, T::Error> {
    //     let x: u8 = x.try_into()?;
    //     let y: u8 = y.try_into()?;
    //     Ok(Tile((y * 8) + x))
    // }
    pub fn get_neighbours(&self) -> Bitboard {
        let mut result = Bitboard::EMPTY;
        if let Some(t) = Tile::new_index(self.0 + 9 as u8) {
            result.set_bit(t, true);
        }
        if let Some(t) = Tile::new_index(self.0 + 8 as u8) {
            result.set_bit(t, true);
        }
        if let Some(t) = Tile::new_index(self.0 + 7 as u8) {
            result.set_bit(t, true);
        }

        if let Some(t) = Tile::new_index(self.0 + 1 as u8) {
            result.set_bit(t, true);
        }
        if let Some(t) = Tile::new_index(self.0.saturating_sub(1)) {
            result.set_bit(t, true);
        }

        if let Some(t) = Tile::new_index(self.0.saturating_sub(7)) {
            result.set_bit(t, true);
        }
        if let Some(t) = Tile::new_index(self.0.saturating_sub(8)) {
            result.set_bit(t, true);
        }
        if let Some(t) = Tile::new_index(self.0.saturating_sub(9)) {
            result.set_bit(t, true);
        }
        
        result
    }
    pub fn as_mask(&self) -> Bitboard {
        if self.0 >= 64 { return Bitboard::EMPTY; }
        Bitboard::new(1u64 << self.0)
    }
    pub fn get_coords(&self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }
    pub fn forward(&self, white: bool) -> Option<Tile> {
        let direction = match white {
            true => 1,
            false => -1,
        };
        let (x, y) = self.get_coords();
        Tile::new_xy(x, (y as i8 + direction) as u8)
    } 
    pub fn backward(&self, white: bool) -> Option<Tile> {
        let direction = match white {
            true => -1,
            false => 1,
        };
        let (x, y) = self.get_coords();
        Tile::new_xy(x, (y as i8 + direction) as u8)
    }
    pub fn left(&self, white: bool) -> Option<Tile> {
        let direction = match white {
            true => -1,
            false => 1,
        };
        let (x, y) = self.get_coords();
        Tile::new_xy((x as i8 + direction) as u8, y)
    }
    pub fn right(&self, white: bool) -> Option<Tile> {
        let direction = match white {
            true => 1,
            false => -1,
        };
        let (x, y) = self.get_coords();
        Tile::new_xy((x as i8 + direction) as u8, y)
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
    pub fn in_board(&self) -> bool {
        self.0 < 64
    }
}
// impl Add for Tile {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Tile(self.0 + rhs.0)
//     }
// }
// impl Add<u8> for Tile {
//     type Output = Self;

//     fn add(self, rhs: u8) -> Self::Output {
//         Tile(self.0 + rhs)
//     }
// }
// impl Add<i32> for Tile {
//     type Output = Self;

//     fn add(self, rhs: i32) -> Self::Output {
//         Tile(self.0 + rhs as u8)
//     }
// }
// impl Add<i8> for Tile {
//     type Output = Self;

//     fn add(self, rhs: i8) -> Self::Output {
//         Tile(self.0 + rhs as u8)
//     }
// }
// impl Add for &Tile {
//     type Output = Tile;

//     fn add(self, rhs: Self) -> Self::Output {
//         Tile(self.0 + rhs.0)
//     }
// }
// impl Add<u8> for &Tile {
//     type Output = Tile;

//     fn add(self, rhs: u8) -> Self::Output {
//         Tile(self.0 + rhs)
//     }
// }
// impl Add<i32> for &Tile {
//     type Output = Tile;

//     fn add(self, rhs: i32) -> Self::Output {
//         Tile(self.0 + rhs as u8)
//     }
// }
// impl Sub<i32> for Tile {
//     type Output = Tile;

//     fn sub(self, rhs: i32) -> Self::Output {
//         Tile(self.0 - rhs as u8)
//     }
// }
// impl Sub<i32> for &Tile {
//     type Output = Tile;

//     fn sub(self, rhs: i32) -> Self::Output {
//         Tile(self.0 - rhs as u8)
//     }
// }
// impl AddAssign<i8> for Tile {
//     fn add_assign(&mut self, rhs: i8) {
//         self.0 += rhs as u8
//     }
// }