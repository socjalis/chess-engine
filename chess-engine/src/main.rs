use bitvec::view::BitViewSized;
use crate::board::fen::create_bitboard_from_fen;
use crate::board::masks::{H1A8, JESUS45, NE};
use crate::board::move_generation::attacks::rook::{get_bishop_attacks, get_bishop_idx, get_rook_attacks, initiate_slider_attacks};
use crate::board::moves::{format_move, pop_lsb_idx};
use crate::board::print_bb;

extern crate lazy_static;
mod board;

fn main() {
    let mut board = create_bitboard_from_fen("rnbqkbnr/p2p1pp1/2p5/1p2p2p/4P2P/PP6/2PP1PP1/RNBQKBNR w KQkq - 0 5");
    board.print();

    initiate_slider_attacks();

    let moves = board.get_legal_moves();
    moves.iter().for_each(|mov1| {
        println!("{}", format_move(mov1));
    });
    //
    // //
    // // print_bb(11637303775739249200u64.wrapping_mul(2));
    // // print_bb(11637303775739249200u64.wrapping_mul(4));
    //
    // let occupancy = board.relevant_occupancy(26);
    //
    // print_bb(occupancy);
    //
    // let attacks = get_rook_attacks(26, occupancy);
    //
    // print_bb(attacks);

    // print_bb(get_bishop_attacks(26, board.occupancy()));

    // println!("{:?}", ROOK_ATTACKS[0][0][0]);

    // println!("{}", format_move(&mov));
    //
    // board.make_move(mov);

    // board.print();

    // board::create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}