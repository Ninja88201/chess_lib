use std::io::stdin;
use std::pin::Pin;

use chess_lib::bitboard::Bitboard;
use chess_lib::board::{Board, Piece};
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

const TILE_SIZE: f32 = 80.0;
const SPRITE_SIZE: f32 = 189.0;

#[macroquad::main("Rust Chess")]
async fn main() {
    // let mut board = Board::new();
    let mut board =
        // Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        // Board::new_from_fen("7N/2Pp2N1/1r6/2k1KPp1/p1p1R3/2n2PBP/8/8 w - - 0 1").unwrap();
        Board::new_from_fen("8/8/rnbqkbnr/pp pppppp/PPPPPPPP/RNBQKBNR/8/8 w").unwrap();
        

    let piece_atlas = load_texture("assets/PieceAtlas.png").await.unwrap();
    piece_atlas.set_filter(FilterMode::Linear);

    let mut flipped = true;
    let mut selected_square: Option<u8> = None;
    // let mut player_white: bool = true;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Escape) {
            return;
        }
        if is_key_pressed(KeyCode::F) {
            flipped = !flipped
        }
        if is_key_pressed(KeyCode::R) {
            board = Board::new();
        }
        if is_key_pressed(KeyCode::U) {
            let mut buffer = String::new();
            let result = stdin().read_line(&mut buffer);
            board = Board::new_from_fen(&buffer.to_string()).unwrap();
        }
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
            board.undo_move();
        }
        if is_key_pressed(KeyCode::P) {
            println!("{}", board.to_fen());
            let square = get_square(mouse_position().into(), flipped);
            println!("mouse on square: {:?}", square);
        }
        // println!("en_passant: {:?}", board.en_passant);
        if is_mouse_button_pressed(MouseButton::Left) {

            if let Some(clicked_square) = get_square(mouse_position().into(), flipped) {
                match selected_square {
                    Some(square) => {
                            let result = board.try_move_piece(square, clicked_square, &graphical_promote, (&piece_atlas, board.white_turn, flipped)).await;
                            match result {
                                Ok(_) => {
                                    selected_square = None;
                                },
                                Err(e) => {
                                    use chess_lib::board::MoveError as me;
                                    match e {
                                        me::IllegalMove=>println!("That move is illegal"),
                                        me::WrongTurn=>println!("It's not your turn"),
                                        me::PiecePinned=>println!("That piece is pinned"),
                                        me::Stalemate=>println!("The board is in a stalemate"),
                                        me::NoPieceSelected => selected_square = None,
                                        me::SameTile => selected_square = None,
                                        me::FriendlyCapture => {
                                            selected_square = Some(clicked_square)
                                        },
                                        me::Cancelled => println!("Move cancelled"),
                                    }
                                    
                                },
                            }
                    }
                    None => {
                        if board.occupied().get_bit(clicked_square) {
                            selected_square = Some(clicked_square);
                        }
                        else {
                            selected_square = None;
                        }
                    }
                }
            }
        }
        // if board.white_turn != player_white {
        //     let moves = board.generate_legal_moves(board.white_turn);
        //     let (from, to) = pick_random_move(moves).unwrap();
        //     board.try_move_piece(from, to);
        // }
        if is_key_pressed(KeyCode::Space) {
            let moves = board.generate_legal_moves(board.white_turn);
            let _ = board.make_move_unchecked(moves.choose().copied().unwrap());
        }

        render_board(&piece_atlas, &board, selected_square, flipped);

        next_frame().await;
    }
}
async fn graphical_promote(square: u8, context: (&Texture2D, bool, bool)) -> Option<Piece> {
    let (x, y) = square_to_screen(square, context.2);
    let mut grace = true;
    loop {
        draw_rectangle(x, y, TILE_SIZE, TILE_SIZE * 4.0, WHITE);

        draw_texture_ex(
            context.0,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(get_piece_sprite_rect(Piece::Queen, context.1)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            context.0,
            x,
            y + TILE_SIZE,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(get_piece_sprite_rect(Piece::Rook, context.1)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            context.0,
            x,
            y + (TILE_SIZE * 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(get_piece_sprite_rect(Piece::Bishop, context.1)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            context.0,
            x,
            y + (TILE_SIZE * 3.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(get_piece_sprite_rect(Piece::Knight, context.1)),
                ..Default::default()
            },
        );
        if is_mouse_button_released(MouseButton::Left) && !grace{
            let queen: Rect = Rect::new(x, y, TILE_SIZE, TILE_SIZE);
            let rook: Rect = Rect::new(x, y + TILE_SIZE, TILE_SIZE, TILE_SIZE);
            let bishop: Rect = Rect::new(x, y + (TILE_SIZE * 2.0), TILE_SIZE, TILE_SIZE);
            let knight: Rect = Rect::new(x, y + (TILE_SIZE * 3.0), TILE_SIZE, TILE_SIZE);
            if queen.contains(mouse_position().into()) {
                return Some(Piece::Queen)
            }
            if rook.contains(mouse_position().into()) {
                return Some(Piece::Rook)
            }
            if bishop.contains(mouse_position().into()) {
                return Some(Piece::Bishop)
            }
            if knight.contains(mouse_position().into()) {
                return Some(Piece::Knight)
            }
            return None
        }
        if is_mouse_button_released(MouseButton::Left) {
            grace = false;
        }
        next_frame().await
    }
}

fn render_board(atlas: &Texture2D, board: &Board, selected: Option<u8>, flipped: bool) {
    let highlight = get_square(mouse_position().into(), flipped);
    let white_in_check = board.is_in_check(true);
    let black_in_check = board.is_in_check(false);
    for rank in 0..8 {
        for file in 0..8 {
            let (x, y) = square_to_screen(rank * 8 + file, flipped);

            let is_light = (file + rank) % 2
                == match flipped {
                    true => 1,
                    false => 0,
                };
            let mut color = if is_light {
                Color::from_rgba(240, 217, 181, 255)
            } else {
                Color::from_rgba(181, 136, 99, 255)
            };
            if let Some(pos) = highlight {
                if (rank * 8 + file) as u8 == pos {
                    color.a = 0.75;
                }
            }
            if white_in_check {
                if (rank * 8 + file) as u8 == board.white.get_king_square() {
                    color.r = 1.0;
                    color.g *= 0.5;
                    color.b *= 0.5;
                }
            }
            if black_in_check {
                if (rank * 8 + file) as u8 == board.black.get_king_square() {
                    color.r = 1.0;
                    color.g *= 0.5;
                    color.b *= 0.5;
                }
            }
            draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
        }
    }
    render_pieces(atlas, flipped, board);
    if let Some(s) = selected {
        if let Some((_, white)) = board.get_piece_at_square(s) {
            if white == board.white_turn {
                render_moves(board, s, flipped);
            }
        }
    }
}

fn render_pieces(atlas: &Texture2D, flipped: bool, board: &Board) {
    for (is_white, player) in [(true, &board.white), (false, &board.black)] {
        for i in 0..player.bb.len() {
            render_piece_type(atlas, player.bb[i], Piece::from_index(i), is_white, flipped);
        }
    }
}

fn render_piece_type(atlas: &Texture2D, bitboard: Bitboard, piece: Piece, white: bool, flipped: bool) {
    for s in bitboard {
        let (x, y) = square_to_screen(s, flipped);

        draw_texture_ex(
            atlas,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(get_piece_sprite_rect(piece, white)),
                ..Default::default()
            },
        );
    }
}

fn get_piece_sprite_rect(piece: Piece, white: bool) -> Rect {
    use Piece as pt;
    let (row, col) = match (piece, white) {
        (pt::Pawn, true) => (0.0, 0.0),
        (pt::Pawn, false) => (2.0, 0.0),
        (pt::Knight, true) => (0.0, 1.0),
        (pt::Knight, false) => (2.0, 1.0),
        (pt::Bishop, true) => (0.0, 2.0),
        (pt::Bishop, false) => (2.0, 2.0),
        (pt::Rook, true) => (1.0, 0.0),
        (pt::Rook, false) => (3.0, 0.0),
        (pt::Queen, true) => (1.0, 1.0),
        (pt::Queen, false) => (3.0, 1.0),
        (pt::King, true) => (1.0, 2.0),
        (pt::King, false) => (3.0, 2.0),
    };
    Rect::new(
        col * SPRITE_SIZE,
        row * SPRITE_SIZE,
        SPRITE_SIZE,
        SPRITE_SIZE,
    )
}
fn render_moves(board: &Board, selected: u8, flipped: bool) {
    for m in board.generate_legal_moves_from(selected) {
        let (x, y) = square_to_screen(m.to, flipped);
        draw_circle(
            x + (TILE_SIZE / 2.0),
            y + (TILE_SIZE / 2.0),
            TILE_SIZE / 2.0 * 0.4,
            DARKGRAY,
        );
    }
}
fn get_square(pos: Vec2, flipped: bool) -> Option<u8> {
    let file = (pos.x / TILE_SIZE).floor() as u8;
    let rank = (pos.y / TILE_SIZE).floor() as u8;
    if file >= 8 || rank >= 8 {
        return None;
    }
    match flipped {
        true => {
            let clicked_rank = 7 - rank;
            return Some(clicked_rank * 8 + file);
        }
        false => {
            let clicked_rank = rank;
            return Some(clicked_rank * 8 + (7 - file));
        }
    }
}
fn square_to_screen(square: u8, flipped: bool) -> (f32, f32) {
    let file = square % 8;
    let rank = square / 8;
    let x = match flipped {
        true => file as f32 * TILE_SIZE,
        false => (7 - file) as f32 * TILE_SIZE,
    };
    let y = match flipped {
        true => (7 - rank) as f32 * TILE_SIZE,
        false => rank as f32 * TILE_SIZE,
    };
    (x, y)
}
