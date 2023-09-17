#[macro_use]
use bitvec::array::BitArray;
use bitvec::order::{Lsb0, Msb0};
use bitvec::view::BitViewSized;
use crate::board::masks::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE, JESUS};
use crate::board::move_generation::attacks::rook::rook_magics::ROOK_MAGICS;
use crate::board::print_bb;

pub mod rook_magics;

#[derive(Clone)]
#[derive(Copy)]
pub struct SquareSlideAttackInfo {
    pub magic: u64,
    pub mask: u64,
    pub shift: u64,
    pub offset: u64,
}

fn get_rank(square: u64) -> u64 {
    return square / 8;
}

fn get_tile(square: u64) -> u64 {
    return square % 8;
}

pub fn get_slide_idx(square: u64, occupancy: u64) -> usize {
    unsafe {
        let info = ROOK_SQUARE_INFO[square as usize];
        return (info.magic.wrapping_mul(occupancy & info.mask) >> info.shift) as usize;
    };
}

pub fn get_rook_attacks(square: u64, occupancy: u64) -> u64 {
    unsafe { return ROOK_ATTACKS[ROOK_SQUARE_INFO[square as usize].offset as usize + get_slide_idx(square, occupancy)]; };
}

pub static mut ROOK_ATTACKS: [u64; 102400] = [0u64; 102400];
pub static mut BISHOP_ATTACKS: [u64; 5248] = [0; 5248];
pub static mut ROOK_SQUARE_INFO: [SquareSlideAttackInfo; 64] = [SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }];
pub static mut BISHOP_SQUARE_INFO: [SquareSlideAttackInfo; 64] = [SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }, SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 }];

pub fn initiate_slider_attacks() {
    let mut deltas = Vec::new();
    deltas.push((1, 0, 0, 0));
    deltas.push((0, 1, 0, 0));
    deltas.push((0, 0, 1, 0));
    deltas.push((0, 0, 0, 1));

    unsafe { initiate_slide_attacks(&deltas, &ROOK_MAGICS, &mut ROOK_ATTACKS, &mut ROOK_SQUARE_INFO); }
}

// let mut slide_attacks_info = [(); 64].map(|_| SquareSlideAttackInfo { shift: 0, magic: 0, mask: 0, offset: 0 });
fn initiate_slide_attacks(
    deltas: &Vec<(u64, u64, u64, u64)>,
    magics: &[u64; 64],
    slide_attacks: &mut [u64],
    slide_attacks_info: &mut [SquareSlideAttackInfo; 64],
) {
    for square in 0u64..64 {
        let edges = if get_rank(square) == 0 { 0 } else { FIRST_RANK } |
            if get_rank(square) == 7 { 0 } else { EIGHTH_RANK } |
            if get_tile(square) == 0 { 0 } else { A_FILE } |
            if get_tile(square) == 7 { 0 } else { H_FILE };

        slide_attacks_info[square as usize].magic = magics[square as usize];
        slide_attacks_info[square as usize].mask = JESUS[square as usize] & !edges & !(1 << square);
        // slide_attacks_info[square as usize].mask = calculate_attacks(&0u64, &square, deltas);

        print_bb(JESUS[square as usize] & !edges & !(1 << square));
        print_bb(calculate_attacks(&0u64, &square, deltas));


        let mask_ones = slide_attacks_info[square as usize].mask.count_ones() as u64;
        slide_attacks_info[square as usize].shift = 64 - mask_ones;

        if square != 63 {
            slide_attacks_info[square as usize + 1].offset = slide_attacks_info[square as usize].offset + 2u64.pow(mask_ones as u32);
        }

        let bitarray: BitArray<u64, Lsb0> = slide_attacks_info[square as usize].mask.into_bitarray();

        let vec: Vec<u64> = bitarray.iter().enumerate().map(|(idx, bit)| {
            return (idx, bit);
        }).filter(|(idx, bit)| {
            return *bit == true;
        }).map(|(idx, bit)| {
            return idx as u64;
        }).collect::<Vec<u64>>();
        let mut occupied = 0u64;
        // loop {
        //     // println!("occupancy bits: {}, vec size: {}, square: {}", slice.len(), vec.len(), square);
        //
        //     let attacks = calculate_attacks(&occupied, &square, deltas);
        //     let hash = get_slide_idx(square, occupied);
        //     let idx = slide_attacks_info[square as usize].offset as usize + hash;
        //
        //     println!("square {} occ number {} existing {} hash {} offset {} ones {}", square, occupied,slide_attacks[idx], hash, slide_attacks_info[square as usize].offset, mask_ones);
        //
        //     slide_attacks[slide_attacks_info[square as usize].offset as usize + hash] = attacks;
        //
        //     occupied = (occupied.wrapping_sub(slide_attacks_info[square as usize].mask)) & slide_attacks_info[square as usize].mask;
        //     if(hash as u64 > 2u64.pow( slide_attacks_info[square as usize].mask.count_ones())){
        //         panic!("chuj");
        //     }
        //     if (occupied == 0) {
        //         break;
        //     }
        // }

        for occupancy_number in 0..(2u64.pow(mask_ones as u32)) {
            let mut occupancy = 0u64;

            let occupancy_bits: BitArray<u64, Lsb0> = occupancy_number.into_bitarray();
            let slice = &occupancy_bits[..vec.len()];

            slice.iter().enumerate().for_each(|(idx, bit)| {
                if *bit {
                    occupancy |= 1 << vec[idx];
                }
            });

            // println!("occupancy bits: {}, vec size: {}, square: {}", slice.len(), vec.len(), square);

            let attacks = calculate_attacks(&occupancy, &square, deltas);
            let hash = get_slide_idx(square, occupancy);

            println!("square {} occ existing {} existing {} hash {}", square, occupancy_number, slide_attacks[(slide_attacks_info[square as usize].offset + hash as u64).clone() as usize], hash);

            slide_attacks[slide_attacks_info[square as usize].offset as usize + hash] = attacks;
        }
    }
}

fn calculate_attacks(occupancy: &u64, square: &u64, moves: &Vec<(u64, u64, u64, u64)>) -> u64 {
    let tile = get_tile(*square);
    let rank = get_rank(*square);

    let mut attacks: u64 = 0;

    moves.iter().for_each(|(up, right, down, left)| {
        for i in 1..8u64 {
            let pushed_tile = tile as i64 + (right * i) as i64 - (left * i) as i64;
            let pushed_rank = rank as i64 + (up * i) as i64 - (down * i) as i64;

            if pushed_rank > 7 || pushed_rank < 0 || pushed_tile > 7 || pushed_tile < 0 {
                break;
            }

            let pushed_square = (1 << pushed_tile) << 8 * pushed_rank;

            attacks |= pushed_square;

            if pushed_square & occupancy > 0 {
                break;
            }
        }
    });

    return attacks;
}