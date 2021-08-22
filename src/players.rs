use crate::console_log;
use crate::utils::log;
use crate::board::Board;
use crate::pieces::ChessMove;
use crate::engine;

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

}

impl ComputerPlayer {
    pub fn new() -> ComputerPlayer {
        console_log!("ComputerPlayer::new:");
        return ComputerPlayer {};
    }
}

/// The Player interface that both the Human Player and Computer Player
/// struct implement.
pub trait Player {
    fn make_move(&self, board: &Board) -> ChessMove;
    fn is_computer(&self) -> bool;
}

impl Player for HumanPlayer {
    fn make_move(&self, _board: &Board) -> ChessMove {
        // Should never reach this. Human moves are entered by the web GUI.
        console_log!("HumanPlayer::make_move: Should never call this method!");
        return ChessMove::new_empty_move();
    }

    fn is_computer(&self) -> bool {
        return false;
    }
}

impl Player for ComputerPlayer {
    fn make_move(&self, board: &Board) -> ChessMove {
        console_log!("players::ComputerPlayer::make_move: todo! ");
        return engine::best_move(board);
    }

    fn is_computer(&self) -> bool {
        return true;
    }
}