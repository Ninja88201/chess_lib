
use crate::{Player, Piece, Tile};

#[test]
fn new_empty_player() {
    let player = Player::new_empty();
    assert!(player.pieces.none());
    for bb in player.bb.iter() {
        assert!(bb.none());
    }
}

#[test]
fn new_white_player() {
    let player = Player::new_white();
    assert!(player.bb[Piece::Pawn as usize].to_u64() == 0x0000_0000_0000_FF00);
    assert_eq!(
        player.pieces.to_u64(),
        0x0000_0000_0000_FF00
            | 0x0000_0000_0000_0042
            | 0x0000_0000_0000_0024
            | 0x0000_0000_0000_0081
            | 0x0000_0000_0000_0008
            | 0x0000_0000_0000_0010
    );
}

#[test]
fn new_black_player() {
    let player = Player::new_black();
    assert!(player.bb[Piece::Pawn as usize].to_u64() == 0x00FF_0000_0000_0000);
    assert_eq!(
        player.pieces.to_u64(),
        0x00FF_0000_0000_0000
            | 0x4200_0000_0000_0000
            | 0x2400_0000_0000_0000
            | 0x8100_0000_0000_0000
            | 0x0800_0000_0000_0000
            | 0x1000_0000_0000_0000
    );
}

#[test]
fn place_and_get_piece() {
    let mut player = Player::new_empty();
    let tile = Tile::new_xy(4, 4).unwrap();
    player.place_piece(Piece::Knight, tile);
    assert_eq!(player.get_piece(tile), Some(Piece::Knight));
}

#[test]
fn move_piece() {
    let mut player = Player::new_empty();
    let from = Tile::new_xy(1, 1).unwrap();
    let to = Tile::new_xy(2, 2).unwrap();
    player.place_piece(Piece::Rook, from);
    player.move_piece(from, to);
    assert_eq!(player.get_piece(from), None);
    assert_eq!(player.get_piece(to), Some(Piece::Rook));
}

#[test]
fn remove_piece() {
    let mut player = Player::new_empty();
    let tile = Tile::new_xy(0, 0).unwrap();
    player.place_piece(Piece::Bishop, tile);
    let removed = player.remove_piece(tile);
    assert_eq!(removed, Some(Piece::Bishop));
    assert_eq!(player.get_piece(tile), None);
}

#[test]
fn get_king_tile() {
    let player = Player::new_white();
    let king_bb = player.bb[Piece::King as usize];
    let tile = king_bb.to_bit().unwrap();
    assert_eq!(tile, player.king_tile());
}