use std::mem::transmute;
use bitvec::macros::internal::funty::Fundamental;
use crate::board::masks::{A_FILE, H_FILE, SECOND_RANK};
use crate::board::move_generation::pawns::get_pawn_moves;
use crate::board::move_generation::knights::get_knight_moves;
use crate::board::moves::{construct_move, PromotionPiece, SpecialMove};
use crate::board::pieces::{Piece, piece_to_str, PieceType};

const WHITE: usize = 0;
const BLACK: usize = 1;

pub mod squares;
mod pieces;
pub mod moves;
pub mod fen;
pub mod masks;
pub mod move_generation;

// 0-1 - special move flag (1-promotion, 2-en passant, 3-castling)
// 2-3 - promotion piece type (0-Knight, 1-Bishop, 2-Rook, 3-Queen)
// 4-9 - origin
// 10-15 - destination
type Move = u16;

pub trait Board {
    fn initialize(fen: &str);
    fn get_legal_moves() -> [(char, char); 1];
    fn get_pieces() -> [char; 1];
}

pub struct BitBoard {
    pieces: [u8; 64],
    pieces_bb: [[u64; 6]; 2],

    black_to_move: bool,

    white_o_o: bool,
    white_o_o_o: bool,
    black_o_o: bool,
    black_o_o_o: bool,

    en_passant: u64,

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

// 0-1 - special move flag (1-promotion, 2-en passant, 3-castling)
// 2-3 - promotion piece type (0-Knight, 1-Bishop, 2-Rook, 3-Queen)
// 4-9 - origin
// 10-15 - destination
impl BitBoard {
    // TODO optimize
    pub fn print(&self) {
        let mut board_string  = String::new();

        for file in 0..8 {
            for rank in 0..8 {
                board_string.push_str(piece_to_str(unsafe{transmute(self.pieces[(7 - file) * 8 + rank])}));
                board_string.push_str(" ");
            }
            board_string.push_str("\n");
        }
        println!("{}", board_string);
    }

    pub fn make_move(&mut self, mov: Move) {
        let dest: usize = (mov >> 10) as usize;
        let origin: usize = ((mov & 0b1111110000) >> 4) as usize;
        let promotion_piece_type: u8 = (mov & 0b1100 + 10) as u8;
        let special_move_flag: SpecialMove = unsafe {transmute((mov & 0b000011) as u8) };

        let dest_mask = 1_u64 << dest;
        let dest_anti_mask = !(dest_mask);
        let origin_mask = 1_u64 << origin;
        let origin_anti_mask = !(origin_mask);

        let current = !self.black_to_move as usize;
        let opponent = self.black_to_move as usize;

        let dest_piece = self.pieces[dest] ^ (1 << 7);

        let dest_piece_type = ((dest_piece - self.black_to_move.as_u8()) >> 2) as usize;

        let origin_piece = self.pieces[origin];
        let origin_piece_type = ((origin_piece - self.black_to_move.as_u8()) >> 2) as usize;

        // remove attacked piece
        self.pieces_bb[opponent][dest_piece_type] ^= dest_mask;

        // remove moved piece
        self.pieces_bb[current][origin_piece_type] ^= origin_mask;
        self.pieces[origin] = Piece::Empty as u8;

        // normal move
        self.pieces_bb[current][origin_piece_type] ^= (special_move_flag == SpecialMove::None) as u64 * dest_mask;
        self.pieces[dest] = (special_move_flag == SpecialMove::None) as u8 * origin_piece;

        // promotion
        self.pieces_bb[current][promotion_piece_type as usize] ^= (special_move_flag == SpecialMove::Promotion) as u64 * dest_mask;
        self.pieces[dest] = (special_move_flag == SpecialMove::Promotion) as u8 * ((promotion_piece_type) << 2) + (self.black_to_move as u8);

        // castling
        if special_move_flag == SpecialMove::Castle {
            self.pieces_bb[current][PieceType::King as usize] ^= dest_mask;
            self.pieces[dest] = (special_move_flag == SpecialMove::None) as u8 * origin_piece;

            if origin > dest {
                self.pieces_bb[current][PieceType::Rook as usize] ^= (dest_mask >> 2) + (dest_mask << 1);
                self.pieces[dest - 2] = Piece::Empty as u8;
                self.pieces[dest + 1] = Piece:: WhiteRook as u8 + self.black_to_move as u8;
            }
            else {
                self.pieces_bb[current][PieceType::Rook as usize] ^= (dest_mask << 1) + (dest_mask >> 1);

                self.pieces[dest + 1] = Piece::Empty as u8;
                self.pieces[dest - 1] = Piece:: WhiteRook as u8 + self.black_to_move as u8;
            }
        }

        if special_move_flag == SpecialMove::EnPassant {
            // if white moved
            if origin < dest {
                self.pieces_bb[opponent][PieceType::Pawn as usize] ^= dest_mask >> 8;
                self.pieces[dest - 8] = Piece::Empty as u8;
            } else {
                self.pieces_bb[opponent][PieceType::Pawn as usize] ^= dest_mask << 8;
                self.pieces[dest + 8] = Piece::Empty as u8;
            }
        }

        self.black_to_move = !self.black_to_move;
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves = get_pawn_moves(self);
        moves.append(&mut get_knight_moves(self));
        return moves;
    }
}

pub fn get_ones_indices(bitboard: &u64) -> Vec<u8> {
    let mut num = *bitboard;
    let mut vec: Vec<u8> = Vec::new();
    let mut shift = 0_u8;
    while num != 0 {
        let position = num.trailing_zeros() as u8;
        num = num >> (position + 1);
        shift += position;
        vec.push(shift);
        shift += 1;
        // println!("{}", position);
    }

    return vec;
}
