use crate::{Board, CastlingRights, Piece};

pub mod consts;
use consts::{CASTLING, EN_PASSANT, PIECE_SQUARE, SIDE_TO_MOVE};
impl Board
{
    pub fn to_zobrist_hash(&self) -> u64 {
        let mut hash: u64 = 0;

        let white_pieces = self.white.get_all_pieces();
        let black_pieces = self.black.get_all_pieces();
        for (p, t) in white_pieces {
            hash ^= PIECE_SQUARE[p.to_zobrist_index(true)][t.to_usize()];
        }
        for (p, t) in black_pieces {
            hash ^= PIECE_SQUARE[p.to_zobrist_index(false)][t.to_usize()];
        }

        if self.white_turn == false {
            hash ^= SIDE_TO_MOVE;
        }

        if self.castling.contains(CastlingRights::WHITE_KINGSIDE) {
            hash ^= CASTLING[0];
        }
        if self.castling.contains(CastlingRights::WHITE_QUEENSIDE) {
            hash ^= CASTLING[1];
        }
        if self.castling.contains(CastlingRights::BLACK_KINGSIDE) {
            hash ^= CASTLING[2];
        }
        if self.castling.contains(CastlingRights::BLACK_QUEENSIDE) {
            hash ^= CASTLING[3];
        }

        if let Some(tile) = self.en_passant {
            let (player, _) = self.current_players();
            for t in player.bb[Piece::Pawn as usize] {
                if t.pawn_attacks(self.white_turn).get_bit(tile) {
                    let (file, _) = tile.get_coords();
        
                    hash ^= EN_PASSANT[file as usize];
                    break;
                }
            }
        }

        hash
    }
}