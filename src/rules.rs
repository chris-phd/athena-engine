use crate::console_log;
use crate::utils::log;
use crate::pieces::{self, ChessMove};
use crate::board::Board;

/// Checks if the requested move is legal based on the current position
pub fn is_move_legal(board: &Board, chess_move: &ChessMove) -> bool {
    console_log!("rules::is_move_legal: todo!");
    
    let _possible_moves = all_possible_moves(board, chess_move.src());

    // check if chess_move is in the set of all_possible_moves

    return true;
}

/// Checks if the current position is checkmate
pub fn is_checkmate(_board: &Board) -> bool {
    console_log!("rules::is_checkmate: todo!");
    return false;
}

/// all_possible_moves: Given a chess board and a square, 
/// generates all possible chess moves for the piece on that square.
pub fn all_possible_moves(board: &Board, rank_file: [usize; 2]) -> Vec<ChessMove> {
    console_log!("move_gen::all_possible_moves: todo!");

    let piece = board.get_piece_on_square(rank_file);
    let is_white = piece.is_uppercase();
    let piece_type = piece.to_ascii_uppercase();

    let moves: Vec<ChessMove>;
    match piece {
        'N' => moves = pieces::knight_moves(&board, rank_file, is_white),
        'P' => moves = pieces::pawn_moves(&board, rank_file, is_white),
        _ => moves = vec![],
    }

    // return moves;
    return moves;
} 

/// Tests to see that the rules are working
#[cfg(test)]
mod tests {
    use crate::pieces::ChessMove;
    use crate::board::{Board, Position};
    use crate::rules::is_move_legal;


    #[test]
    fn king_legal_capture() {

        let board = Board::new(Position::TestKing);
        board.render();
        let mut requested_move = ChessMove::new_empty_move();

        eprintln!("Test a regular capture");
        requested_move.set_move(&board, [1 as usize, 5 as usize], [2 as usize, 4 as usize]);
        assert!( is_move_legal(&board, &requested_move) );

    }
}