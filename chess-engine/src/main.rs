use bitvec::view::BitViewSized;
use crate::board::fen::create_bitboard_from_fen;
use crate::board::moves::format_move;

mod board;

fn main() {
    let mut board = create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    board.print();

    let mov = board.get_legal_white_pawn_moves()[2];
    // moves.iter().for_each(|mov1| {
    //     println!("{}", format_move(mov1));
    // });

    println!("{}", format_move(&mov));

    board.make_move(mov);

    board.print();

    // board::create_bitboard_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}