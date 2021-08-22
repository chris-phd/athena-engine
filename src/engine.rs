use crate::console_log;
use crate::utils::{log, coord_to_rank_file};
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

    let square_rank_file = [0 as usize; 2];
    let _possible_moves = all_possible_moves(&board, square_rank_file);

    let src_rank_file = coord_to_rank_file("e7");
    let dest_rank_file = coord_to_rank_file("e5");
    let chess_move = ChessMove::new(&board, src_rank_file, dest_rank_file);
    return chess_move;
}