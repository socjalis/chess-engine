pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub enum SpecialMove {
    None,
    Promotion,
    Castle,
    EnPassant,
}

// 0-5 - destination
// 6-11 - origin
// 12-13 - promotion piece type (0-Knight, 1-Bishop, 2-Rook, 3-Queen)
// 14-15 - special move flag (1-promotion, 2-en passant, 3-castling)
pub fn construct_move(dest: u8, origin: u8, promotion_piece: PromotionPiece, special_move_flag: SpecialMove) -> u16 {
    return ((dest as u16) << 10)
        + ((origin as u16) << 4)
        + ((promotion_piece as u16) << 2)
        + special_move_flag as u16;
}