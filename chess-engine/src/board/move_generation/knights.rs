use crate::board::{BitBoard, BLACK, get_ones_indices, Move, WHITE};
use crate::board::masks::{A_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK};
use crate::board::moves::{construct_move, PromotionPiece, SpecialMove};
use crate::board::pieces::PieceType;

pub fn get_knight_moves(board: &BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    return moves
}