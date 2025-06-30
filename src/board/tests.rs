use crate::{Piece, Board, Tile};

fn empty_board_with(piece: Piece, tile: Tile, white: bool) -> Board {
    let mut board = Board::new_empty();
    let player = if white {
        &mut board.white
    } else {
        &mut board.black
    };
    player.place_piece(piece, tile);
    board
}

#[test]
fn knight_attacks() {
    let board = empty_board_with(Piece::Knight, Tile::E4, true);
    let attacks = board.generate_attacks_from(Tile::E4);

    let expected = Tile::E4.knight_attacks();
    assert_eq!(
        attacks, expected,
        "Knight attacks should match precomputed values"
    );
}

#[test]
fn pawn_attacks_white() {
    let board = empty_board_with(Piece::Pawn, Tile::E4, true);
    let attacks = board.generate_attacks_from(Tile::E4);

    let expected = Tile::E4.pawn_attacks(true);
    assert_eq!(attacks, expected);
}

#[test]
fn pawn_attacks_black() {
    let board = empty_board_with(Piece::Pawn, Tile::E4, false);
    let attacks = board.generate_attacks_from(Tile::E4);

    let expected = Tile::E4.pawn_attacks(false);
    assert_eq!(attacks, expected);
}

#[test]
fn king_attacks() {
    let board = empty_board_with(Piece::King, Tile::A1, true);
    let attacks = board.generate_attacks_from(Tile::A1);

    let expected = Tile::A1.king_attacks();
    assert_eq!(attacks, expected);
}

#[test]
fn rook_attacks_with_blocker() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Rook, Tile::D4);
    board.white.place_piece(Piece::Pawn, Tile::D6); // block vertical
    board.black.place_piece(Piece::Pawn, Tile::F4); // enemy piece

    let attacks = board.generate_attacks_from(Tile::D4);
    let occ = board.occupied();

    let expected = Tile::D4.rook_attacks(occ);
    assert_eq!(attacks, expected);
}

#[test]
fn mulitple_attacks() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Knight, Tile::E4);
    board.white.place_piece(Piece::Bishop, Tile::C1);

    let attacks = board.generate_attacks(true);

    let expected =
        Tile::E4.knight_attacks() | board.generate_sliding_attacks(Tile::C1, false, true, None);
    assert_eq!(attacks, expected);
}

#[test]
fn king_danger() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Queen, Tile::D1);
    board.white.place_piece(Piece::King, Tile::E1); // should be ignored

    let attacks = board.generate_king_danger(false);

    assert!(
        !attacks.get_bit(Tile::E1),
        "King danger should not include friendly king tile"
    );
}