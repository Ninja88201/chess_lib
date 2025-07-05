use crate::{Bitboard, Tile};


#[test]
fn set_get_bit() {
    let tile = Tile::new_index(5).unwrap();
    let mut bb = Bitboard::EMPTY;

    bb.set_bit(tile, true);
    assert!(bb.get_bit(tile));

    bb.set_bit(tile, false);
    assert!(!bb.get_bit(tile));
}

#[test]
fn bitwise_and_or() {
    let t1 = Tile::new_index(3).unwrap();
    let t2 = Tile::new_index(6).unwrap();

    let bb1 = Bitboard::from_tile(t1);
    let bb2 = Bitboard::from_tile(t2);

    let bb_or = bb1 | bb2;
    assert!(bb_or.get_bit(t1));
    assert!(bb_or.get_bit(t2));

    let bb_and = bb1 & bb2;
    assert_eq!(bb_and.count_ones(), 0);
}

#[test]
fn not_operation() {
    let tile = Tile::new_index(2).unwrap();
    let bb = Bitboard::from_tile(tile);
    let bb_not = !bb;

    assert!(!bb_not.get_bit(tile));
    assert_eq!(bb_not.count_ones(), 63);
}

#[test]
fn shift_operations() {
    let bb = Bitboard::new(0b1);
    let shifted = bb << 3;
    assert_eq!(shifted.to_u64(), 0b1000);
    let shifted_back = shifted >> 3;
    assert_eq!(shifted_back.to_u64(), 0b1);
}

#[test]
fn to_bit_singleton() {
    let tile = Tile::new_index(10).unwrap();
    let bb = Bitboard::from_tile(tile);
    assert_eq!(bb.to_bit(), Some(tile));

    let combined = bb | Bitboard::from_tile(Tile::new_index(11).unwrap());
    assert_eq!(combined.to_bit(), None);
}

#[test]
fn iteration_over_bits() {
    let mut bb = Bitboard::EMPTY;
    let t1 = Tile::new_index(7).unwrap();
    let t2 = Tile::new_index(15).unwrap();
    bb.set_bit(t1, true);
    bb.set_bit(t2, true);

    let mut collected: Vec<u8> = bb.clone().map(|t| t.to_u8()).collect();
    collected.sort();
    assert_eq!(collected, vec![7, 15]);
}

#[test]
fn some_and_none() {
    let bb = Bitboard::EMPTY;
    assert!(bb.none());
    assert!(!bb.some());

    let bb2 = Bitboard::from(1u64);
    assert!(bb2.some());
    assert!(!bb2.none());
}