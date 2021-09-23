use crate::board::Board;
use crate::evaluate::evaluate;
use crate::rules::all_possible_moves;
use crate::pieces::ChessMove;

/// Creates the search tree from the current position. 
/// Returns the root of the tree. The search tree is a binary tree.
fn create_search_tree(mut root : Box<Node>, depth : usize) -> Box<Node> {

    // Recursive function break condition. 
    // The search depth has been reached.
    if depth == 0 {
        root.evaluation = evaluate(&root.position);
        return root;
    }


    let all_possible_moves = all_possible_moves(&root.position);
    let num_possible_moves = all_possible_moves.len();
    let mut first_child_node : Box<Node>;
    for i in 0..num_possible_moves {

        let mut next_position = root.position.clone();
       
        // continue here working on generating the search tree...
        // once this is working, can test the number of nodes in the 
        // tree up to depth. Will likely fail due to no draw from three fold
        // repetition and no dtaw by 50 repeatable moves

        // let mut new_position = root.position.clone();
        // new_position.make_move(possible_move);
        // let 



        // create_search_tree(position );
    }

    // Another break condition when there are no possible moves 
    // from the current position. 
    root.evaluation = evaluate(&root.position);
    return root;
}

/// Each node in the search tree stores the position and
/// the leaves of the tree will have an evaluation.
struct Node {
    left: Box<DirectedEdge>,
    right: Box<DirectedEdge>,
    position: Board,
    evaluation: f32,
}

/// Each edge in the search tree points to the next node
/// in the tree and stores the chess move required to get 
/// there.
struct DirectedEdge {
    next_node: Box<Node>,
    chess_move: ChessMove,
}

#[cfg(test)]
mod tests {
    use crate::console_log;
    use crate::board::Board;
    use crate::search::create_search_tree;

    #[test]
    fn create_tree() {
        //create_search_tree(1 as usize);
        assert!(true);
    }
}