use crate::{Bitboard, Board, Piece, Tile};

impl Board {
    pub fn generate_attacks(&self, white: bool) -> Bitboard {
        let (player, _) = self.get_players(white);
        let mut attacks = Bitboard::EMPTY;
        for tile in player.pieces {
            if let Some((piece, is_white)) = self.get_piece_at_tile(tile) {
                attacks |= self.generate_attacks_from_piece(tile, piece, is_white, None);
            }
        }
        attacks
    }
    pub fn generate_king_danger(&self, white: bool) -> Bitboard {
        let (player, _) = self.get_players(white);
        let mut attacks = Bitboard::EMPTY;
        for tile in player.pieces {
            if let Some((piece, is_white)) = self.get_piece_at_tile(tile) {
                attacks |= self.generate_attacks_from_piece(tile, piece, is_white, Some(is_white));
            }
        }
        attacks
    }

    pub fn generate_attacks_from(&self, tile: Tile) -> Bitboard {
        match self.get_piece_at_tile(tile) {
            Some((piece, white)) => self.generate_attacks_from_piece(tile, piece, white, None),
            None => Bitboard::EMPTY,
        }
    }

    pub fn generate_attacks_from_piece(
        &self,
        tile: Tile,
        piece: Piece,
        white: bool,
        king_danger: Option<bool>,
    ) -> Bitboard {
        match piece {
            Piece::Pawn => tile.pawn_attacks(white),
            Piece::Knight => tile.knight_attacks(),
            Piece::Bishop => self.generate_sliding_attacks(tile, false, true, king_danger),
            Piece::Rook => self.generate_sliding_attacks(tile, true, false, king_danger),
            Piece::Queen => self.generate_sliding_attacks(tile, true, true, king_danger),
            Piece::King => tile.king_attacks(),
        }
    }

    pub fn generate_sliding_attacks(
        &self,
        tile: Tile,
        straight: bool,
        diagonal: bool,
        king_danger: Option<bool>,
    ) -> Bitboard {
        let occ = match king_danger {
            None => self.occupied(),
            Some(white) => {
                self.occupied()
                    & if !white {
                        !self.white.bb[Piece::King as usize]
                    } else {
                        !self.black.bb[Piece::King as usize]
                    }
            }
        };

        let mut attacks = Bitboard::EMPTY;

        if straight {
            attacks |= tile.rook_attacks(occ);
        }

        if diagonal {
            attacks |= tile.bishop_attacks(occ);
        }

        attacks
    }
}