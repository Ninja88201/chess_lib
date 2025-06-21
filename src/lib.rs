pub mod board;
pub mod player;
pub mod bitboard;
pub mod tile;
pub mod lookup_tables;

#[cfg(test)]
mod tests {
    use crate::board::Board;

    // Regular starting position
    static position: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    // Position 5
    // static position: &'static str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

    // Position 4
    // static position: &'static str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

    #[test]
    fn depth_2() {
        let mut board = Board::new_from_fen(position).unwrap();
        let positions = board.positions(2);
        assert_eq!(positions, 400);
    }

    #[test]
    fn depth_3() {
        let mut board = Board::new_from_fen(position).unwrap();
        let positions = board.positions(3);
        assert_eq!(positions, 8902);
    }

    #[test]
    fn depth_4() {
        let mut board = Board::new_from_fen(position).unwrap();
        let positions = board.positions(4);
        assert_eq!(positions, 197281);
    }
    // #[test]
    // fn depth_5() {
    //     let mut board = Board::new_from_fen(position).unwrap();
    //     let positions = board.positions(5);
    //     assert_eq!(positions, 4865609);
    // }
}
