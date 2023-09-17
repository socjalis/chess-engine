use bitvec::view::BitViewSized;
use crate::board::fen::create_bitboard_from_fen;
use crate::board::move_generation::attacks::rook::{get_rook_attacks, initiate_slider_attacks};
use crate::board::moves::format_move;
use crate::board::print_bb;

extern crate lazy_static;
mod board;

fn main() {
    let mut board = create_bitboard_from_fen("rn1q1bnr/ppPp1p1p/8/4pk2/8/P7/1PP1PPpP/RNBQKB1R b KQ - 0 5");
    board.print();
    //
    // let moves = board.get_legal_moves();
    // moves.iter().for_each(|mov1| {
    //     println!("{}", format_move(mov1));
    // });

    initiate_slider_attacks();
    //
    // print_bb(11637303775739249200u64.wrapping_mul(2));
    // print_bb(11637303775739249200u64.wrapping_mul(4));

    let occupancy = board.relevant_occupancy(26);

    print_bb(occupancy);

    let attacks = get_rook_attacks(26, occupancy);

    print_bb(attacks);

    // println!("{:?}", ROOK_ATTACKS[0][0][0]);

    // println!("{}", format_move(&mov));
    //
    // board.make_move(mov);

    // board.print();

    // board::create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}