pub mod board;
pub use board::Board;
pub mod player;
pub use player::Player;

pub mod r#move;
pub use r#move::Move;
pub mod move_list;
pub use move_list::MoveList;
pub mod move_enums;
pub use move_enums::MoveError;
pub use move_enums::GameState;
pub use move_enums::MoveResult;
pub mod castling;
pub use castling::CastlingRights;

pub mod piece;
pub use piece::Piece;
pub mod bitboard;
pub use bitboard::Bitboard;
pub mod tile;
pub use tile::Tile;
pub mod magics;
pub use magics::MagicEntry;
pub use magics::between::BETWEEN;
pub use magics::bishop_attacks::BISHOP_ATTACKS;
pub use magics::bishop_magics::BISHOP_MAGICS;
pub use magics::king_attacks::KING_ATTACKS;
pub use magics::knight_attacks::KNIGHT_ATTACKS;
pub use magics::rook_attacks::ROOK_ATTACKS;
pub use magics::rook_magics::ROOK_MAGICS;
pub mod zobrist;

#[cfg(test)]
mod tests {

    use crate::board::Board;

    // Regular starting position
    static POSITION_0: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    static NODES_0: &[i64] = &[20, 400, 8_902, 197_281, 4_865_609, 119_060_324, 3_195_901_860];

    // Position 3
    static POSITION_1: &'static str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    static NODES_1: &[i64] = &[14, 191, 2_812, 43_238, 674_624, 11_030_083];

    // Position 4
    static POSITION_2: &'static str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    static NODES_2: &[i64] = &[6, 264, 9_467, 422_333, 15_833_292, 706_045_033];

    // Position 5
    static POSITION_3: &'static str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    static NODES_3: &[i64] = &[44, 1_486, 62_379, 2_103_487, 89_941_194];

    #[test]
    fn position_0() {
        test_position(POSITION_0, NODES_0[4], 5);
    }
    #[test]
    fn position_1() {
        test_position(POSITION_1, NODES_1[4], 5);
    }
    #[test]
    fn position_2() {
        test_position(POSITION_2, NODES_2[4], 5);
    }
    #[test]
    fn position_3() {
        test_position(POSITION_3, NODES_3[4], 5);
    }
    fn test_position(fen: &str, expected: i64, depth: usize) {
        let mut board = Board::new_from_fen(fen).unwrap();
        let pos = board.positions_divide(depth);
        assert_eq!(pos, expected);
    }
}
