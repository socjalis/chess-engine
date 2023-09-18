use std::num::FpCategory::Normal;
use bitvec::array::BitArray;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::order::Msb0;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use bitvec::view::BitViewSized;
use crate::board::{Move, print_bb};
use crate::board::moves::PromotionPiece::Knight;
use crate::board::pieces::PieceType;
use crate::board::squares::SQUARES;

#[derive(PartialEq)]
pub enum PromotionPiece {
    Knight = 0,
    Bishop = 1,
    Rook = 2,
    Queen = 3,
}

#[derive(PartialEq)]
pub enum SpecialMove {
    None = 0,
    Promotion = 1,
    Castle = 2,
    EnPassant = 3,
}

// 0-5 - destination
// 6-11 - origin
// 12-13 - promotion piece type (0-Knight, 1-Bishop, 2-Rook, 3-Queen)
// 14-15 - special move flag (1-promotion, 2-en passant, 3-castling)
pub fn construct_move(dest: u8, origin: u8, promotion_piece: PromotionPiece, special_move_flag: SpecialMove) -> u16 {
   let x = ((dest as u16) << 10)
       + ((origin as u16) << 4)
       + ((promotion_piece as u16) << 2)
       + special_move_flag as u16;

    return x;
}


pub fn pop_lsb_idx(number: &mut u64) -> u64 {
    let idx = number.trailing_zeros() as u64;
    *number ^= 1 << idx;

    return idx;
}

pub fn generate_moves(square: u64, attacks: u64, current_pieces: u64) -> Vec<Move> {
    let mut piece_attacks = attacks & !current_pieces;
    let mut moves: Vec<Move> = Vec::new();

    while piece_attacks != 0 {
        let dest_idx = pop_lsb_idx(&mut piece_attacks) as u8;
        moves.push(construct_move(dest_idx, square as u8, Knight, SpecialMove::None));
    }

    return moves;
}
pub fn get_moves_for_piece_type(pieces_bb: u64, attacks: [u64; 64], current_pieces: u64) -> Vec<Move> {
    let mut pieces = pieces_bb;
    let mut moves: Vec<Move> = Vec::new();

    while pieces != 0 { let piece_idx = pop_lsb_idx(&mut pieces);
       moves.append(&mut generate_moves(piece_idx, attacks[piece_idx as usize], current_pieces));
    }

    return moves;
}

pub fn get_slider_moves(pieces_bb: u64, get_attacks: fn(square: u64, occupancy: u64) -> u64, occupancy: u64, current_pieces: u64) -> Vec<Move> {
    let mut pieces = pieces_bb;
    let mut moves: Vec<Move> = Vec::new();

    while pieces != 0 {
        let piece_idx = pop_lsb_idx(&mut pieces);
        let piece_attacks = get_attacks(piece_idx, occupancy);

        println!("piece idx {}", piece_idx);
        print_bb(piece_attacks);

        moves.append(&mut generate_moves(piece_idx, piece_attacks, current_pieces));
    }

    return moves;
}

pub fn format_move(mov: &Move) -> String {
    let bits: BitArray<u16, Msb0> = mov.into_bitarray();
    let dest = &bits[0..6];
    let origin = &bits[6..12];
    let promotion_piece = &bits[12..14];
    let special_move_flag = &bits[14..15];

    let dest = bool_vec_to_number(dest.to_bitvec()) as usize;
    let origin = bool_vec_to_number(origin.to_bitvec()) as usize;

    return format!("from: {}, to: {}", SQUARES[origin], SQUARES[dest]);
}

pub fn bool_vec_to_number<T: BitStore>(bits: BitVec<T, Msb0>) -> u32 {
    let a: u32 = bits.iter()
        .filter(|bit| *bit == true || *bit == false)
        .fold(0, |result, bit| (result << 1) ^ bit.as_u32());
    return a;
}