#[macro_use]
use lazy_static::lazy_static;
use crate::board::masks::{A_FILE, B_FILE, G_FILE, H_FILE};

const NOT_A_FILE: u64 = !A_FILE;
const NOT_AB_FILE: u64 = !A_FILE & !B_FILE;
const NOT_H_FILE: u64 = !H_FILE;
const NOT_GH_FILE: u64 = !G_FILE & !H_FILE;

fn get_knight_attacks_for_square(s: u64) -> u64 {
    let mut result: u64 = 0;
    result |= (s << 17) & NOT_A_FILE; // up up right
    result |= (s << 10) & NOT_AB_FILE; // up right right
    result |= (s >> 6) & NOT_AB_FILE; // down right right
    result |= (s >> 15) & NOT_A_FILE; // down down right
    result |= (s << 15) & NOT_H_FILE; // up up left
    result |= (s << 6) & NOT_GH_FILE; // up left left
    result |= (s >> 10) & NOT_GH_FILE; // down left left
    result |= (s >> 17) & NOT_H_FILE; // down down left

    return result;
}

lazy_static! {
    pub static ref KNIGHT_ATTACKS: [u64; 64] = {
        let mut knight_attacks: [u64; 64] = [0u64; 64];
        for shift in 0..64u64 {
            let square: u64 = 1 << shift;
            knight_attacks[shift as usize] = get_knight_attacks_for_square(square);
        }
        return knight_attacks;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_knight_attacks_for_square() {
        let a1: u64 = 1;
        let e4: u64 = 1 << 28;
        let h8: u64 = 1 << 63;
        assert_eq!(get_knight_attacks_for_square(0u64), 0u64);
        assert_eq!(get_knight_attacks_for_square(a1), a1 << 17 | a1 << 10);
        assert_eq!(get_knight_attacks_for_square(e4), e4 << 17 | e4 << 10 | e4 >> 6 | e4 >> 15 | e4 << 15 | e4 << 6 |e4 >> 10 | e4 >> 17);
        assert_eq!(get_knight_attacks_for_square(h8), h8 >> 10 | h8 >> 17);
    }
}
