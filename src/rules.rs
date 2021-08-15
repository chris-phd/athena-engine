use crate::console_log;
use crate::utils::log;
use crate::pieces::ChessMove;
use crate::board::Board;

/// Checks if the requested move is legal based on the current position
pub fn is_move_legal(board: &Board, _chess_move: &ChessMove) -> bool {
    console_log!("rules::is_move_legal: todo!");
    
    let _possible_moves = all_possible_moves(board);

    // check if chess_move is in the set of all_possible_moves

    return true;
}

/// all_possible_moves: Given a chess board and a square, 
/// generates all possible chess moves for the piece on that square.
pub fn all_possible_moves(_board: &Board) -> Vec<ChessMove> {
    console_log!("move_gen::all_possible_moves: todo!");

    let moves = vec![ChessMove::new()];

    return moves;
} 