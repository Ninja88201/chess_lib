pub mod board;
pub use board::Board;
pub mod player;
pub use player::Player;

pub mod r#move;
pub use r#move::Move;
pub mod move_list;
pub use move_list::MoveList;
pub mod move_error;
pub use move_error::MoveError;
pub mod castling_rights;
pub use castling_rights::CastlingRights;

pub mod piece;
pub use piece::Piece;
pub mod bitboard;
pub use bitboard::Bitboard;
pub mod tile;
pub use tile::Tile;
pub mod lookup_tables;



#[cfg(test)]
mod tests {
    use crate::board::Board;

    // Regular starting position
    static POSITION: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    static NODES: [i64; 6] = [20, 400, 8902, 197281, 4865609, 119060324];
    
    // Position 4
    // static POSITION: &'static str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    // static NODES: [i64; 6] = [6, 264, 9467, 422333, 15833292, 706045033];
    
    // Position 5
    // static POSITION: &'static str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    // static NODES: [i64; 5] = [44, 1486, 62379, 2103487, 89941194];

    #[test]
    fn depth_2() {
        let mut board = Board::new_from_fen(POSITION).unwrap();
        let positions = board.positions(2);
        assert_eq!(positions, NODES[1]);
    }

    #[test]
    fn depth_3() {
        let mut board = Board::new_from_fen(POSITION).unwrap();
        let positions = board.positions(3);
        assert_eq!(positions, NODES[2]);
    }

    #[test]
    fn depth_4() {
        let mut board = Board::new_from_fen(POSITION).unwrap();
        let positions = board.positions(4);
        assert_eq!(positions, NODES[3]);
    }
    #[test]
    fn depth_5() {
        let mut board = Board::new_from_fen(POSITION).unwrap();
        let positions = board.positions(5);
        assert_eq!(positions, NODES[4]);
    }
    #[test]
    fn depth_6() {
        let mut board = Board::new_from_fen(POSITION).unwrap();
        let positions = board.positions(6);
        assert_eq!(positions, NODES[5]);
    }
}
