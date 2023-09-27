#[macro_use]
use lazy_static::lazy_static;
use crate::board::masks::{A_FILE, H_FILE};

const NOT_A_FILE: u64 = !A_FILE;
const NOT_H_FILE: u64 = !H_FILE;

fn get_king_attacks_for_square(square: u64) -> u64 {
    let mut result: u64 = 0;
    result |= (square << 8); // up
    result |= (square << 9) & NOT_A_FILE; // up right
    result |= (square << 1) & NOT_A_FILE; // right
    result |= (square >> 7) & NOT_A_FILE; // down right
    result |= (square >> 8); // down
    result |= (square >> 9) & NOT_H_FILE; // down left
    result |= (square >> 1) & NOT_H_FILE; // left
    result |= (square << 7) & NOT_H_FILE; // up left

    return result;
}

lazy_static! {
    pub static ref KING_ATTACKS: [u64; 64] = {
        let mut king_attacks: [u64; 64] = [0u64; 64];
        for shift in 0..64u64 {
            let square: u64 = 1 << shift;
            king_attacks[shift as usize] = get_king_attacks_for_square(square);
        }
        return king_attacks;
    };
}

