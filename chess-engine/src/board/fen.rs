use crate::board::{BitBoard, get_ones_indices};
use crate::board::pieces::Piece;

pub fn create_bitboard_from_fen(fen: &str) -> BitBoard {
    let [fen_pieces, fen_color, fen_castles, fen_en_passant, fen_half_moves, fen_full_moves]: [&str; 6] =
        fen.split(" ").collect::<Vec<&str>>().try_into().unwrap();

    let (
        white_pawns,
        white_knights,
        white_bishops,
        white_rooks,
        white_queens,
        white_king,
        black_pawns,
        black_knights,
        black_bishops,
        black_rooks,
        black_queens,
        black_king
    ) = get_pieces_from_fen(fen_pieces);

    println!("xdd {:b}", white_pawns);

    let (
        white_o_o,
        white_o_o_o,
        black_o_o,
        black_o_o_o
    ) = get_castles_from_fen(fen_castles);

    let mut pieces: [u8; 64] = [Piece::Empty as u8; 64];

    get_ones_indices(&white_pawns).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::WhitePawn as u8;
    });

    get_ones_indices(&white_knights).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::WhiteKnight as u8;
    });

    get_ones_indices(&white_bishops).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::WhiteBishop as u8;
    });

    get_ones_indices(&white_rooks).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::WhiteRook as u8;
    });

    get_ones_indices(&white_queens).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::WhiteQueen as u8;
    });

    get_ones_indices(&white_king).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::WhiteKing as u8;
    });

    get_ones_indices(&black_pawns).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::BlackPawn as u8;
    });

    get_ones_indices(&black_knights).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::BlackKnight as u8;
    });

    get_ones_indices(&black_bishops).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::BlackBishop as u8;
    });

    get_ones_indices(&black_rooks).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::BlackRook as u8;
    });

    get_ones_indices(&black_queens).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::BlackQueen as u8;
    });

    get_ones_indices(&black_king).iter().for_each(|idx| {
        pieces[*idx as usize] = Piece::BlackKing as u8;
    });


    println!("dupa {:?}", pieces[16]);

    return BitBoard {
        pieces_bb: [[
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_king,
        ],
        [
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_king,
        ]],
        pieces,
        black_to_move: fen_color == "b",
        white_o_o,
        white_o_o_o,
        black_o_o,
        black_o_o_o,
        en_passant: get_en_passant_from_fen(fen_en_passant),
        half_moves: fen_half_moves.parse().unwrap(),
        full_moves: fen_full_moves.parse().unwrap(),
    };
}

fn get_pieces_from_fen(fen_pieces: &str) -> (u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut white_pawns: u64 = 0;
    let mut white_knights: u64 = 0;
    let mut white_bishops: u64 = 0;
    let mut white_rooks: u64 = 0;
    let mut white_queens: u64 = 0;
    let mut white_king: u64 = 0;
    let mut black_pawns: u64 = 0;
    let mut black_knights: u64 = 0;
    let mut black_bishops: u64 = 0;
    let mut black_rooks: u64 = 0;
    let mut black_queens: u64 = 0;
    let mut black_king: u64 = 0;

    for (line, part) in fen_pieces.split("/").enumerate() {
        let mut offset: u64 = 0;
        fn get_position(column: u64, row: u64) -> u64 {
            return 1_u64 << (7 - column) * 8 + row;
        }

        for (idx, ch) in part.chars().enumerate() {
            let number = get_position(line as u64, offset + idx as u64);

            match ch {
                'P' => {
                    white_pawns |= number;
                }
                'N' => {
                    white_knights |= number;
                }
                'B' => {
                    white_bishops |= number;
                }
                'R' => {
                    white_rooks |= number;
                }
                'Q' => {
                    white_queens |= number;
                }
                'K' => {
                    white_king |= number;
                }
                'p' => {
                    black_pawns |= number;
                }
                'n' => {
                    black_knights |= number;
                }
                'b' => {
                    black_bishops |= number;
                }
                'r' => {
                    black_rooks |= number;
                }
                'q' => {
                    black_queens |= number;
                }
                'k' => {
                    black_king |= number;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    offset += ch.to_digit(10).unwrap() as u64 - 1;
                }
                _ => {
                    panic!("unexpected character {}", ch);
                }
            }
        }
    }

    return (
        white_pawns,
        white_knights,
        white_bishops,
        white_rooks,
        white_queens,
        white_king,
        black_pawns,
        black_knights,
        black_bishops,
        black_rooks,
        black_queens,
        black_king
    );
}

fn get_castles_from_fen(castles_fen: &str) -> (bool, bool, bool, bool) {
    let white_o_o: bool = castles_fen.contains("K");
    let white_o_o_o: bool = castles_fen.contains("Q");
    let black_o_o: bool = castles_fen.contains("k");
    let black_o_o_o: bool = castles_fen.contains("q");

    return (
        white_o_o,
        white_o_o_o,
        black_o_o,
        black_o_o_o
    );
}

fn get_en_passant_from_fen(en_passant_fen: &str) -> u64 {
    if en_passant_fen == "-" {
        return 0;
    }
    assert!(en_passant_fen.len() == 2, "Incorrect en passant length {}", en_passant_fen);

    let first_char = en_passant_fen.chars().nth(0).unwrap();
    let second_char = en_passant_fen.chars().nth(1).unwrap();
    let is_proper_letter = |c: char| -> bool { c.is_ascii() && 'a' <= c && 'h' >= c };
    let is_proper_number = |c: char| -> bool { c.is_ascii_digit() && c != '0' && c != '9'};
    assert!(is_proper_letter(first_char), "Incorrect first character of en passant {}", en_passant_fen);
    assert!(is_proper_number(second_char), "Incorrect number of en passant {}", en_passant_fen);

    let column_shift: u64 = first_char as u64 - 'a' as u64;
    let rank_shift: u64 = (second_char.to_digit(10).unwrap() - 1) as u64 * 8_u64;
    let en_passant_square: u64 = 1 << (rank_shift + column_shift);

    return en_passant_square;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_en_passant_from_fen_correct_input() {
        assert_eq!(get_en_passant_from_fen("-"), 0);
        assert_eq!(get_en_passant_from_fen("a1"), 1);
        assert_eq!(get_en_passant_from_fen("b1"), 2);
        assert_eq!(get_en_passant_from_fen("a2"), 2_u64.pow(8));
        assert_eq!(get_en_passant_from_fen("h8"), 2_u64.pow(63));
    }

    #[test]
    #[should_panic(expected = "Incorrect en passant length xd2137")]
    fn test_get_en_passant_from_incorrect_fen_too_long() {
        get_en_passant_from_fen("xd2137");
    }

    #[test]
    #[should_panic(expected = "Incorrect first character of en passant i4")]
    fn test_get_en_passant_from_incorrect_fen_first_character() {
        get_en_passant_from_fen("i4");
    }

    #[test]
    #[should_panic(expected = "Incorrect number of en passant a9")]
    fn test_get_en_passant_from_fen_incorrect_number() {
        get_en_passant_from_fen("a9");
    }
}
