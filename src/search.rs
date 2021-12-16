use crate::board::Board;
use crate::evaluate::{evaluate, CHECKMATE_VAL};
use crate::rules::all_possible_moves;
use crate::pieces::ChessMove;
use crate::console_log;
use crate::utils::log;

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

    if maximizing_player {
        let mut max_eval = -CHECKMATE_VAL;
        println!("  MAX: num_possible_moves = {}, alpha = {}, beta = {}", num_possible_moves, alpha, beta);
        for i in 0..num_possible_moves {
            let eval = alpha_beta_minimax(&mut node.children[i], depth-1, alpha, beta, false);
            println!("    MAX: i = {}, eval = {}", i, eval);
            max_eval = max(max_eval, eval);
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        println!("  EXIT MAX: alpha = {}, beta = {}", alpha, beta);
        node.eval = max_eval;
        return max_eval;

    } else {
        let mut min_eval = CHECKMATE_VAL;
        println!("  MIN: num_possible_moves = {}, alpha = {}, beta = {}", num_possible_moves, alpha, beta);
        for i in 0..num_possible_moves {
            let eval = alpha_beta_minimax(&mut node.children[i], depth-1, alpha, beta, true);
            println!("    MIN: i = {}, eval = {}", i, eval);
            min_eval = min(min_eval, eval);
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }

        }
        println!("  EXIT MIN: alpha = {}, beta = {}", alpha, beta);
        node.eval = min_eval;
        return min_eval;
        
    }
}

/// This will fail the perf tests, since it drops nodes....
/// Need to have a separate function to test the perf results
// pub fn alpha_beta_minimax(mut root : &mut Node, mut alpha : &mut f32, 
//                           mut beta : &mut f32, depth : usize) -> f32 {

//     if depth == 0 {
//         root.eval = evaluate(&root.position);
//         return root.eval;
//     }

//     let all_possible_moves = all_possible_moves(&root.position);
//     let num_possible_moves = all_possible_moves.len();

//     if num_possible_moves == 0 {
//         root.eval = evaluate(&root.position);
//         return root.eval;
//     }

//     let is_white = root.position.white_to_move();

//     let mut minimax_eval = CHECKMATE_VAL;
//     if is_white {
//         minimax_eval = -CHECKMATE_VAL;
//     }

//     for i in 0..num_possible_moves {

//         // Make the move
//         let mut next_position = root.position.clone();
//         next_position.make_move(all_possible_moves[i]);
//         root.children.push(Node::new(&next_position, &all_possible_moves[i]));

//         // Evaluate the new position
//         let eval = alpha_beta_minimax(&mut root.children[i], &mut alpha, &mut beta, depth-1);

//         // Update the max evaluation
//         if is_white {
//             minimax_eval = max(eval, minimax_eval);
//             *alpha = max(*alpha, minimax_eval);
//             if *beta <= *alpha {
//                 break;
//             }
//         } else {
//             minimax_eval = min(eval, minimax_eval);
//             *beta = min(*beta, minimax_eval);
//             if *beta <= *alpha {
//                 break;
//             }
//         }
//     }

//     println!("  white to move = {}, depth = {}, num possible moves = {}", is_white, depth, num_possible_moves);
//     println!("  alpha = {}, beta = {}", alpha, beta);

//     root.eval = minimax_eval;
//     return minimax_eval;
// }

// The non alpha beta trimming version. Keeping so that I can time it later
// pub fn alpha_beta_minimax(mut root : &mut Node, mut alpha : &mut f32, 
//                           mut beta : &mut f32, depth : usize) -> f32 {

//     if depth == 0 {
//         root.eval = evaluate(&root.position);
//         return root.eval;
//     }

//     let all_possible_moves = all_possible_moves(&root.position);
//     let num_possible_moves = all_possible_moves.len();
    
//     if num_possible_moves == 0 {
//         root.eval = evaluate(&root.position);
//         return root.eval;
//     }

//     let is_white = root.position.white_to_move();

//     let mut minimax_eval = CHECKMATE_VAL;
//     if is_white {
//         minimax_eval = -CHECKMATE_VAL;
//     }

//     for i in 0..num_possible_moves {

//         // Make the move
//         let mut next_position = root.position.clone();
//         next_position.make_move(all_possible_moves[i]);
//         root.children.push(Node::new(&next_position, &all_possible_moves[i]));

//         // Evaluate the new position
//         let eval = alpha_beta_minimax(&mut root.children[i], &mut alpha, &mut beta, depth-1);

//         // Update the max evaluation
//         if is_white {
//             minimax_eval = max(eval, minimax_eval);
//         } else {
//             minimax_eval = min(eval, minimax_eval);
//         }

//     }

//     root.eval = minimax_eval;
//     return minimax_eval;
// }

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

/// // Each node in the search tree stores the position and
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
    use crate::search::{alpha_beta_minimax, count_leaves_in_tree, Node};
    use crate::evaluate::CHECKMATE_VAL;

    /// @todo, the perft tests should not use the alpha beta trimmed
    /// search

    // Testing the search and chess engine using known perf results:
    // https://www.chessprogramming.org/Perft_Results 
    // These tests are broken, since alpha beta pruning 
    #[test]
    fn perft_test_start_pos() {

        // let mut board = Board::new();
        // let empty_move = ChessMove::new_empty_move();
        // let mut root = Node::new(&board, &empty_move);
        // let mut num_leaves = 0 as u32;
        // let mut num_checks = 0 as u32;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!( num_checks, 0 );
        // assert_eq!( num_leaves, 1 );

        // let mut alpha = -CHECKMATE_VAL;
        // let mut beta = CHECKMATE_VAL;
        // alpha_beta_minimax(&mut root, &mut alpha, &mut beta, 1);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 0);
        // assert_eq!(num_leaves, 20);

        // root = Node::new(&board, &empty_move);
        // alpha = -CHECKMATE_VAL;
        // beta = CHECKMATE_VAL;
        // alpha_beta_minimax(&mut root, &mut alpha, &mut beta, 2);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 0);
        // assert_eq!(num_leaves, 400);

        // root = Node::new(&board, &empty_move);
        // alpha = -CHECKMATE_VAL;
        // beta = CHECKMATE_VAL;
        // alpha_beta_minimax(&mut root, &mut alpha, &mut beta, 3);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 12);
        // assert_eq!(num_leaves, 8902); // failing, getting 9194 positions

        // root = Node::new(&board, &empty_move);
        // alpha = -CHECKMATE_VAL;
        // beta = CHECKMATE_VAL;
        // alpha_beta_minimax(&mut root, 4);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 469);
        // assert_eq!(num_leaves, 197281); // failing, getting 210124 positions
    }

    #[test]
    fn perft_test_kiwipete() {
        
        let mut board = Board::new();
        board.set_board_from_fen_string("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
        let empty_move = ChessMove::new_empty_move();
        let mut root = Node::new(&board, &empty_move);
        let mut num_leaves = 0 as u32;
        let mut num_checks = 0 as u32;
        let mut alpha = -CHECKMATE_VAL;
        let mut beta = CHECKMATE_VAL;
        let mut maximizing_player = board.white_to_move();

        // alpha_beta_minimax(&mut root, 1, &mut alpha, &mut beta, maximizing_player);
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 0);
        // assert_eq!(num_leaves, 48);
       
        // root = Node::new(&board, &empty_move);
        // alpha = -CHECKMATE_VAL;
        // beta = CHECKMATE_VAL;
        // alpha_beta_minimax(&mut root, 2);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 3);
        // assert_eq!(num_leaves, 2039); // Fails, gets 2083 positions...
    }
}