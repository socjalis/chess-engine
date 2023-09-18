use std::f32::consts::E;

pub const FIRST_RANK: u64 = 255;
pub const SECOND_RANK: u64 = 255 << 8;
pub const THIRD_RANK: u64 = 255 << 16;
pub const FOURTH_RANK: u64 = 255 << 24;
pub const FIFTH_RANK: u64 = 255 << 32;
pub const SIXTH_RANK: u64 = 255 << 40;
pub const SEVENTH_RANK: u64 = 255 << 48;
pub const EIGHTH_RANK: u64 = 255 << 56;

pub const A_FILE: u64 = 1_u64 + (1_u64 << 8) | (1_u64 << 16) | (1_u64 << 24) | (1_u64 << 32) | (1_u64 << 40) | (1_u64 << 48) | (1_u64 << 56);
pub const B_FILE: u64 = A_FILE << 1;
pub const C_FILE: u64 = A_FILE << 2;
pub const D_FILE: u64 = A_FILE << 3;
pub const E_FILE: u64 = A_FILE << 4;
pub const F_FILE: u64 = A_FILE << 5;
pub const G_FILE: u64 = A_FILE << 6;
pub const H_FILE: u64 = A_FILE << 7;

pub const JESUS: [u64; 64] = [
    A_FILE | FIRST_RANK, B_FILE | FIRST_RANK, C_FILE | FIRST_RANK, D_FILE | FIRST_RANK, E_FILE | FIRST_RANK, F_FILE | FIRST_RANK, G_FILE | FIRST_RANK, H_FILE | FIRST_RANK,
    A_FILE | SECOND_RANK, B_FILE | SECOND_RANK, C_FILE | SECOND_RANK, D_FILE | SECOND_RANK, E_FILE | SECOND_RANK, F_FILE | SECOND_RANK, G_FILE | SECOND_RANK, H_FILE | SECOND_RANK,
    A_FILE | THIRD_RANK, B_FILE | THIRD_RANK, C_FILE | THIRD_RANK, D_FILE | THIRD_RANK, E_FILE | THIRD_RANK, F_FILE | THIRD_RANK, G_FILE | THIRD_RANK, H_FILE | THIRD_RANK,
    A_FILE | FOURTH_RANK, B_FILE | FOURTH_RANK, C_FILE | FOURTH_RANK, D_FILE | FOURTH_RANK, E_FILE | FOURTH_RANK, F_FILE | FOURTH_RANK, G_FILE | FOURTH_RANK, H_FILE | FOURTH_RANK,
    A_FILE | FIFTH_RANK, B_FILE | FIFTH_RANK, C_FILE | FIFTH_RANK, D_FILE | FIFTH_RANK, E_FILE | FIFTH_RANK, F_FILE | FIFTH_RANK, G_FILE | FIFTH_RANK, H_FILE | FIFTH_RANK,
    A_FILE | SIXTH_RANK, B_FILE | SIXTH_RANK, C_FILE | SIXTH_RANK, D_FILE | SIXTH_RANK, E_FILE | SIXTH_RANK, F_FILE | SIXTH_RANK, G_FILE | SIXTH_RANK, H_FILE | SIXTH_RANK,
    A_FILE | SEVENTH_RANK, B_FILE | SEVENTH_RANK, C_FILE | SEVENTH_RANK, D_FILE | SEVENTH_RANK, E_FILE | SEVENTH_RANK, F_FILE | SEVENTH_RANK, G_FILE | SEVENTH_RANK, H_FILE | SEVENTH_RANK,
    A_FILE | EIGHTH_RANK, B_FILE | EIGHTH_RANK, C_FILE | EIGHTH_RANK, D_FILE | EIGHTH_RANK, E_FILE | EIGHTH_RANK, F_FILE | EIGHTH_RANK, G_FILE | EIGHTH_RANK, H_FILE | EIGHTH_RANK,
];


pub const A1H8: u64 = 1 | 1 << (8 + 1) | 1 << (2 * 8 + 2) | 1 << (3 * 8 + 3) | 1 << (4 * 8 + 4) | 1 << (5 * 8 + 5) | 1 << (6 * 8 + 6) | 1 << (7 * 8 + 7);

pub const H1A8: u64 =
    (1 << 7) |
        (1 << 7) << (8 - 1) |
        (1 << 7) << (2 * 8 - 2) |
        (1 << 7) << (3 * 8 - 3) |
        (1 << 7) << (4 * 8 - 4) |
        (1 << 7) << (5 * 8 - 5) |
        (1 << 7) << (6 * 8 - 6) |
        (1 << 7) << (7 * 8 - 7);

pub const NE: [u64; 15] = [
    (A1H8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE & !E_FILE & !F_FILE & !G_FILE) >> 7,
    (A1H8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE & !E_FILE & !F_FILE) >> 6,
    (A1H8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE & !E_FILE) >> 5,
    (A1H8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE) >> 4,
    (A1H8 & !A_FILE & !B_FILE & !C_FILE) >> 3,
    (A1H8 & !A_FILE & !B_FILE) >> 2,
    A1H8 >> 1,
    A1H8,
    (A1H8 & !H_FILE) << 1,
    (A1H8 & !H_FILE & !G_FILE) << 2,
    (A1H8 & !H_FILE & !G_FILE & !F_FILE) << 3,
    (A1H8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE) << 4,
    (A1H8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE & !D_FILE) << 5,
    (A1H8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE & !D_FILE & !C_FILE) << 6,
    (A1H8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE & !D_FILE & !C_FILE & !B_FILE) << 7,
];

pub const NW: [u64; 15] = [
    (H1A8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE & !E_FILE & !F_FILE & !G_FILE) >> 7,
    (H1A8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE & !E_FILE & !F_FILE) >> 6,
    (H1A8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE & !E_FILE) >> 5,
    (H1A8 & !A_FILE & !B_FILE & !C_FILE & !D_FILE) >> 4,
    (H1A8 & !A_FILE & !B_FILE & !C_FILE) >> 3,
    (H1A8 & !A_FILE & !B_FILE) >> 2,
    H1A8 >> 1,
    H1A8,
    (H1A8 & !H_FILE) << 1,
    (H1A8 & !H_FILE & !G_FILE) << 2,
    (H1A8 & !H_FILE & !G_FILE & !F_FILE) << 3,
    (H1A8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE) << 4,
    (H1A8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE & !D_FILE) << 5,
    (H1A8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE & !D_FILE & !C_FILE) << 6,
    (H1A8 & !H_FILE & !G_FILE & !F_FILE & !E_FILE & !D_FILE & !C_FILE & !B_FILE) << 7,
];

// x
pub const JESUS45: [u64; 64] = [
    NE[7] | NW[0], NE[8] | NW[1], NE[9] | NW[2], NE[10] | NW[3], NE[11] | NW[4], NE[12] | NW[5], NE[13] | NW[6], NE[14] | NW[7],
    NE[6] | NW[1], NE[7] | NW[2], NE[8] | NW[3], NE[9] | NW[4], NE[10] | NW[5], NE[11] | NW[6], NE[12] | NW[7], NE[13] | NW[8],
    NE[5] | NW[2], NE[6] | NW[3], NE[7] | NW[4], NE[8] | NW[5], NE[9] | NW[6], NE[10] | NW[7], NE[11] | NW[8], NE[12] | NW[9],
    NE[4] | NW[3], NE[5] | NW[4], NE[6] | NW[5], NE[7] | NW[6], NE[8] | NW[7], NE[9] | NW[8], NE[10] | NW[9], NE[11] | NW[10],
    NE[3] | NW[4], NE[4] | NW[5], NE[5] | NW[6], NE[6] | NW[7], NE[7] | NW[8], NE[8] | NW[9], NE[9] | NW[10], NE[10] | NW[11],
    NE[2] | NW[5], NE[3] | NW[6], NE[4] | NW[7], NE[5] | NW[8], NE[6] | NW[9], NE[7] | NW[10], NE[8] | NW[11], NE[9] | NW[12],
    NE[1] | NW[6], NE[2] | NW[7], NE[3] | NW[8], NE[4] | NW[9], NE[5] | NW[10], NE[6] | NW[11], NE[7] | NW[12], NE[8] | NW[13],
    NE[0] | NW[7], NE[1] | NW[8], NE[2] | NW[9], NE[3] | NW[10], NE[4] | NW[11], NE[5] | NW[12], NE[6] | NW[13], NE[7] | NW[14],
];


