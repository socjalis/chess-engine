use crate::board::{BLACK, Board, WHITE};
use crate::board::pieces::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

const PIECE_VALUES: [i32; 6] = [
    1000,
    3000,
    3100,
    5000,
    9000,
    213700
];

pub fn get_material_eval(board: &Board) -> i32 {
    let mut eval = 0i32;
    eval += board.pieces_bb[WHITE][Pawn as usize].count_ones() as i32 * PIECE_VALUES[Pawn as usize];
    eval += board.pieces_bb[WHITE][Knight as usize].count_ones() as i32 * PIECE_VALUES[Knight as usize];
    eval += board.pieces_bb[WHITE][Bishop as usize].count_ones() as i32 * PIECE_VALUES[Bishop as usize];
    eval += board.pieces_bb[WHITE][Rook as usize].count_ones() as i32 * PIECE_VALUES[Rook as usize];
    eval += board.pieces_bb[WHITE][Queen as usize].count_ones() as i32 * PIECE_VALUES[Queen as usize];
    eval += board.pieces_bb[WHITE][King as usize].count_ones() as i32 * PIECE_VALUES[King as usize];

    eval -= board.pieces_bb[BLACK][Pawn as usize].count_ones() as i32 * PIECE_VALUES[Pawn as usize];
    eval -= board.pieces_bb[BLACK][Knight as usize].count_ones() as i32 * PIECE_VALUES[Knight as usize];
    eval -= board.pieces_bb[BLACK][Bishop as usize].count_ones() as i32 * PIECE_VALUES[Bishop as usize];
    eval -= board.pieces_bb[BLACK][Rook as usize].count_ones() as i32 * PIECE_VALUES[Rook as usize];
    eval -= board.pieces_bb[BLACK][Queen as usize].count_ones() as i32 * PIECE_VALUES[Queen as usize];
    eval -= board.pieces_bb[BLACK][King as usize].count_ones() as i32 * PIECE_VALUES[King as usize];


    return eval;
}