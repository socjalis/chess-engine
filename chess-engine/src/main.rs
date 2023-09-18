use bitvec::view::BitViewSized;
use crate::board::fen::create_bitboard_from_fen;
use crate::board::masks::{H1A8, JESUS45, NE};
use crate::board::move_generation::attacks::rook::{get_bishop_attacks, get_bishop_idx, get_rook_attacks, initiate_slider_attacks};
use crate::board::moves::format_move;
use crate::board::print_bb;

extern crate lazy_static;
mod board;

fn main() {
    let mut board = create_bitboard_from_fen("rnbqkbnr/pppp2pp/4pp2/8/2B1P3/3P4/PPP2PPP/RNBQK1NR b KQkq - 0 3");
    board.print();
    // //
    // // let moves = board.get_legal_moves();
    // // moves.iter().for_each(|mov1| {
    // //     println!("{}", format_move(mov1));
    // // });
    //
    initiate_slider_attacks();
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

    print_bb(get_bishop_attacks(26, board.occupancy()));

    // println!("{:?}", ROOK_ATTACKS[0][0][0]);

    // println!("{}", format_move(&mov));
    //
    // board.make_move(mov);

    // board.print();

    // board::create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}