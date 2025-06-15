use chess_lib::board::{Board, Piece};
use macroquad::prelude::*;

const TILE_SIZE: f32 = 80.0;
const SPRITE_SIZE: f32 = 189.0;

#[macroquad::main("Rust Chess")]
async fn main() {
    // let mut board = Board::new();
    let mut board =
        Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1").unwrap();

    let piece_atlas = load_texture("assets/PieceAtlas.png").await.unwrap();
    piece_atlas.set_filter(FilterMode::Linear);

    let mut flipped = true;
    let mut selected_square: Option<u8> = None;

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
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
            board.undo_move();
        }
        if is_key_pressed(KeyCode::P) {
            println!("{}", board.to_fen());
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(clicked_square) = get_square(mouse_position().into(), flipped) {
                match selected_square {
                    Some(square) => {
                        // Unselect Piece
                        if square == clicked_square {
                            selected_square = None
                        }
                        // Deselect after move
                        else if board.try_move_piece(square, clicked_square) {
                            selected_square = None;
                        }
                        // Select different piece
                        else {
                            if board.occupied() & (1 << clicked_square) != 0 {
                                if let Some((_, white)) = board.get_piece_at_square(clicked_square)
                                {
                                    if white == board.white_turn {
                                        selected_square = Some(clicked_square);
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        if board.occupied() & (1 << clicked_square) != 0 {
                            selected_square = Some(clicked_square);
                        }
                    }
                }
            }
        }

        render_board(&piece_atlas, &board, selected_square, flipped);

        next_frame().await;
    }
}
fn render_board(atlas: &Texture2D, board: &Board, selecetd: Option<u8>, flipped: bool) {
    let highlight = get_square(mouse_position().into(), flipped);
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
            draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
        }
    }
    render_pieces(atlas, flipped, board);
    if let Some(s) = selecetd {
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

fn render_piece_type(atlas: &Texture2D, bitboard: u64, piece: Piece, white: bool, flipped: bool) {
    let mut b = bitboard;
    while b != 0 {
        let sq = b.trailing_zeros() as u8;
        let (x, y) = square_to_screen(sq, flipped);

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

        b &= b - 1;
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
    // let mut moves = board.generate_moves_from(selected);
    let mut moves = board.generate_legal_moves(selected);
    while moves != 0 {
        let sq = moves.trailing_zeros() as u8;
        let (x, y) = square_to_screen(sq, flipped);
        draw_circle(
            x + (TILE_SIZE / 2.0),
            y + (TILE_SIZE / 2.0),
            TILE_SIZE / 2.0 * 0.4,
            DARKGRAY,
        );
        moves &= moves - 1;
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
