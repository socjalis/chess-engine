use crate::board::{BLACK, Board, Move, WHITE};
use crate::board::moves::construct_move;
use crate::board::moves::PromotionPiece::Knight;
use crate::board::moves::SpecialMove::Castle;

const BLACK_O_O_SQUARES: u64 = 1 << 61 | 1 << 62;
const BLACK_O_O_0_NOT_ATTACKED: u64 = 1 << 58 | 1 << 59;
const BLACK_O_O_0_NOT_OCCUPIED: u64 = 1 << 57 | 1 << 58 | 1 << 59;

const WHITE_O_O_SQUARES: u64 = 1 << 5 | 1 << 6;
const WHITE_O_O_O_NOT_ATTACKED: u64 = 1 << 2 | 1 << 3;
const WHITE_O_O_O_NOT_OCCUPIED: u64 = 1 << 1 | 1 << 2 | 1 << 3;

pub fn get_castling_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    if board.black_to_move {
        if board.black_o_o
            && board.attacks_color_bb[WHITE] & BLACK_O_O_SQUARES & board.occupancy() == 0
        {
            moves.push(construct_move(62, 60, Knight, Castle));
        }

        if board.black_o_o
            && board.attacks_color_bb[WHITE] & BLACK_O_O_0_NOT_ATTACKED == 0
            && board.occupancy() & BLACK_O_O_0_NOT_OCCUPIED == 0
        {
            moves.push(construct_move(58, 60, Knight, Castle));
        }
    } else {
        if board.white_o_o
            && board.attacks_color_bb[BLACK] & WHITE_O_O_SQUARES & board.occupancy() == 0
        {
            moves.push(construct_move(6, 4, Knight, Castle));
        }

        if board.white_o_o_o
            && board.attacks_color_bb[BLACK] & WHITE_O_O_O_NOT_ATTACKED == 0
            && board.occupancy() & WHITE_O_O_O_NOT_OCCUPIED == 0
        {
            moves.push(construct_move(2, 4, Knight, Castle));
        }
    }

    return moves;
}