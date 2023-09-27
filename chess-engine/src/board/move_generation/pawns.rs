use crate::board::{Board, BLACK, get_ones_indices, Move, WHITE};
use crate::board::masks::{A_FILE, H_FILE, FIFTH_RANK, SECOND_RANK, SEVENTH_RANK, FOURTH_RANK};
use crate::board::moves::{construct_move, pop_lsb_idx, PromotionPiece, SpecialMove};
use crate::board::pieces::PieceType;
use crate::board::pieces::PieceType::Pawn;

pub fn get_pawn_moves(board: &Board) -> Vec<Move> {
    return if board.black_to_move { get_black_pawn_moves(board) } else { get_white_pawn_moves(board) };
}

pub fn get_white_pawn_moves(board: &Board) -> Vec<Move> {
    let white_pawns = board.pieces_bb[WHITE][PieceType::Pawn as usize];
    let white_pieces = white_pawns
        | board.pieces_bb[WHITE][PieceType::Knight as usize]
        | board.pieces_bb[WHITE][PieceType::Bishop as usize]
        | board.pieces_bb[WHITE][PieceType::Rook as usize]
        | board.pieces_bb[WHITE][PieceType::Queen as usize]
        | board.pieces_bb[WHITE][PieceType::King as usize];

    let black_pieces = board.pieces_bb[1][PieceType::Pawn as usize]
        | board.pieces_bb[BLACK][PieceType::Knight as usize]
        | board.pieces_bb[BLACK][PieceType::Bishop as usize]
        | board.pieces_bb[BLACK][PieceType::Rook as usize]
        | board.pieces_bb[BLACK][PieceType::Queen as usize]
        | board.pieces_bb[BLACK][PieceType::King as usize];

    let mut pawn_moves: Vec<Move> = Vec::new();

    // TODO might be faster without Vec heap usage, use fixed array instead?

    // push 1 square
    {
        let pushed_bitboard = white_pawns << 8;
        let squares = pushed_bitboard & !(white_pieces | black_pieces);
        let pawn_squares = get_ones_indices(&squares);

        pawn_squares.iter().for_each(|&dest| {
            let origin = dest - 8;
            if dest < 56 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            else {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    // push 2 squares
    {
        let eligible_pawns = white_pawns & SECOND_RANK;
        let pushed_bitboard = eligible_pawns << 8;
        let pushed_2_bitboard = eligible_pawns << 16;
        let squares = pushed_2_bitboard & !(white_pieces | black_pieces)
            & ((pushed_bitboard & !(white_pieces | black_pieces)) << 8);

        let pawn_squares = get_ones_indices(&squares);

        // TODO probably can by optimized
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest - 16;
            pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
        });
    }

    // attack left
    {
        let eligible_pawns = white_pawns & !A_FILE;
        let pawn_attack_squares = (eligible_pawns << 7) & black_pieces;
        let pawn_squares = get_ones_indices(&pawn_attack_squares);
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest - 7;

            if dest < 56 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            else {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    // attack right
    {
        let eligible_pawns = white_pawns & !H_FILE;
        let pawn_attack_squares = eligible_pawns << 9 & black_pieces;
        let pawn_squares = get_ones_indices(&pawn_attack_squares);
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest - 9;

            if dest < 56 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            else {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    // en passant
    {
        if (board.en_passant != 0) {
            let eligible_pawns = white_pawns & FIFTH_RANK;
            let attacks_from_ep_square = (board.en_passant & !A_FILE) >> 9 | (board.en_passant & !H_FILE) >> 7 ;
            let squares = eligible_pawns & attacks_from_ep_square;
            let pawn_squares = get_ones_indices(&squares);
            let dest = board.en_passant.trailing_zeros() as u8;
            pawn_squares.iter().for_each(|&origin| {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::EnPassant));
            });
        }
    }

    return pawn_moves;
}

pub fn get_black_pawn_moves(board: &Board) -> Vec<Move> {
    let black_pawns = board.pieces_bb[BLACK][PieceType::Pawn as usize];

    let white_pieces = board.pieces_bb[WHITE][PieceType::Pawn as usize]
        | board.pieces_bb[WHITE][PieceType::Knight as usize]
        | board.pieces_bb[WHITE][PieceType::Bishop as usize]
        | board.pieces_bb[WHITE][PieceType::Rook as usize]
        | board.pieces_bb[WHITE][PieceType::Queen as usize]
        | board.pieces_bb[WHITE][PieceType::King as usize];

    let black_pieces = black_pawns
        | board.pieces_bb[BLACK][PieceType::Knight as usize]
        | board.pieces_bb[BLACK][PieceType::Bishop as usize]
        | board.pieces_bb[BLACK][PieceType::Rook as usize]
        | board.pieces_bb[BLACK][PieceType::Queen as usize]
        | board.pieces_bb[BLACK][PieceType::King as usize];

    let mut pawn_moves: Vec<Move> = Vec::new();

    // TODO might be faster without Vec heap usage, use fixed array instead?

    // push 1 square
    {
        let pushed_bitboard = black_pawns >> 8;
        let squares = pushed_bitboard & !(white_pieces | black_pieces);
        let pawn_squares = get_ones_indices(&squares);

        pawn_squares.iter().for_each(|&dest| {
            let origin = dest + 8;

            if dest > 7 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            else{
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    // push 2 squares
    {
        let eligible_pawns = black_pawns & SEVENTH_RANK;
        let pushed_bitboard = eligible_pawns >> 8;
        let pushed_2_bitboard = eligible_pawns >> 16;
        let squares = pushed_2_bitboard & !(white_pieces | black_pieces)
            & ((pushed_bitboard & !(white_pieces | black_pieces)) >> 8);

        let pawn_squares = get_ones_indices(&squares);

        // TODO probably can by optimized
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest + 16;
            pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
        });
    }

    // attack left
    {
        let eligible_pawns = black_pawns & !A_FILE;

        let pawn_attack_squares = eligible_pawns >> 9 & white_pieces;
        let pawn_squares = get_ones_indices(&pawn_attack_squares);
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest + 9;

            if dest > 7 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            else {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    // attack right
    {
        let eligible_pawns = black_pawns & !H_FILE;
        let pawn_attack_squares = eligible_pawns >> 7 & white_pieces;
        let pawn_squares = get_ones_indices(&pawn_attack_squares);
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest + 7;

            if dest > 7 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            else {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    // en passant
    {
        if board.en_passant != 0 {
            let eligible_pawns = black_pawns & FOURTH_RANK;
            let attacks_from_ep_square = (board.en_passant & !A_FILE) << 9 | (board.en_passant & !H_FILE) << 7 ;
            let squares = eligible_pawns & attacks_from_ep_square;
            let dest = board.en_passant.trailing_zeros() as u8;
            let pawn_squares = get_ones_indices(&squares);
            pawn_squares.iter().for_each(|&origin| {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::EnPassant));
            });
        }
    }

    return pawn_moves;
}


pub fn get_pawn_attacks(board: &Board, color: usize) -> u64 {
    return if color == BLACK {
        let eligible_pawns_left = board.pieces_color_bb[BLACK] & !H_FILE;
        let eligible_pawns_right = board.pieces_color_bb[BLACK] & !A_FILE;

        let pawn_attack_squares_left = eligible_pawns_left >> 9 & board.pieces_color_bb[WHITE];
        let pawn_attack_squares_right = eligible_pawns_right >> 7 & board.pieces_color_bb[WHITE];

        pawn_attack_squares_left | pawn_attack_squares_right
    } else {
        let eligible_pawns_left = board.pieces_color_bb[WHITE] & !A_FILE;
        let eligible_pawns_right = board.pieces_bb[WHITE][Pawn as usize] & !H_FILE;

        let pawn_attack_squares_left = eligible_pawns_left << 7 & board.pieces_color_bb[BLACK];
        let pawn_attack_squares_right = eligible_pawns_right << 9 & board.pieces_color_bb[BLACK];

        pawn_attack_squares_left | pawn_attack_squares_right
    };
}