use crate::console_log;
use crate::utils::log;
use crate::board::Board;
use crate::pieces::ChessMove;
use crate::engine;
use crate::search::Node;
use crate::book::read_opening_book;

/// HumanPlayer: Moves are entered via the web UI.
pub struct HumanPlayer {

}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        console_log!("HumanPlayer::new:");
        return HumanPlayer {};
    }
}

/// ComputerPlayer: Moves are calculated by the chess engine.
pub struct ComputerPlayer {
    opening_book: Node,
}

impl ComputerPlayer {
    pub fn new() -> ComputerPlayer {
        console_log!("ComputerPlayer::new:");
        let max_depth: usize = 10;
        let mut maybe_book = read_opening_book(max_depth);
        return match maybe_book {
            Some(book) => { 
                console_log!("    Successfully read the opening book!");
                ComputerPlayer {
                opening_book: book,
            }},
            _=> { 
                console_log!("    Failed to read the opening book!");
                let board = Board::new();
                ComputerPlayer {
                opening_book: Node::new_root(&board),
            }}
        }
    }
}

/// The Player interface that both the Human Player and Computer Player
/// struct implement.
pub trait Player {
    fn make_move(&mut self, board: &Board) -> ChessMove;
    fn is_computer(&self) -> bool;
}

impl Player for HumanPlayer {
    fn make_move(&mut self, _board: &Board) -> ChessMove {
        // Should never reach this. Human moves are entered by the web GUI.
        assert!(false);
        return ChessMove::new_empty_move();
    }

    fn is_computer(&self) -> bool {
        return false;
    }
}

impl Player for ComputerPlayer {
    fn make_move(&mut self, board: &Board) -> ChessMove {
        console_log!("players::ComputerPlayer::make_move: ");

        if let Some(book_move) = engine::move_from_opening_book(&mut self.opening_book, &board) {
            return book_move;
        }

        let depth = 3;
        return engine::best_move(&board, depth);
    }

    fn is_computer(&self) -> bool {
        return true;
    }
}