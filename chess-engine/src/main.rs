use bitvec::bitbox;
use crate::board::{get_ones_indices};
use crate::board::fen::create_bitboard_from_fen;

mod board;

fn printlol(idx: u32){
    println!("dupa {}", idx);
}

fn main() {
    println!("Witaj Andrzeju {}", '😻');
    // println!("{}", 2_u64.trailing_zeros());
    // let bits = get_ones_indices(&31_u64);
    // println!("{:?}", bits);
    create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").get_legal_white_pawn_moves();

    // println!("{:?}", bitBoard.get_legal_white_pawn_moves())
    // board::create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}