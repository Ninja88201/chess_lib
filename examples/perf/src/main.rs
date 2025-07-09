use chess_lib::Board;

fn main() {
    let mut board = Board::new();
    let depth = 5;
    let nodes = board.positions(depth);
    println!("Perft({}) = {}", depth, nodes);
}
