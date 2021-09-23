use crate::console_log;
use crate::utils::{log, coord_to_rank_file};
use crate::board::Board;
use crate::pieces::ChessMove;
use crate::rules::possible_moves_from_square;

use crate::Math::random;

/// Generates the best chess move from the current position.
pub fn best_move(_board: &Board) -> ChessMove {
    console_log!("engine::best_move: todo!");
    let chess_move = ChessMove::new_empty_move();
    return chess_move;
}

/// Generates a random legal move from the current position
pub fn random_move(board: &Board) -> ChessMove {
    console_log!("engine::random_move: ");

    let mut possible_moves : Vec<ChessMove> = vec![];

    let mut timeout_counter = 0;
    let mut num_possible_moves = 0;
    while num_possible_moves == 0 && timeout_counter < 100 {
        let square_rank_file = get_random_piece_to_move(&board);
        possible_moves = possible_moves_from_square(&board, square_rank_file);
        num_possible_moves = possible_moves.len();
        console_log!("    num possible moves = {}, for piece on square {:?}", 
                     num_possible_moves, square_rank_file);

        timeout_counter = timeout_counter + 1;
    }

    if num_possible_moves == 0 {
        panic!();
    }

    let rand_inx = get_random_usize(num_possible_moves); 
    let chess_move = possible_moves[rand_inx];

    console_log!("    selected move, src = {:?}, dest = {:?}", chess_move.src, chess_move.dest);

    return chess_move;
}

/// For testing a computer that makes random but legal moves
fn get_random_piece_to_move(board: &Board) -> [usize; 2] {
    console_log!("get_random_piece_to_move:");
    // Generate random number in the range [0, 99]
    let mut timeout_counter = 0;
    let mut rand_occupied_rank_file = [0 as usize, 0 as usize];
    while timeout_counter < 1000 {
        let rank = get_random_usize(8) + 1;
        let file = get_random_usize(8) + 1;

        if (board.white_to_move() && board.is_occupied_by_white([rank, file]))|| 
            (!board.white_to_move() && board.is_occupied_by_black([rank, file])){
                rand_occupied_rank_file = [rank, file];
            break;
        }

        timeout_counter = timeout_counter + 1;
    }

    return rand_occupied_rank_file;
}

fn get_random_usize(max: usize) -> usize {
    return ( random() * (max as f64) ) as usize;
}