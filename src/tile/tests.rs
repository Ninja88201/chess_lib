use crate::{Colour, Tile};

#[test]
fn directions_white() {
    let center = Tile::new_xy(3, 3).unwrap();
    assert_eq!(center.forward(Colour::White).unwrap().get_coords(), (3, 4));
    assert_eq!(center.backward(Colour::White).unwrap().get_coords(), (3, 2));
    assert_eq!(center.left(Colour::White).unwrap().get_coords(), (2, 3));
    assert_eq!(center.right(Colour::White).unwrap().get_coords(), (4, 3));
}

#[test]
fn directions_black() {
    let center = Tile::new_xy(3, 3).unwrap();
    assert_eq!(center.forward(Colour::Black).unwrap().get_coords(), (3, 2));
    assert_eq!(center.backward(Colour::Black).unwrap().get_coords(), (3, 4));
    assert_eq!(center.left(Colour::Black).unwrap().get_coords(), (4, 3));
    assert_eq!(center.right(Colour::Black).unwrap().get_coords(), (2, 3));
}

#[test]
fn promotion_rows() {
    let white_promo = Tile::new_xy(4, 7).unwrap();
    let black_promo = Tile::new_xy(4, 0).unwrap();
    assert!(white_promo.is_promotion(Colour::White));
    assert!(black_promo.is_promotion(Colour::Black));
}

#[test]
fn pawn_start_rows() {
    let white_start = Tile::new_xy(4, 1).unwrap();
    let black_start = Tile::new_xy(4, 6).unwrap();
    assert!(white_start.is_pawn_start(Colour::White));
    assert!(black_start.is_pawn_start(Colour::Black));
}

#[test]
fn display_tile() {
    let tile = Tile::new_xy(0, 0).unwrap();
    assert_eq!(tile.to_string(), "a1");

    let tile = Tile::new_xy(7, 7).unwrap();
    assert_eq!(tile.to_string(), "h8");
}

#[test]
fn offset_out_of_bounds() {
    let tile = Tile::new_xy(0, 0).unwrap();
    assert!(tile.offset(-1, 0).is_none());
    assert!(tile.offset(0, -1).is_none());
}