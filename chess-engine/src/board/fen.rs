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

    println!("{:?}", pieces);

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
        // TODO
        en_passant: 0,
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
            // println!("bit idx {}, number {}", (7 - column) * 8 + row, 1_u64 << (7 - column) * 8 + row);
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
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    // println!("ADDING OFFSET {}", ch.to_digit(10).unwrap());
                    offset += ch.to_digit(10).unwrap() as u64;
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