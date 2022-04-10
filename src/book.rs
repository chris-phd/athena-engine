use crate::search::Node;
use crate::board::Board;
use crate::console_log;
use crate::utils::{log, coord_to_rank_file};
use crate::pieces::{ChessMove, MoveType};

use std::fs::File;
use std::io::Read;

// If this turns out to be too slow, encode and save the in binary format.
// Returns the root of the search tree.
pub fn read_opening_book(book_filepath: &str) -> Option<Node> {

    // Read the opening from the file. Returns error if unable to read the file.
    let mut file = match File::open(book_filepath) {
        Ok(file) => file,
        Err(e) => return None,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();

    // Assumes the search tree starts in a normal starting position.
    let board = Board::new();
    let mut root = Node::new_root(&board);

    for line in lines {
        // -1 since last move is the game outcome
        let num_moves = line.split_whitespace().collect::<Vec<&str>>().len() - 1; 
        let moves = line.split_whitespace().collect::<Vec<&str>>();
        
        let mut node_this_depth = &mut root;

        // Do not want to read in the full game
        let max_book_depth = 10;
        for i in 0..num_moves {
            let white_to_move = i%2 == 0;
            let pgn_notation = String::from(moves[i]);
            let mut chess_move = ChessMove::new_empty_move();
            chess_move.set_move_from_pgn_notation(&node_this_depth.position, &pgn_notation, white_to_move);
            
            if chess_move.move_type == MoveType::Invalid {
                console_log!("book::read_opening_book: Invalid move type {:?}", pgn_notation);
                panic!();
            }

            let num_children = node_this_depth.children.len();
            let mut found_matching_move = false;
            for j in 0..num_children {
                
                // If the move already exists in the opening book, don't need to add it again. 
                let move_in_tree = node_this_depth.children[j].chess_move_from_parent;
                if move_in_tree.is_the_same_as(&chess_move) {
                    found_matching_move = true;
                    node_this_depth = &mut node_this_depth.children[j];
                    break;
                }
            }

            // If the move is not already in the opening book, add it
            if !found_matching_move {
                let mut next_position = node_this_depth.position.clone();
                next_position.make_move(chess_move);
                node_this_depth.children.push(Node::new(&next_position, &chess_move));
                node_this_depth = &mut node_this_depth.children[num_children as usize]; // get last element
            }

            if i == max_book_depth - 1 {
                break;
            }
        }
    }

    return Some(root);
}

#[cfg(test)]
mod tests {

    use crate::console_log;
    use crate::book::read_opening_book;
    use crate::search::count_leaves_in_tree;

    #[test]
    fn read_test_file() {
        let path_1 = "static/resources/test_games_1.pgn";
        if let Some(root) = read_opening_book(path_1) {

            let mut num_leaves : u32 = 0;
            let mut num_checks : u32 = 0;
            count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);

            assert!(num_leaves == 1);
        } else {
            console_log!("Failed to open {:?}", path_1);
            assert!(false);
        }

        let path_2 = "static/resources/test_games_2.pgn";
        if let Some(root) = read_opening_book(path_2) {

            let mut num_leaves : u32 = 0;
            let mut num_checks : u32 = 0;
            count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);

            assert!(num_leaves == 2);
        } else {
            console_log!("Failed to open {:?}", path_2);
            assert!(false);
        }
    }

    #[test]
    fn read_broken_games() {
        let path_1 = "static/resources/test_games_3.pgn";
        if let Some(root) = read_opening_book(path_1) {

            let mut num_leaves : u32 = 0;
            let mut num_checks : u32 = 0;
            count_leaves_in_tree(&root, &mut num_leaves, &mut num_checks);

            assert!(num_leaves == 1);
        } else {
            console_log!("Failed to open {:?}", path_1);
            assert!(false);
        }
    }

    #[test]
    fn test_read_opening_book() {
        let path_to_book = "static/resources/games.pgn";
        if let Some(root) = read_opening_book(path_to_book) {

        } else {
            console_log!("Failed to open {:?}", path_to_book);
            assert!(false);
        }
    }
}