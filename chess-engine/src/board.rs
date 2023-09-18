use std::mem::transmute;
use std::ops::BitXor;
use bitvec::array::BitArray;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::order::{Lsb0, Msb0};
use bitvec::view::BitViewSized;
use crate::board::masks::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE, JESUS, SECOND_RANK};
use crate::board::move_generation::attacks::knight::KNIGHT_ATTACKS;
use crate::board::move_generation::attacks::rook::{get_bishop_attacks, get_rook_attacks};
use crate::board::move_generation::knights::get_knight_moves;
use crate::board::move_generation::pawns::get_pawn_moves;
use crate::board::moves::{construct_move, get_moves_for_piece_type, get_slider_moves, PromotionPiece, SpecialMove};
use crate::board::pieces::{Piece, piece_to_str, PieceType};
use crate::board::pieces::PieceType::{Bishop, Knight, Queen, Rook};

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
    fn get_legal_moves() -> Move;
    fn get_pieces() -> [char; 1];
}

pub struct BitBoard {
    pieces: [u8; 64],
    pieces_bb: [[u64; 6]; 2],
    pieces_color: [u64; 2],

    black_to_move: bool,

    white_o_o: bool,
    white_o_o_o: bool,
    black_o_o: bool,
    black_o_o_o: bool,

    en_passant: u64,

    half_moves: u16,
    full_moves: u16,
}

impl BitBoard {
    fn initialize(fen: &str) {
        let splitted = fen.split("/");
        for part in splitted {
            println!("{}", part);
        }
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
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let current_pieces = self.pieces_color[self.black_to_move as usize];
        let occupancy = self.pieces_color[WHITE] | self.pieces_color[BLACK];

        let mut pawn_moves = get_pawn_moves(self);
        let mut knight_moves = get_moves_for_piece_type(self.pieces_bb[self.black_to_move as usize][Knight as usize], *KNIGHT_ATTACKS, current_pieces);
        let mut rook_moves = get_slider_moves(self.pieces_bb[self.black_to_move as usize][Rook as usize], get_rook_attacks, occupancy, current_pieces);
        let mut bishop_moves = get_slider_moves(self.pieces_bb[self.black_to_move as usize][Bishop as usize], get_bishop_attacks, occupancy, current_pieces);
        let mut queen_moves: Vec<Move> = Vec::new();
        queen_moves.append(&mut get_slider_moves(self.pieces_bb[self.black_to_move as usize][Queen as usize], get_rook_attacks, occupancy, current_pieces));
        queen_moves.append(&mut get_slider_moves(self.pieces_bb[self.black_to_move as usize][Queen as usize], get_bishop_attacks, occupancy, current_pieces));

        let mut moves :Vec<Move> = Vec::new();

        moves.append(&mut pawn_moves);
        moves.append(&mut knight_moves);
        moves.append(&mut rook_moves);
        moves.append(&mut bishop_moves);
        moves.append(&mut queen_moves);

        return moves;
    }
    pub fn occupancy(&self) -> u64 {
        return self.pieces_bb[WHITE][PieceType::Pawn as usize] |
            self.pieces_bb[WHITE][PieceType::Knight as usize] |
            self.pieces_bb[WHITE][PieceType::Bishop as usize] |
            self.pieces_bb[WHITE][PieceType::Rook as usize] |
            self.pieces_bb[WHITE][PieceType::Queen as usize] |
            self.pieces_bb[WHITE][PieceType::King as usize] |
            self.pieces_bb[BLACK][PieceType::Pawn as usize] |
            self.pieces_bb[BLACK][PieceType::Knight as usize] |
            self.pieces_bb[BLACK][PieceType::Bishop as usize] |
            self.pieces_bb[BLACK][PieceType::Rook as usize] |
            self.pieces_bb[BLACK][PieceType::Queen as usize] |
            self.pieces_bb[BLACK][PieceType::King as usize];
    }
    pub fn print(&self) {
        let mut board_string = String::new();

        for file in 0..8 {
            for rank in 0..8 {
                board_string.push_str(piece_to_str((unsafe { transmute(self.pieces[(7 - file) * 8 + rank]) })));
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
        let special_move_flag: SpecialMove = unsafe { transmute((mov & 0b000011) as u8) };

        let dest_mask = 1_u64 << dest;
        let origin_mask = 1_u64 << origin;

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
                self.pieces[dest + 1] = Piece::WhiteRook as u8 + self.black_to_move as u8;
            } else {
                self.pieces_bb[current][PieceType::Rook as usize] ^= (dest_mask << 1) + (dest_mask >> 1);

                self.pieces[dest + 1] = Piece::Empty as u8;
                self.pieces[dest - 1] = Piece::WhiteRook as u8 + self.black_to_move as u8;
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

        self.pieces_color[WHITE] = self.pieces_bb[WHITE][PieceType::Pawn as usize] |
            self.pieces_bb[WHITE][PieceType::Knight as usize] |
            self.pieces_bb[WHITE][PieceType::Bishop as usize] |
            self.pieces_bb[WHITE][PieceType::Rook as usize] |
            self.pieces_bb[WHITE][PieceType::Queen as usize] |
            self.pieces_bb[WHITE][PieceType::King as usize];

        self.pieces_color[BLACK] = self.pieces_bb[BLACK][PieceType::Pawn as usize] |
            self.pieces_bb[BLACK][PieceType::Knight as usize] |
            self.pieces_bb[BLACK][PieceType::Bishop as usize] |
            self.pieces_bb[BLACK][PieceType::Rook as usize] |
            self.pieces_bb[BLACK][PieceType::Queen as usize] |
            self.pieces_bb[BLACK][PieceType::King as usize];

        self.black_to_move = !self.black_to_move;
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

pub fn print_bb(bitboard: u64) {
    let bits: BitArray<u64, Lsb0> = bitboard.into_bitarray();
    let mut count = 0;
    let mut strings: [String; 8] = [(); 8].map(|_| String::new());

    bits.iter().enumerate().for_each(|(idx, bit)| {
        count += 1;
        strings[idx / 8].push_str(if bit == true { "1" } else { "0" });
    });

    strings.reverse();

    strings.iter().enumerate().for_each(|(idx, string)| {
        println!("{}", string);
    });

    println!();
}