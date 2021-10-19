use crate::board::Board;
use crate::evaluate::evaluate;
use crate::rules::all_possible_moves;
use crate::pieces::ChessMove;

/// Creates the search tree from the current position. The depth is the number 
/// of half moves to search.
/// Todo! Could improve effiency by avoiding recusion and using a binary tree
/// the search tree is a binary tree.
pub fn create_search_tree(mut root : &mut Node, depth : usize) {

    // Recursive function break condition. 
    // The search depth has been reached.
    if depth == 0 {
        root.evaluation = evaluate(&root.position);
        root.is_evaluated = true;
        return;
    }

    let all_possible_moves = all_possible_moves(&root.position);
    let num_possible_moves = all_possible_moves.len();
    let new_depth = depth - 1;

    for i in 0..num_possible_moves {

        let mut next_position = root.position.clone();
        next_position.make_move(all_possible_moves[i]);

        root.children.push(Node::new(&next_position, &all_possible_moves[i]));

        create_search_tree( root.children.last_mut().unwrap(), new_depth );
    }

    // There are no possible moves from the current position. 
    root.evaluation = evaluate(&root.position);
    root.is_evaluated = true;
}

/// Returns the move that leads toward the position with the best evaluation
/// for the selected colour. 
pub fn find_best_move(root : &Node, best_for_white : bool) -> ChessMove {

    assert!(root.children.len() > 0 );

    let mut best_eval : f32  = 0.0;
    let mut best_eval_inx : usize = 0;
    let num_possible_moves = root.children.len();
    for i in 0..num_possible_moves {

        if num_possible_moves == 1 {
            break;
        }

        let mut eval : f32 = 0.0;
        basic_depth_first_search(&root.children[i], best_for_white, &mut eval);

        if ( best_for_white && eval > best_eval ) || 
           ( !best_for_white && eval < best_eval ){
            best_eval = eval;
            best_eval_inx = i;
        }
    }

    let best_move = root.children[best_eval_inx].chess_move_from_parent.clone();
    return best_move;
}

fn basic_depth_first_search(root : &Node, best_for_white : bool, mut best_eval : &mut f32) {
    let num_child_nodes = root.children.len();

    // break condition, this is a leaf node 
    if num_child_nodes == 0 {
        if !root.is_evaluated {
            // Should never reach a leaf that isn't evaluated.
            assert!(false);
        }
        
        if best_for_white && root.evaluation > *best_eval {
            *best_eval = root.evaluation;
        } else if !best_for_white && root.evaluation < *best_eval {
            *best_eval = root.evaluation;
        } 
        return;
    }

    for i in 0..num_child_nodes {
        basic_depth_first_search(&root.children[i], best_for_white, &mut best_eval);
    }

    return;
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

/// // Each node in the search tree stores the position and
/// the leaves of the tree will have an evaluation.
pub struct Node {
    pub children: Vec<Node>,
    pub chess_move_from_parent: ChessMove,
    pub position: Board,
    pub is_evaluated: bool, 
    pub evaluation: f32,
}

impl Node {
    pub fn new(board : &Board, chess_move : &ChessMove) -> Node {
        return Node {
            children : vec![],
            chess_move_from_parent : chess_move.clone(),
            position : board.clone(),
            is_evaluated : false,
            evaluation : 0.0,
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
    use crate::search::{create_search_tree, count_leaves_in_tree, Node};

    // Testing the search and chess engine using known perf results:
    // https://www.chessprogramming.org/Perft_Results 
    #[test]
    fn perft_test_start_pos() {

        let mut board = Board::new();
        let empty_move = ChessMove::new_empty_move();
        let mut root = Node::new(&board, &empty_move);
        let mut num_leaves = 0 as u32;
        let mut num_checks = 0 as u32;
        count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        assert_eq!( num_checks, 0 );
        assert_eq!( num_leaves, 1 );

        create_search_tree(&mut root, 1);
        num_leaves = 0;
        num_checks = 0;
        count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        assert_eq!(num_checks, 0);
        assert_eq!(num_leaves, 20);

        root = Node::new(&board, &empty_move);
        create_search_tree(&mut root, 2);
        num_leaves = 0;
        num_checks = 0;
        count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        assert_eq!(num_checks, 0);
        assert_eq!(num_leaves, 400);

        // root = Node::new(&board, &empty_move);
        // create_search_tree(&mut root, 3);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 12);
        // assert_eq!(num_leaves, 8902); // failing, getting 9194 positions

        // root = Node::new(&board, &empty_move);
        // create_search_tree(&mut root, 4);
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

        create_search_tree(&mut root, 1);
        count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        assert_eq!(num_checks, 0);
        assert_eq!(num_leaves, 48);
       
        // root = Node::new(&board, &empty_move);
        // create_search_tree(&mut root, 2);
        // num_leaves = 0;
        // num_checks = 0;
        // count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);
        // assert_eq!(num_checks, 3);
        // assert_eq!(num_leaves, 2039); // Fails, gets 2083 positions...
    }
}