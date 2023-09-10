use std::collections::HashMap;

pub fn piece_to_str(piece: Piece) -> &'static str {
    return match piece {
        Piece::WhitePawn => { "P" }
        Piece::BlackPawn => { "p" }
        Piece::WhiteKnight => { "N" }
        Piece::BlackKnight => { "n" }
        Piece::WhiteBishop => { "B" }
        Piece::BlackBishop => { "b" }
        Piece::WhiteRook => { "R" }
        Piece::BlackRook => { "r" }
        Piece::WhiteQueen => { "Q" }
        Piece::BlackQueen => { "q" }
        Piece::WhiteKing => { "K" }
        Piece::BlackKing => { "k" }
        Piece::Empty => { "-" }
    }
}

#[derive(PartialEq)]
pub enum Piece {
    WhitePawn = 0,
    BlackPawn = 1,
    WhiteKnight = 4,
    BlackKnight = 5,
    WhiteBishop = 8,
    BlackBishop = 9,
    WhiteRook = 12,
    BlackRook = 13,
    WhiteQueen = 16,
    BlackQueen = 17,
    WhiteKing = 20,
    BlackKing = 21,
    Empty = 128,
}

#[derive(PartialEq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

