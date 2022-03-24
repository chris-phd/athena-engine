use crate::board::Board;
use crate::evaluate::{evaluate, CHECKMATE_VAL};
use crate::rules::all_possible_moves;
use crate::pieces::ChessMove;
use crate::console_log;
use crate::utils::log;

/// Returns the move that leads toward the position with the best evaluation
/// for the selected colour. 
pub fn find_best_move(root : &Node) -> ChessMove {

    assert!(root.children.len() > 0);

    let is_white = root.position.white_to_move();

    let mut best_eval_inx : usize = 0;
    let mut best_minimax_eval : f32;
    if is_white {
        best_minimax_eval = -CHECKMATE_VAL;
    } else {
        best_minimax_eval = CHECKMATE_VAL;
    }

    let num_possible_moves = root.children.len();
    println!("find_best_move:");
    println!("    num possible moves = {}", num_possible_moves);
    for i in 0..num_possible_moves {

        let eval = root.children[i].eval;
        console_log!("  i = {}, eval = {}", i, eval);
        if ( is_white && eval > best_minimax_eval ) || 
           ( !is_white && eval < best_minimax_eval ){
            best_minimax_eval = eval;
            best_eval_inx = i;
        }
    }

    console_log!("  best_eval_inx = {}", best_eval_inx);
    return root.children[best_eval_inx].chess_move_from_parent.clone();
}

/// Create search tree of possible positions to a given depth.
/// Uses a minimax algorithm with alpha-beta trimming for improved efficiency.
/// https://en.wikipedia.org/wiki/Minimax
pub fn alpha_beta_minimax(mut node: &mut Node, depth: usize, 
                          initial_alpha: f32, initial_beta: f32, maximizing_player: bool) -> f32 {

    if depth == 0 {
        node.eval = evaluate(&node.position);
        return node.eval;
    }

    let all_possible_moves = all_possible_moves(&node.position);
    let num_possible_moves = all_possible_moves.len();
    if num_possible_moves == 0 {
        node.eval = evaluate(&node.position);
        return node.eval;
    }

    for i in 0..num_possible_moves {
        let mut child = node.position.clone();
        child.make_move(all_possible_moves[i]);
        node.children.push(Node::new(&child, &all_possible_moves[i]));
    }

    let mut alpha = initial_alpha;
    let mut beta = initial_beta;

    let mut minimax_eval = if maximizing_player { -CHECKMATE_VAL } else { CHECKMATE_VAL };

    for i in 0..num_possible_moves {

        // Make the move
        let mut next_position = node.position.clone();
        next_position.make_move(all_possible_moves[i]);
        node.children.push(Node::new(&next_position, &all_possible_moves[i]));

        // Evaluate the position
        let eval = alpha_beta_minimax(&mut node.children[i], depth-1, alpha, beta, !maximizing_player);

        if maximizing_player {
            minimax_eval = max(minimax_eval, eval);
            alpha = max(alpha, eval);
        } else {
            minimax_eval = min(minimax_eval, eval);
            beta = min(beta, eval);
        }

        if beta <= alpha {
            break;
        }
    }
    node.eval = minimax_eval;
    return minimax_eval;
}

/// Performance test move path optimization 
/// https://www.chessprogramming.org/Perft
pub fn perft(board: &Board, depth: u32) -> u32 {

    if depth == 0 {
        return 1;
    }

    let mut total_leaves = 0;
    let all_possible_moves = all_possible_moves(&board);
    let num_possible_moves = all_possible_moves.len();

    for i in 0..num_possible_moves {
        let mut next_position = board.clone();
        next_position.make_move(all_possible_moves[i]);
        total_leaves += perft(&next_position, depth-1);
    }

    return total_leaves;
}

pub fn count_leaves_in_tree(root : &Node, num_leaves : &mut u32, num_checks : &mut u32) {

    let num_child_nodes = root.children.len();

    if root.position.is_check() {
        *num_checks = *num_checks + 1;
    }

    // Break condition, when there are no child nodes we are
    // at a leaf.
    if num_child_nodes == 0 {
        *num_leaves = *num_leaves + 1 as u32;
        return;
    }

    for i in 0..num_child_nodes {
        count_leaves_in_tree(&root.children[i], num_leaves, num_checks);
    }
}

fn max(a : f32, b : f32) -> f32 {
    if b > a {
        return b;
    }
    return a;
}

fn min(a : f32, b : f32) -> f32 {
    if b < a {
        return b;
    }
    return a;
}

/// Each node in the search tree stores the position and
/// the leaves of the tree will have an evaluation.
pub struct Node {
    pub children: Vec<Node>,
    pub chess_move_from_parent: ChessMove,
    pub position: Board,
    pub is_evaluated: bool, 
    pub eval: f32,
}

impl Node {
    pub fn new(board : &Board, chess_move : &ChessMove) -> Node {
        return Node {
            children : vec![],
            chess_move_from_parent : chess_move.clone(),
            position : board.clone(),
            is_evaluated : false,
            eval : 0.0,
        }
    }

    pub fn new_root(board : &Board) -> Node {
        let empty_move = ChessMove::new_empty_move();
        return Node::new(&board, &empty_move);
    }
}

#[cfg(test)]
mod tests {
    use crate::console_log;
    use crate::board::Board;
    use crate::pieces::ChessMove;
    use crate::search::{alpha_beta_minimax, perft, Node};
    use crate::evaluate::CHECKMATE_VAL;

    // Testing the search and chess engine using known perf results:
    // https://www.chessprogramming.org/Perft_Results 
    // These tests are broken, since alpha beta pruning 
    #[test]
    fn perft_test_start_pos() {

        let mut board = Board::new();
        let mut depth = 0;
        let mut num_leaves = perft(&board, depth);
        assert_eq!( num_leaves, 1 );

        depth = 1;
        num_leaves = perft(&board, depth);
        assert_eq!(num_leaves, 20);

        depth = 2;
        num_leaves = perft(&board, depth);
        assert_eq!(num_leaves, 400);

        depth = 3;
        num_leaves = perft(&board, depth);
        assert_eq!(num_leaves, 8902); // failing, getting 9194 positions

        depth = 4;
        num_leaves = perft(&board, depth);
        assert_eq!(num_leaves, 197281); // failing, getting 209691 positions
    }

    #[test]
    fn perft_test_kiwipete() {
        
        let mut board = Board::new();
        board.set_board_from_fen_string("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
        let mut depth = 1;
        let mut num_leaves = perft(&board, depth);
        assert_eq!( num_leaves, 48);
       
        depth = 2;
        num_leaves = perft(&board, depth);
        assert_eq!(num_leaves, 2039); // Fails, gets 2082 positions...
    }
}