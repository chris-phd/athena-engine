use crate::console_log;
use crate::utils::log;
use crate::board::Board;
use crate::pieces::ChessMove;
use crate::rules::all_possible_moves;

/// Evalutates the current position.
pub fn _evaluate(_board: &Board) -> i32 {
    console_log!("engine::evalutate: todo!");
    let eval_score = 0;
    return eval_score;
}

/// Generates the best chess move from the current position.
pub fn best_move(board: &Board) -> ChessMove {
    console_log!("engine::best_move: todo!");

    let _possible_moves = all_possible_moves(&board);

    return ChessMove::new();
}