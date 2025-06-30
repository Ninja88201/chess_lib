use chess_lib::{Board, MoveList};
use std::io::{self, stdin, Write};

fn main() {
    let mut board = Board::new();
    loop {
        // println!("\n{}\n", board);
        let _ = board.draw_terminal_board();

        if board.is_checkmate(board.white_turn) {
            println!("Checkmate! {} wins.", if board.white_turn { "Black" } else { "White" });
            break;
        }
        if board.is_stalemate(board.white_turn) {
            println!("Stalemate!");
            break;
        }

        print!("Enter move (e.g. e2e4), 'u' to undo, 'r' to reset, 'fen' to input FEN, or 'q' to quit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "q" => break,
            "u" => {
                board.undo_move();
                continue;
            }
            "r" => {
                board = Board::new();
                continue;
            }
            "fen" => {
                print!("Enter FEN: ");
                io::stdout().flush().unwrap();
                let mut fen = String::new();
                stdin().read_line(&mut fen).unwrap();
                match Board::new_from_fen(fen.trim()) {
                    Ok(b) => board = b,
                    Err(e) => println!("Invalid FEN: {}", e),
                }
                continue;
            }
            _ => {
            let mov = board.move_from_algebraic(&input);
            match mov {
                Some(m) => {
                    let mut moves = MoveList::new();
                    board.generate_legal_moves(board.white_turn, &mut moves);
                    if moves.contains(&m) {
                        board.make_move_unchecked(m)
                    }
                },
                None => println!("Not a move"),
            }
            },
        }
    }
}