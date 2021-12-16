use crate::board::Board;
use crate::console_log;
use crate::evaluate::CHECKMATE_VAL;
use crate::pieces::ChessMove;
use crate::rules::possible_moves_from_square;
use crate::search::{Node, alpha_beta_minimax, find_best_move, count_leaves_in_tree};
use crate::utils::{log, coord_to_rank_file};

use crate::Math::random;

/// Generates the best chess move from the current position.
pub fn best_move(board: &Board, depth: usize) -> ChessMove {
    console_log!("engine::best_move: ");
    
    // Create, evaluate and prune the search tree
    let mut root = Node::new_root(&board);
    let alpha = -CHECKMATE_VAL;
    let beta = CHECKMATE_VAL;
    let maximizing_player = board.white_to_move();
    alpha_beta_minimax(&mut root, depth, alpha, beta, maximizing_player);

    // Return the move that gives the best evaluation
    let chess_move = find_best_move(&root);
    
    console_log!("    selected move, src = {:?}, dest = {:?}", chess_move.src, chess_move.dest);
    
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


#[cfg(test)]
mod tests {
    use crate::console_log;
    use crate::board::Board;
    use crate::pieces::ChessMove;
    use crate::engine::{best_move};

    #[test]
    fn hanging_queen() {

        let mut board = Board::new();
        board.set_board_from_fen_string("5rk1/5p1p/6p1/1q6/8/7P/5PP1/1R3RK1");
        board.render();
        let mut depth = 1 as usize;
        let mut selected_move = best_move(&board, depth);
        let known_best_move = ChessMove::new(&board, [1, 2], [5, 2]);
        assert!(selected_move.is_the_same_as(&known_best_move));

        depth = 2 as usize;
        selected_move = best_move(&board, depth);
        assert!(selected_move.is_the_same_as(&known_best_move));

        depth = 3 as usize;
        selected_move = best_move(&board, depth);
        assert!(selected_move.is_the_same_as(&known_best_move));

        depth = 4 as usize;
        selected_move = best_move(&board, depth);
        assert!(selected_move.is_the_same_as(&known_best_move));
    }

    #[test]
    fn promote_to_queen() {
        let mut board = Board::new();
        board.set_board_from_fen_string("7k/2P5/8/8/8/8/8/K7");
        board.render();

        let mut depth = 3 as usize;
        let mut selected_move = best_move(&board, depth);
        let promote_queen = 1;
        let mut known_best_move = ChessMove::new_promotion(&board, [7, 3], [8, 3], promote_queen);
        assert!( selected_move.is_the_same_as(&known_best_move) );

        board.set_board_from_fen_string("3q3k/2P5/8/8/8/8/8/K7");
        board.render();
        selected_move = best_move(&board, depth);
        known_best_move = ChessMove::new_promotion(&board, [7, 3], [8, 4], promote_queen);
        assert!( selected_move.is_the_same_as(&known_best_move) );


    }
}