use crate::board::{BitBoard, get_ones_indices, Move};
use crate::board::moves::{construct_move, PromotionPiece, SpecialMove};
use crate::board::move_generation::attacks::knight::KNIGHT_ATTACKS;
use crate::board::pieces::PieceType;

pub fn get_knight_moves(board: &BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let color = board.black_to_move as usize;
    let current_knights = board.pieces_bb[color][PieceType::Knight as usize];
    let current_pieces = board.pieces_bb[color][PieceType::Pawn as usize]
        | board.pieces_bb[color][PieceType::Knight as usize]
        | board.pieces_bb[color][PieceType::Bishop as usize]
        | board.pieces_bb[color][PieceType::Rook as usize]
        | board.pieces_bb[color][PieceType::Queen as usize]
        | board.pieces_bb[color][PieceType::King as usize];

    let knight_squares = get_ones_indices(&current_knights);
    
    knight_squares.iter().for_each(|&origin| {
        let valid_attack_squares = KNIGHT_ATTACKS[origin as usize] & !current_pieces;
        let attack_squares = get_ones_indices(&valid_attack_squares);
        attack_squares.iter().for_each(|&dest| {
            moves.push(construct_move(dest, origin, PromotionPiece::Knight, SpecialMove::None));
        });

    });
    

    return moves;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_bitboard_from_fen;

    #[test]
    fn test_get_knight_moves_starting_position() {
        let board = create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let knight_moves = get_knight_moves(&board);
        assert_eq!(knight_moves.len(), 4);
    }

    #[test]
    fn test_get_knight_moves_no_moves() {
        let board = create_bitboard_from_fen("4k3/8/3R1R2/2B3B1/4N3/2K3Q1/PPPPPPPP/8 w - - 0 1");
        let knight_moves = get_knight_moves(&board);
        assert_eq!(knight_moves.len(), 0);
    }

    #[test]
    fn test_get_knight_moves_black_7_moves() {
        let board = create_bitboard_from_fen("4k3/8/3R1r2/2B3B1/4n3/2K3Q1/PPPPPPPP/8 b - - 0 1");
        let knight_moves = get_knight_moves(&board);
        assert_eq!(knight_moves.len(), 7);
    }
}
