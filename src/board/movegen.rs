use std::collections::HashMap;

use crate::{Bitboard, Board, CastlingRights, MoveList, Piece, Tile};

impl Board {
    #[inline]
    pub fn generate_legal_moves(&self, white: bool, moves: &mut MoveList) {
        let (player, _) = self.get_players(white);
        let checkers = self.get_checkers(white);
        let checkers_count = checkers.count_ones();

        self.generate_king_moves(player.king_tile(), white, moves);

        if checkers_count > 1 {
            return; // Double check — only king moves allowed
        }

        let mut targets: Option<Bitboard> = None;
        if checkers_count == 1 {
            if let Some(pos) = checkers.to_bit() {
                let checker = self.get_piece_at_tile(pos).unwrap();
                let between = if matches!(checker.0, Piece::Bishop | Piece::Rook | Piece::Queen) {
                    pos.get_between(player.king_tile())
                } else {
                    Bitboard::EMPTY
                };
                let mut valid_targets = Bitboard::EMPTY;
                valid_targets.set_bit(pos, true);
                valid_targets |= between;
                targets = Some(valid_targets);
            }
        }
        let pinned = self.get_pinned_pieces(white);

        for pawn in player.bb[Piece::Pawn as usize] {
            let mask = combine_masks(pinned.get(&pawn).copied(), targets);
            self.generate_pawn_moves(pawn, white, mask, moves);
        }
        for knight in player.bb[Piece::Knight as usize] {
            if pinned.contains_key(&knight) {
                continue;
            }
            let mask = targets;
            self.generate_knight_moves(knight, white, mask, moves);
        }
        for bishop in player.bb[Piece::Bishop as usize] {
            let mask = combine_masks(pinned.get(&bishop).copied(), targets);
            self.generate_sliding_moves(bishop, white, false, true, mask, moves);
        }
        for rook in player.bb[Piece::Rook as usize] {
            let mask = combine_masks(pinned.get(&rook).copied(), targets);
            self.generate_sliding_moves(rook, white, true, false, mask, moves);
        }
        for queen in player.bb[Piece::Queen as usize] {
            let mask = combine_masks(pinned.get(&queen).copied(), targets);
            self.generate_sliding_moves(queen, white, true, true, mask, moves);
        }
    }

    pub fn generate_legal_moves_from(&self, tile: Tile, moves: &mut MoveList) {
        let (piece, white) = match self.get_piece_at_tile(tile) {
            Some(p) => p,
            _ => return,
        };

        let (player, _) = self.get_players(white);
        let checkers = self.get_checkers(white);
        let checkers_count = checkers.count_ones();

        if checkers_count > 1 && piece != Piece::King {
            return;
        }

        // Check if this piece is pinned
        let pinned = self.get_pinned_pieces(white);

        // Determine targets due to a single check
        let check_targets = if checkers_count == 1 {
            if let Some(checker_pos) = checkers.to_bit() {
                let checker_piece = self.get_piece_at_tile(checker_pos).unwrap();
                let mut mask = Bitboard::EMPTY;
                mask.set_bit(checker_pos, true);
                if matches!(checker_piece.0, Piece::Bishop | Piece::Rook | Piece::Queen) {
                    mask |= player.king_tile().get_between(checker_pos)
                }
                Some(mask)
            } else {
                None
            }
        } else {
            None
        };
        let mask = combine_masks(pinned.get(&tile).copied(), check_targets);

        match piece {
            Piece::Pawn => self.generate_pawn_moves(tile, white, mask, moves),
            Piece::Knight => {
                if pinned.contains_key(&tile) {
                    return;
                }
                self.generate_knight_moves(tile, white, mask, moves);
            }
            Piece::Bishop => self.generate_sliding_moves(tile, white, false, true, mask, moves),
            Piece::Rook => self.generate_sliding_moves(tile, white, true, false, mask, moves),
            Piece::Queen => self.generate_sliding_moves(tile, white, true, true, mask, moves),
            Piece::King => self.generate_king_moves(tile, white, moves),
        }
    }

    fn get_checkers(&self, white: bool) -> Bitboard {
        let (player, attacker) = self.get_players(white);
        let occ = self.occupied();
        let mut checkers = Bitboard::EMPTY;
        let kt = player.king_tile();

        let straight_mask = kt.rook_attacks(occ);
        let diag_mask = kt.bishop_attacks(occ);
        let knight_mask = kt.knight_attacks();
        let pawn_mask = kt.pawn_attacks(white);

        checkers |= attacker.bb[Piece::Rook as usize] & straight_mask;
        checkers |= attacker.bb[Piece::Bishop as usize] & diag_mask;
        checkers |= attacker.bb[Piece::Queen as usize] & (diag_mask | straight_mask);
        checkers |= attacker.bb[Piece::Knight as usize] & knight_mask;
        checkers |= attacker.bb[Piece::Pawn as usize] & pawn_mask;

        checkers
    }
    fn get_pinned_pieces(&self, white: bool) -> HashMap<Tile, Bitboard> {
        use crate::Piece::{Bishop, Queen, Rook};
        let mut pins = HashMap::new();

        let (player, _) = self.get_players(white);
        let kt = player.king_tile();
        let occ = self.occupied();

        let directions: [(i8, i8); 8] = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, -1),
            (-1, 1),
        ];

        for (dx, dy) in directions {
            let mut current = kt;
            let mut maybe_pinned: Option<Tile> = None;

            while let Some(next) = current.offset(dx, dy) {
                current = next;

                if !occ.get_bit(current) {
                    continue;
                }

                if player.pieces.get_bit(current) {
                    if maybe_pinned.is_none() {
                        maybe_pinned = Some(current);
                    } else {
                        break; // second friendly piece: no pin
                    }
                } else {
                    if let Some(pinned_tile) = maybe_pinned {
                        if let Some((ptype, _)) = self.get_piece_at_tile(current) {
                            let is_diag = dx.abs() == dy.abs();
                            let is_straight = dx == 0 || dy == 0;

                            if ptype == Queen
                                || (ptype == Rook && is_straight)
                                || (ptype == Bishop && is_diag)
                            {
                                pins.insert(
                                    pinned_tile,
                                    kt.get_between(current) | current.to_mask(),
                                );
                            }
                        }
                    }
                    break;
                }
            }
        }

        pins
    }

    #[inline]
    pub fn generate_pawn_moves(
        &self,
        tile: Tile,
        white: bool,
        targets: Option<Bitboard>,
        moves: &mut MoveList,
    ) {
        if let Some(one_step) = tile.forward(white) {
            if !self.occupied().get_bit(one_step) {
                if targets.map_or(true, |m: Bitboard| m.get_bit(one_step)) {
                    self.try_push_pawn_move(tile, one_step, white, None, moves);
                }

                if tile.is_pawn_start(white) {
                    if let Some(two_step) = one_step.forward(white) {
                        if !self.occupied().get_bit(two_step) {
                            if targets.map_or(true, |m| m.get_bit(two_step)) {
                                moves.push(self.create_move(
                                    tile,
                                    two_step,
                                    Piece::Pawn,
                                    None,
                                    None,
                                ));
                            }
                        }
                    }
                }
            }
        }

        for maybe_target in [
            tile.left(white).and_then(|t| t.forward(white)),
            tile.right(white).and_then(|t| t.forward(white)),
        ] {
            if let Some(to) = maybe_target {
                if Some(to) == self.en_passant
                    && targets.map_or(true, |m| m.get_bit(to.backward(white).unwrap()))
                {
                    if let Some(_) = if white {
                        to.offset(0, -1)
                    } else {
                        to.offset(0, 1)
                    } {
                        let king_tile = if white {
                            self.white.bb[Piece::King as usize].to_bit().unwrap()
                        } else {
                            self.black.bb[Piece::King as usize].to_bit().unwrap()
                        };

                        let occupied = self.occupied();
                        let (_, opponent) = self.get_players(white);
                        let enemy_sliders =
                            opponent.bb[Piece::Rook as usize] | opponent.bb[Piece::Queen as usize];

                        if !Self::is_illegal_en_passant_discovery(
                            tile,
                            to,
                            king_tile,
                            occupied,
                            enemy_sliders,
                        ) {
                            moves.push(self.create_move(
                                tile,
                                to,
                                Piece::Pawn,
                                Some(Piece::Pawn),
                                None,
                            ));
                        }
                    }
                }

                if self.is_square_occupied_by_enemy(to, white)
                    && targets.map_or(true, |m| m.get_bit(to))
                {
                    let captured = self.get_piece_at_tile(to).map(|(p, _)| p);
                    self.try_push_pawn_move(tile, to, white, captured, moves);
                }
            }
        }
    }
    /// Returns true if en passant would expose a discovered check on the king.
    /// `tile`: the capturing pawn tile
    /// `to`: the en passant destination tile (behind the captured pawn)
    /// `king_tile`: your king's tile
    /// `occupied`: all occupied tiles
    /// `opponent_sliders`: enemy rooks and queens
    pub fn is_illegal_en_passant_discovery(
        capturing_pawn_tile: Tile,
        ep_target_tile: Tile,
        king_tile: Tile,
        occupied: Bitboard,
        opponent_sliders: Bitboard,
    ) -> bool {
        let (king_file, king_rank) = king_tile.get_coords();
        let (_, pawn_rank) = capturing_pawn_tile.get_coords();
        let (_, ep_rank) = ep_target_tile.get_coords();

        // Check if king is on same rank as the capturing pawn (en passant only matters horizontally)
        if king_rank != pawn_rank {
            return false;
        }

        // Determine the square of the captured pawn (behind en passant square)
        let captured_pawn_tile = if ep_rank > pawn_rank {
            ep_target_tile.offset(0, -1)
        } else {
            ep_target_tile.offset(0, 1)
        };

        let Some(captured_pawn_tile) = captured_pawn_tile else {
            return false;
        };

        // Create a new occupancy bitboard with the capturing and captured pawns removed
        let new_occupied = {
            let mut new_occ = occupied;
            new_occ.set_bit(capturing_pawn_tile, false);
            new_occ.set_bit(captured_pawn_tile, false);
            new_occ
        };

        // Now scan along the rank for a rook or queen that attacks the king
        for dx in [-1, 1] {
            let mut x = king_file as i8 + dx;
            while (0..8).contains(&x) {
                let tile = Tile::new_xy(x as u8, king_rank).unwrap();
                if new_occupied.get_bit(tile) {
                    if opponent_sliders.get_bit(tile) {
                        return true; // Discovered attack after EP
                    } else {
                        break; // Blocked
                    }
                }
                x += dx;
            }
        }

        false
    }

    #[inline]
    fn try_push_pawn_move(
        &self,
        from: Tile,
        to: Tile,
        white: bool,
        capture: Option<Piece>,
        moves: &mut MoveList,
    ) {
        if to.is_promotion(white) {
            for promo in [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
                moves.push(self.create_move(from, to, Piece::Pawn, capture, Some(promo)));
            }
        } else {
            moves.push(self.create_move(from, to, Piece::Pawn, capture, None));
        }
    }

    #[inline]
    fn generate_knight_moves(
        &self,
        tile: Tile,
        white: bool,
        targets: Option<Bitboard>,
        moves: &mut MoveList,
    ) {
        let mut attacks = tile.knight_attacks();
        if let Some(mask) = targets {
            attacks &= mask;
        }
        let friendly_mask = if white {
            self.white.pieces
        } else {
            self.black.pieces
        };

        attacks &= !friendly_mask;
        for to in attacks {
            moves.push(self.create_move(
                tile,
                to,
                Piece::Knight,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }
    }

    #[inline]
    fn generate_sliding_moves(
        &self,
        tile: Tile,
        white: bool,
        straight: bool,
        diagonal: bool,
        targets: Option<Bitboard>,
        moves: &mut MoveList,
    ) {
        let mut attacks = self.generate_sliding_attacks(tile, straight, diagonal, None);
        if let Some(mask) = targets {
            attacks &= mask;
        }
        let friendly_mask = if white {
            self.white.pieces
        } else {
            self.black.pieces
        };
        attacks &= !friendly_mask;

        let piece = self.get_piece_at_tile(tile).unwrap().0;
        for to in attacks {
            moves.push(self.create_move(
                tile,
                to,
                piece,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }
    }

    #[inline]
    fn generate_king_moves(&self, tile: Tile, white: bool, moves: &mut MoveList) {
        let mut attacks = tile.king_attacks();
        let friendly_mask = if white {
            self.white.pieces
        } else {
            self.black.pieces
        };
        let attack_mask = self.generate_king_danger(!white);
        // let attack_mask = self.generate_attacks(!white);

        attacks &= !friendly_mask & !attack_mask;

        for to in attacks {
            moves.push(self.create_move(
                tile,
                to,
                Piece::King,
                self.get_piece_at_tile(to).map(|(p, _)| p),
                None,
            ));
        }

        if attack_mask.get_bit(tile) {
            return; // Can't castle out of, through, or into check
        }

        let castling_rights = self.castling;
        let occupied = self.occupied();

        // macro_rules! castle_check {
        //     ($cond:expr, $mask:expr, $target:expr) => {
        //         if $cond && (occupied & $mask).none() && (attack_mask & $mask).none() {
        //             moves.push(self.create_move(tile, $target, Piece::King, None, None));
        //         }
        //     };
        // }

        if white {
            // Kingside: check F1 and G1 are empty and not attacked
            if castling_rights.contains(CastlingRights::WHITE_KINGSIDE)
                && (occupied & (Tile::F1.to_mask() | Tile::G1.to_mask())).none()
                && (attack_mask & (Tile::F1.to_mask() | Tile::G1.to_mask())).none()
            {
                moves.push(self.create_move(tile, Tile::G1, Piece::King, None, None));
            }

            // Queenside: D1, C1 must be empty and not attacked, B1 must not have *any* piece (including friendly)
            if castling_rights.contains(CastlingRights::WHITE_QUEENSIDE)
                && (occupied & (Tile::D1.to_mask() | Tile::C1.to_mask() | Tile::B1.to_mask()))
                    .none()
                && (attack_mask & (Tile::D1.to_mask() | Tile::C1.to_mask())).none()
            {
                moves.push(self.create_move(tile, Tile::C1, Piece::King, None, None));
            }
        } else {
            // Kingside
            if castling_rights.contains(CastlingRights::BLACK_KINGSIDE)
                && (occupied & (Tile::F8.to_mask() | Tile::G8.to_mask())).none()
                && (attack_mask & (Tile::F8.to_mask() | Tile::G8.to_mask())).none()
            {
                moves.push(self.create_move(tile, Tile::G8, Piece::King, None, None));
            }

            // Queenside
            if castling_rights.contains(CastlingRights::BLACK_QUEENSIDE)
                && (occupied & (Tile::D8.to_mask() | Tile::C8.to_mask() | Tile::B8.to_mask()))
                    .none()
                && (attack_mask & (Tile::D8.to_mask() | Tile::C8.to_mask())).none()
            {
                moves.push(self.create_move(tile, Tile::C8, Piece::King, None, None));
            }
        }
    }

    #[inline]
    fn is_square_occupied_by_enemy(&self, square: Tile, white: bool) -> bool {
        let (_, opponent) = self.get_players(white);
        opponent.pieces.get_bit(square)
    }
}
fn combine_masks(a: Option<Bitboard>, b: Option<Bitboard>) -> Option<Bitboard> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a & b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}
