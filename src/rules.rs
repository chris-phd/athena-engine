use crate::console_log;
use crate::utils::log;
use crate::pieces::ChessMove;
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

fn pawn_moves() -> Vec<ChessMove> {
    console_log!("rules::pawn_moves: todo!");
    let moves = vec![ChessMove::new()];
    return moves;
}

/// all_possible_moves: Given a chess board and a square, 
/// generates all possible chess moves for the piece on that square.
pub fn all_possible_moves(board: &Board, rank_file: [usize; 2]) -> Vec<ChessMove> {
    console_log!("move_gen::all_possible_moves: todo!");

    // let piece = board.get_piece_on_square(rank_file);

    // let moves: Vec<ChessMove>;
    // match piece {
    //     'p' | 'P' => moves = pawn_moves(),
    //     _ => moves = panic!(),
    // }

    // return moves;
    return vec![ChessMove::new()];
} 