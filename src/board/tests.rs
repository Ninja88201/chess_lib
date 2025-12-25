use crate::{Board, Colour, MoveList, Piece, Tile};

fn empty_board_with(piece: Piece, tile: Tile, colour: Colour) -> Board {
    let mut board = Board::new_empty();
    let player = if colour.white() {
        &mut board.white
    } else {
        &mut board.black
    };
    player.place_piece(piece, tile);
    board
}

#[test]
fn knight_attacks() {
    let board = empty_board_with(Piece::Knight, Tile::E4, Colour::White);
    let attacks = board.generate_attacks_from(Tile::E4);

    let expected = Tile::E4.knight_attacks();
    assert_eq!(
        attacks, expected,
        "Knight attacks should match precomputed values"
    );
}

#[test]
fn pawn_attacks_white() {
    let board = empty_board_with(Piece::Pawn, Tile::E4, Colour::White);
    let attacks = board.generate_attacks_from(Tile::E4);

    let expected = Tile::E4.pawn_attacks(Colour::White);
    assert_eq!(attacks, expected);
}

#[test]
fn pawn_attacks_black() {
    let board = empty_board_with(Piece::Pawn, Tile::E4, Colour::Black);
    let attacks = board.generate_attacks_from(Tile::E4);

    let expected = Tile::E4.pawn_attacks(Colour::Black);
    assert_eq!(attacks, expected);
}

#[test]
fn king_attacks() {
    let board = empty_board_with(Piece::King, Tile::A1, Colour::White);
    let attacks = board.generate_attacks_from(Tile::A1);

    let expected = Tile::A1.king_attacks();
    assert_eq!(attacks, expected);
}

#[test]
fn rook_attacks_with_blocker() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Rook, Tile::D4);
    board.white.place_piece(Piece::Pawn, Tile::D6);
    board.black.place_piece(Piece::Pawn, Tile::F4);

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

    let attacks = board.generate_attacks(Colour::White);

    let expected =
        Tile::E4.knight_attacks() | board.generate_sliding_attacks(Tile::C1, false, true, None);
    assert_eq!(attacks, expected);
}

#[test]
fn king_danger() {
    let mut board = Board::new_empty();
    board.black.place_piece(Piece::Queen, Tile::E8);
    board.white.place_piece(Piece::King, Tile::E2);

    let attacks = board.generate_king_danger(Colour::White);

    println!("Board: {}", board);

    assert!(
        attacks.get_bit(Tile::E1),
        "King danger should not include friendly king tile"
    );
}
#[test]
fn captures() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::King, Tile::A3);
    board.white.place_piece(Piece::Queen, Tile::B3);

    board.black.place_piece(Piece::Pawn, Tile::H3);
    board.black.place_piece(Piece::Pawn, Tile::G3);
    board.black.place_piece(Piece::King, Tile::A8);

    let mut moves = MoveList::new();
    board.generate_legal_captures(board.turn, &mut moves);
    assert!(moves.len() > 0);
    for &m in moves.iter() {
        assert!(m.capture().is_some(), "Found a non capturing move")
    }
}

#[test]
fn parse_san_pawn_move() {
    let board = Board::new();
    let expected = board.create_move(
        Tile::E2, 
        Tile::E4, 
        Piece::Pawn, 
        None, 
        None
    );

    let actual = board.move_from_algebraic("e4");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_move() {
    let board = Board::new();
    let expected = board.create_move(
        Tile::B1, 
        Tile::C3, 
        Piece::Knight, 
        None, 
        None
    );

    let actual = board.move_from_algebraic("Nc3");

    assert_eq!(Some(expected), actual)
}

#[test]
fn parse_san_promotion() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Pawn, Tile::E7);
    let expected = board.create_move(
        Tile::E7, 
        Tile::E8, 
        Piece::Pawn, 
        None, 
        Some(Piece::Queen)
    );

    let actual = board.move_from_algebraic("e8=Q");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_promotion_capture() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Pawn, Tile::D7);
    board.black.place_piece(Piece::Queen, Tile::E8);
    let expected = board.create_move(
        Tile::D7, 
        Tile::E8, 
        Piece::Pawn, 
        Some(Piece::Queen), 
        Some(Piece::Queen)
    );

    let actual = board.move_from_algebraic("dxe8=Q");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_pawn_capture() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Pawn, Tile::D6);
    board.black.place_piece(Piece::Queen, Tile::E7);
    let expected = board.create_move(
        Tile::D6, 
        Tile::E7, 
        Piece::Pawn, 
        Some(Piece::Queen), 
        None
    );

    let actual = board.move_from_algebraic("dxe7");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_capture() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Bishop, Tile::A1);
    board.black.place_piece(Piece::Rook, Tile::D4);
    let expected = board.create_move(
        Tile::A1, 
        Tile::D4, 
        Piece::Bishop, 
        Some(Piece::Rook), 
        None
    );

    let actual = board.move_from_algebraic("Bxd4");

    assert_eq!(Some(expected), actual)
}

#[test]
fn parse_san_single_disambiguation() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Knight, Tile::D2);
    board.white.place_piece(Piece::Knight, Tile::F6);
    let expected = board.create_move(
        Tile::D2, 
        Tile::E4, 
        Piece::Knight, 
        None, 
        None
    );

    let actual = board.move_from_algebraic("Nde4");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_single_disambiguation_capture() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Knight, Tile::D2);
    board.white.place_piece(Piece::Knight, Tile::F6);
    board.black.place_piece(Piece::Rook, Tile::E4);
    let expected = board.create_move(
        Tile::D2, 
        Tile::E4, 
        Piece::Knight, 
        Some(Piece::Rook), 
        None
    );

    let actual = board.move_from_algebraic("Ndxe4");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_double_disambiguation() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Knight, Tile::D2);
    board.white.place_piece(Piece::Knight, Tile::D6);
    board.white.place_piece(Piece::Knight, Tile::F6);
    let expected = board.create_move(
        Tile::D2, 
        Tile::E4, 
        Piece::Knight, 
        None, 
        None
    );

    let actual = board.move_from_algebraic("Nd2e4");

    assert_eq!(Some(expected), actual)
}
#[test]
fn parse_san_double_disambiguation_capture() {
    let mut board = Board::new_empty();
    board.white.place_piece(Piece::Knight, Tile::D2);
    board.white.place_piece(Piece::Knight, Tile::D6);
    board.white.place_piece(Piece::Knight, Tile::F6);
    board.black.place_piece(Piece::Rook, Tile::E4);
    let expected = board.create_move(
        Tile::D2, 
        Tile::E4, 
        Piece::Knight, 
        Some(Piece::Rook), 
        None
    );

    let actual = board.move_from_algebraic("Nd2xe4");

    assert_eq!(Some(expected), actual)
}