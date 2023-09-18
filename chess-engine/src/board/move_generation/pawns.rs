use crate::board::{BitBoard, BLACK, get_ones_indices, Move, WHITE};
use crate::board::masks::{A_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK};
use crate::board::moves::{construct_move, PromotionPiece, SpecialMove};
use crate::board::pieces::PieceType;
use crate::board::pieces::PieceType::Pawn;

pub fn get_pawn_moves(board: &BitBoard) -> Vec<Move> {
    return if board.black_to_move { get_black_pawn_moves(board) } else { get_white_pawn_moves(board) };
}

pub fn get_white_pawn_moves(board: &BitBoard) -> Vec<Move> {
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
            if dest >= 56 {
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
            if dest >= 56 {
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
            if dest >= 56 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    return pawn_moves;
}

pub fn get_black_pawn_moves(board: &BitBoard) -> Vec<Move> {
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
            if dest <= 7 {
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
        let eligible_pawns = black_pawns & !H_FILE;

        let pawn_attack_squares = eligible_pawns >> 9 & white_pieces;
        let pawn_squares = get_ones_indices(&pawn_attack_squares);
        pawn_squares.iter().for_each(|&dest| {
            let origin = dest + 9;

            if dest > 7 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
            }
            if dest <= 7 {
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
            if dest <= 7 {
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Bishop, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Rook, SpecialMove::Promotion));
                pawn_moves.push(construct_move(dest, origin, PromotionPiece::Queen, SpecialMove::Promotion));
            }
        });
    }

    return pawn_moves;
}

fn get_pawn_attacks(board: &BitBoard) -> u64 {
    let eligible_pawns = board.pieces_bb[board.black_to_move as usize][Pawn as usize] & !H_FILE;
    let pawn_attack_squares_left = eligible_pawns >> 9 & board.pieces_color[WHITE];

    let eligible_pawns = board.pieces_bb[board.black_to_move as usize][Pawn as usize] & !H_FILE;
    let pawn_attack_squares_right = eligible_pawns >> 7 & board.pieces_color[WHITE];

    return pawn_attack_squares_left >> pawn_attack_squares_right;
}