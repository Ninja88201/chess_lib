use crate::Tile;

#[test]
fn directions_white() {
    let center = Tile::new_xy(3, 3).unwrap();
    assert_eq!(center.forward(true).unwrap().get_coords(), (3, 4));
    assert_eq!(center.backward(true).unwrap().get_coords(), (3, 2));
    assert_eq!(center.left(true).unwrap().get_coords(), (2, 3));
    assert_eq!(center.right(true).unwrap().get_coords(), (4, 3));
}

#[test]
fn directions_black() {
    let center = Tile::new_xy(3, 3).unwrap();
    assert_eq!(center.forward(false).unwrap().get_coords(), (3, 2));
    assert_eq!(center.backward(false).unwrap().get_coords(), (3, 4));
    assert_eq!(center.left(false).unwrap().get_coords(), (4, 3));
    assert_eq!(center.right(false).unwrap().get_coords(), (2, 3));
}

#[test]
fn promotion_rows() {
    let white_promo = Tile::new_xy(4, 7).unwrap();
    let black_promo = Tile::new_xy(4, 0).unwrap();
    assert!(white_promo.is_promotion(true));
    assert!(black_promo.is_promotion(false));
}

#[test]
fn pawn_start_rows() {
    let white_start = Tile::new_xy(4, 1).unwrap();
    let black_start = Tile::new_xy(4, 6).unwrap();
    assert!(white_start.is_pawn_start(true));
    assert!(black_start.is_pawn_start(false));
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