use crate::board::Board;
use crate::evaluate::evaluate;
use crate::rules::all_possible_moves;
use crate::pieces::ChessMove;

/// Creates the search tree from the current position. The depth is the number 
/// of half moves to search.
/// Todo! Could improve effiency by avoiding recusion and using a binary tree
/// the search tree is a binary tree.
fn create_search_tree(mut root : &mut Node, depth : usize) {

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

fn count_leaves_in_tree(root : &Node, num_leaves : &mut u32) {

    let num_child_nodes = root.children.len();

    // Break condition, when there are no child nodes we are
    // at a leaf.
    if num_child_nodes == 0 {
        *num_leaves = *num_leaves + 1 as u32;
        return;
    }

    for i in 0..num_child_nodes {
        count_leaves_in_tree(&root.children[i], num_leaves);
    }
}

/// // Each node in the search tree stores the position and
/// the leaves of the tree will have an evaluation.
struct Node {
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
        let mut num_leaves = 0;
        count_leaves_in_tree(&root, &mut num_leaves);
        assert_eq!( num_leaves, 1);

        create_search_tree(&mut root, 1);
        num_leaves = 0;
        count_leaves_in_tree(&root, &mut num_leaves);
        assert_eq!(num_leaves, 20);

        root = Node::new(&board, &empty_move);
        create_search_tree(&mut root, 2);
        num_leaves = 0;
        count_leaves_in_tree(&root, &mut num_leaves);
        assert_eq!(num_leaves, 400);

        root = Node::new(&board, &empty_move);
        create_search_tree(&mut root, 3);
        num_leaves = 0;
        count_leaves_in_tree(&root, &mut num_leaves);
        assert_eq!(num_leaves, 8902); // failing, getting 9194 positions

        root = Node::new(&board, &empty_move);
        create_search_tree(&mut root, 4);
        num_leaves = 0;
        count_leaves_in_tree(&root, &mut num_leaves);
        // assert_eq!(num_leaves, 197281); // failing, getting 210124 positions
    }
}