use crate::{Tile, Piece, Bitboard, Board, lookup_tables};

impl Board {

    pub fn generate_attacks(&self, white: bool) -> Bitboard {
        let (player, _) = self.get_players(white);
        let mut attacks = Bitboard::EMPTY;
        for t in player.pieces() {
            attacks |= self.generate_attacks_from(t);
        }
        attacks
    }
    pub fn generate_attacks_from(&self, tile: Tile) -> Bitboard {
        match self.get_piece_at_tile(tile) {
            Some((p, w)) => self.generate_attacks_from_piece(tile, p, w),
            None => Bitboard::EMPTY,
        }
    }
    pub fn generate_attacks_from_piece(&self, tile: Tile, piece: Piece, white: bool) -> Bitboard {
        match piece {
            Piece::Pawn => Board::generate_pawn_attacks(tile, white),
            Piece::Knight => lookup_tables::knight_attacks_on(Into::<usize>::into(tile)),
            Piece::Bishop => self.generate_sliding_attacks(tile, false, true),
            Piece::Rook => self.generate_sliding_attacks(tile, true, false),
            Piece::Queen => self.generate_sliding_attacks(tile, true, true),
            Piece::King => lookup_tables::king_attacks_on(Into::<usize>::into(tile)),
        }
    }
    fn generate_pawn_attacks(tile: Tile, white: bool) -> Bitboard {
        let bb = tile.as_mask();
        match white {
            true => ((bb << 7) & !Self::FILE_H) | ((bb << 9) & !Self::FILE_A),
            false => ((bb >> 7) & !Self::FILE_A) | ((bb >> 9) & !Self::FILE_H),
        }
    }

    pub fn generate_sliding_attacks(&self, tile: Tile, straight: bool, diagonal: bool) -> Bitboard {
        let sq = Into::<usize>::into(tile);
        let occ = self.occupied();

        let mut attacks = Bitboard::EMPTY;

        if straight {
            attacks |= lookup_tables::rook_attacks_on(sq, occ);
        }

        if diagonal {
            attacks |= lookup_tables::bishop_attacks_on(sq, occ);
        }

        attacks
    }

}