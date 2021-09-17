use crate::console_log;
use crate::utils::log;
use crate::pieces::{self, ChessMove};
use crate::board::Board;

/// Checks if the requested move is legal based on the current position
pub fn is_move_legal(board: &Board, requested_move: &ChessMove) -> bool {
    console_log!("rules::is_move_legal: ");
    
    let possible_moves = all_possible_moves(board, requested_move.src);

    let mut is_legal = false;
    for possible_move in possible_moves {
        if requested_move.is_the_same_as(&possible_move) {
            is_legal = true;
            break;
        }
    }

    if is_legal {
        console_log!("    move IS legal");
    } else {
        console_log!("    move is NOT legal");
    }

    return is_legal;
}

/// all_possible_moves: Given a chess board and a square, 
/// generates all possible chess moves for the piece on that square.
pub fn all_possible_moves(board: &Board, rank_file: [usize; 2]) -> Vec<ChessMove> {

    let piece = board.get_piece_on_square(rank_file);
    let is_white = piece.is_uppercase();
    let piece_type = piece.to_ascii_uppercase();

    let moves: Vec<ChessMove>;
    match piece_type {
        'K' => moves = pieces::king_moves(&board, rank_file, is_white),
        'Q' => moves = pieces::queen_moves(&board, rank_file, is_white),
        'R' => moves = pieces::rook_moves(&board, rank_file, is_white),
        'B' => moves = pieces::bishop_moves(&board, rank_file, is_white),
        'N' => moves = pieces::knight_moves(&board, rank_file, is_white),
        'P' => moves = pieces::pawn_moves(&board, rank_file, is_white),
        _ => panic!(),
    }

    // return moves;
    return moves;
} 

/// Tests to see that the rules are working
#[cfg(test)]
mod tests {
    use crate::pieces::ChessMove;
    use crate::board::Board;
    use crate::rules::{is_move_legal};


    #[test]
    fn king_legal_capture() {

        let mut board = Board::new();
        board.set_board_from_fen_string("8/8/8/8/8/8/3r1PPP/R3K2R");
        board.render();
        let mut requested_move = ChessMove::new_empty_move();

        eprintln!("Test a regular capture");
        requested_move.set_move(&board, [1 as usize, 5 as usize], [2 as usize, 4 as usize]);
        assert!( is_move_legal(&board, &requested_move) );

    }
}