#[macro_use]
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROOK_ATTACKS: [[[(u8, u8, u8, u8); 255]; 2]; 64] = {
        let rook_attacks: [[[(u8, u8, u8, u8); 255]; 2]; 64] = [[[(0u8, 0u8, 0u8, 0u8); 255]; 2]; 64];

        for square in 0u8..63 {
            let rank = square / 8;
            for vertical in 0u8..6{
                for horizontal in 0u8..6 {

                }
            }
        }

        return rook_attacks;
    };
}
