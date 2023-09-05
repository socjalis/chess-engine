use std::fmt::format;
use crate::board::moves::{construct_move, PromotionPiece, SpecialMove};

pub mod squares;
mod pieces;
pub mod moves;
pub mod fen;

type Move = u16;

pub trait Board {
    fn initialize(fen: &str);
    fn get_legal_moves() -> [(char, char); 1];
    fn get_pieces() -> [char; 1];
}

pub struct BitBoard {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,

    white_o_o: bool,
    white_o_o_o: bool,
    black_o_o: bool,
    black_o_o_o: bool,

    en_passant: u64,

    white_to_move: bool,

    half_moves: u16,
    full_moves: u16,
}

impl Board for BitBoard {
    fn initialize(fen: &str) {
        let splitted = fen.split("/");
        for part in splitted {
            println!("{}", part);
        }
    }
    fn get_legal_moves() -> [(char, char); 1] {
        return [('a', 'b')];
    }
    fn get_pieces() -> [char; 1] {
        return ['a'];
    }
}

// 0-5 - destination
// 6-11 - origin
// 12-13 - promotion piece type (0-Knight, 1-Bishop, 2-Rook, 3-Queen)
// 14-15 - special move flag (1-promotion, 2-en passant, 3-castling)
impl BitBoard {
    pub fn get_legal_white_pawn_moves(&self) {
        let white_pieces = self.white_king & self.white_knights & self.white_pawns & self.white_bishops & self.white_queens & self.white_rooks;
        let black_pieces = self.black_king & self.black_knights & self.black_pawns & self.black_bishops & self.black_queens & self.black_rooks;

        let white_1_pushes = (|| {
            let mask = u64::MAX >> 16;

            let eligible_pawns = self.white_pawns & mask;
            let pushed_bitboard = eligible_pawns << 8;
            let squares = pushed_bitboard & !(white_pieces | black_pieces);
            let pawn_squares = get_ones_indices(&squares);

            let moves: Vec<Move> = pawn_squares.iter().map(|&dest| {
                return construct_move(dest as u8, (dest << 3) as u8, PromotionPiece::Knight, SpecialMove::None);
            }).collect();

            // let c = moves.iter().map(|&a| {
            //     return format!("{:b}", a);
            // }).collect::<Vec<String>>();

            // println!("dupa1 {:?}", c);
        })();

        let pawn_moves: Vec<Move> = Vec::new();

        let white_2_pushes = (|| {
            let second_line_mask = u64::MAX >> 48;

            let eligible_pawns = self.white_pawns & second_line_mask;
            let pushed_bitboard = eligible_pawns << 8;
            let pushed_2_bitboard = eligible_pawns << 16;
            let squares = pushed_2_bitboard & !(white_pieces | black_pieces)
                & ((pushed_bitboard & !(white_pieces | black_pieces)) << 8);

            let pawn_squares = get_ones_indices(&squares);
            let moves: Vec<Move> = pawn_squares.iter().map(|&dest| {
                return construct_move(dest as u8, (dest << 4) as u8, PromotionPiece::Knight, SpecialMove::None);
            }).collect();
            println!("dupa2 {:?}", moves);
        })();

        fn get_2_pushes() {}
    }
}

pub fn get_ones_indices(bitboard: &u64) -> Vec<u32> {
    let mut num = *bitboard;
    let mut vec: Vec<u32> = Vec::new();
    let mut shift = 0_u32;
    while num != 0 {
        let position = num.trailing_zeros();
        num = num >> (position + 1);
        shift += position;
        vec.push(shift);
        shift += 1;
        // println!("{}", position);
    }

    return vec;
}