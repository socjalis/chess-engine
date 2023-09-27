use bitvec::view::BitViewSized;
use crate::board::fen::create_bitboard_from_fen;
use crate::board::move_generation::attacks::rook::{initiate_slider_attacks};
use crate::board::moves::{format_move};

extern crate lazy_static;
mod board;

fn main() {
    let mut board = create_bitboard_from_fen("r3k2r/pppp1ppp/2n5/4p3/2B1P3/2n2N2/PPPP1PPP/R3K2R w KQkq - 4 4");
    board.print();

    initiate_slider_attacks();

    let moves = board.get_legal_moves();
    moves.iter().for_each(|mov1| {
        println!("{}", format_move(mov1));
    });

    println!("{}", board.static_eval());
}