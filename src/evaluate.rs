use crate::board::Board;
use crate::pieces;

const KING_VAL : f32 = 200.0;
const QUEEN_VAL : f32 = 9.0;
const ROOK_VAL : f32 = 5.0;
const BISHOP_VAL : f32 = 3.0;
const KNIGHT_VAL : f32 = 3.0;
const PAWN_VAL : f32 = 1.0;

pub fn evaluate_material(board : &Board) -> f32 {
    let pieces = count_pieces(&board);
 
    let material_score: f32 = KING_VAL * (pieces.white_kings - pieces.black_kings) +
                              QUEEN_VAL * (pieces.white_queens - pieces.black_queens) +
                              ROOK_VAL * (pieces.white_rooks - pieces.black_rooks) + 
                              BISHOP_VAL * (pieces.white_bishops - pieces.black_bishops) +
                              KNIGHT_VAL * (pieces.white_knights - pieces.black_knights) +
                              PAWN_VAL * (pieces.white_pawns - pieces.black_pawns);

    return material_score;
}

fn count_pieces(board : &Board) -> NumPiecesOnBoard {

    let mut pieces = NumPiecesOnBoard::new();

    for i in 0..64 {
        match board.get_piece_by_square_index(i) {
                'p' => pieces.black_pawns += 1.0,
                'P' => pieces.white_pawns += 1.0,
                'n' => pieces.black_knights += 1.0,
                'N' => pieces.white_knights += 1.0,
                'b' => pieces.black_bishops += 1.0,
                'B' => pieces.white_bishops += 1.0,
                'r' => pieces.black_rooks += 1.0,
                'R' => pieces.white_rooks += 1.0,
                'q' => pieces.black_queens += 1.0,
                'Q' => pieces.white_queens += 1.0,
                'k' => pieces.black_kings += 1.0,
                'K' => pieces.white_kings += 1.0,
                _   => {  },
        }
    }

    return pieces;
}

/// The number of pieces of each type in a given position
struct NumPiecesOnBoard {
    pub white_kings: f32,
    pub black_kings: f32,
    pub white_queens: f32,
    pub black_queens: f32,
    pub white_rooks: f32,
    pub black_rooks: f32,
    pub white_bishops: f32,
    pub black_bishops: f32,
    pub white_knights: f32,
    pub black_knights: f32,
    pub white_pawns: f32,
    pub black_pawns: f32,
}

impl NumPiecesOnBoard {
    pub fn new() -> NumPiecesOnBoard {
        return NumPiecesOnBoard {
            white_kings: 0.0,
            black_kings: 0.0,
            white_queens: 0.0,
            black_queens: 0.0,
            white_rooks: 0.0,
            black_rooks: 0.0,
            white_bishops: 0.0,
            black_bishops: 0.0,
            white_knights: 0.0,
            black_knights: 0.0,
            white_pawns: 0.0,
            black_pawns: 0.0,
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::evaluate::evaluate_material;

    #[test]
    fn evaluate_material_1() {
        let mut board1 = Board::new();
        board1.set_board_from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        let mut board2 = Board::new();
        board2.set_board_from_fen_string("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        assert_eq!(evaluate_material(&board1), 0.0);
        assert!(evaluate_material(&board2) > evaluate_material(&board1));
    }
}