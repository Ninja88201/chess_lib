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
    static POSITION_0: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    static NODES_0: &[i64] = &[20, 400, 8902, 197281, 4865609, 119060324, 3195901860];
    
    // Position 2
    static POSITION_1: &'static str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    static NODES_1: &[i64] = &[14, 191, 2812, 43238, 674624, 11030083];
    
    // Position 4
    static POSITION_2: &'static str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    static NODES_2: &[i64] = &[6, 264, 9467, 422333, 15833292, 706045033];
    
    // Position 5
    static POSITION_3: &'static str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    static NODES_3: &[i64] = &[44, 1486, 62379, 2103487, 89941194];
    
    static POSITIONS: [&'static str; 4] = [POSITION_0, POSITION_1, POSITION_2, POSITION_3];
    static NODES: [&'static [i64]; 4] = [NODES_0, NODES_1, NODES_2, NODES_3];
    
    // #[test]
    // fn depth_2() {
    //     let mut board = Board::new_from_fen(POSITION_3).unwrap();
    //     let pos = board.positions_divide(2);
    //     assert_eq!(pos, NODES_3[1]);
    // }

    // #[test]
    // fn depth_3() {
    //     let mut board = Board::new_from_fen(POSITION_3).unwrap();
    //     let pos = board.positions_divide(3);
    //     assert_eq!(pos, NODES_3[2]);
    // }

    // #[test]
    // fn depth_4() {
    //     let mut board = Board::new_from_fen(POSITION_3).unwrap();
    //     let pos = board.positions_divide(4);
    //     assert_eq!(pos, NODES_3[3]);
    // }
    // #[test]
    // fn depth_5() {
    //     let mut board = Board::new_from_fen(POSITION_3).unwrap();
    //     let pos = board.positions_divide(5);
    //     assert_eq!(pos, NODES_3[4]);
    // }
    // #[test]
    // fn depth_6() {
    //     let mut board = Board::new_from_fen(POSITION).unwrap();
    //     let pos = board.positions(6);
    //     assert_eq!(pos, NODES[5]);
    // }
    // #[test]
    // fn depth_7() {
    //     let mut board = Board::new_from_fen(POSITION).unwrap();
    //     let pos = board.positions(7);
    //     assert_eq!(pos, NODES[6]);
    // }
    #[test]
    fn all_positions() {
        for i in 0..4 {
            let mut board = Board::new_from_fen(POSITIONS[i]).unwrap();
            for depth in 1..6 {
                let pos = board.positions(depth);
                assert_eq!(pos, NODES[i][depth as usize - 1])
            }
        }
    }
}
