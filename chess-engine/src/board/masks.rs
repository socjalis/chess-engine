pub const FIRST_RANK: u64 = 255;
pub const SECOND_RANK: u64 = 255 << 8;
pub const THIRD_RANK: u64 = 255 << 16;
pub const FOURTH_RANK: u64 = 255 << 24;
pub const FIFTH_RANK: u64 = 255 << 32;
pub const SIXTH_RANK: u64 = 255 << 40;
pub const SEVENTH_RANK: u64 = 255 << 48;
pub const EIGHTH_RANK: u64 = 255 << 56;

pub const A_FILE: u64 = 1_u64 + (1_u64 << 8) | (1_u64 << 16) | (1_u64 << 24) | (1_u64 << 32) | (1_u64 << 40) | (1_u64 << 48) | (1_u64 << 56);
pub const B_FILE: u64 = A_FILE >> 1;
pub const C_FILE: u64 = A_FILE >> 2;
pub const D_FILE: u64 = A_FILE >> 3;
pub const E_FILE: u64 = A_FILE >> 4;
pub const F_FILE: u64 = A_FILE >> 5;
pub const G_FILE: u64 = A_FILE >> 6;
pub const H_FILE: u64 = A_FILE >> 7;