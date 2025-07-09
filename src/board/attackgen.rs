use crate::{Bitboard, Board, Colour, Piece, Tile};

impl Board {
    /// Generates a bitboard representing all the tiles the given colour attacks
    pub fn generate_attacks(&self, colour: Colour) -> Bitboard {
        let (player, _) = self.get_players(colour);
        let mut attacks = Bitboard::EMPTY;
        for tile in player.pieces {
            if let Some((piece, c)) = self.get_piece_at_tile(tile) {
                attacks |= self.generate_attacks_from_piece(tile, piece, c, None);
            }
        }
        attacks
    }
    /// Generates the tiles that the given colours king cannot move into
    pub fn generate_king_danger(&self, colour: Colour) -> Bitboard {
        let (_, attacker) = self.get_players(colour);
        let mut attacks = Bitboard::EMPTY;
        for tile in attacker.pieces {
            if let Some((piece, _)) = self.get_piece_at_tile(tile) {
                attacks |= self.generate_attacks_from_piece(tile, piece, !colour, Some(colour));
            }
        }
        attacks
    }

    /// Generates a bitboard representing all the tiles that a specific tile attacks
    pub fn generate_attacks_from(&self, tile: Tile) -> Bitboard {
        match self.get_piece_at_tile(tile) {
            Some((piece, colour)) => self.generate_attacks_from_piece(tile, piece, colour, None),
            None => Bitboard::EMPTY,
        }
    }

    /// Generates the attacked tiles for a specific piece type on a specific tile
    pub fn generate_attacks_from_piece(
        &self,
        tile: Tile,
        piece: Piece,
        colour: Colour,
        exclude_king: Option<Colour>,
    ) -> Bitboard {
        match piece {
            Piece::Pawn => tile.pawn_attacks(colour),
            Piece::Knight => tile.knight_attacks(),
            Piece::Bishop => self.generate_sliding_attacks(tile, false, true, exclude_king),
            Piece::Rook => self.generate_sliding_attacks(tile, true, false, exclude_king),
            Piece::Queen => self.generate_sliding_attacks(tile, true, true, exclude_king),
            Piece::King => tile.king_attacks(),
        }
    }
    
    /// Generates a bitboard representing the orthogonal and/or diagonal attacks for a specific tile
    pub fn generate_sliding_attacks(
        &self,
        tile: Tile,
        straight: bool,
        diagonal: bool,
        exclude_king: Option<Colour>,
    ) -> Bitboard {
        let occ = match exclude_king {
            None => self.occupied(),
            Some(colour) => {
                self.occupied()
                    & if colour.white() {
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