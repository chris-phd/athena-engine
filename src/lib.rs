use wasm_bindgen::prelude::*;
use js_sys::Math;
extern crate console_error_panic_hook;

mod board;
mod engine;
mod pieces;
mod players;
mod utils;
mod rules;

use board::{Board, Position};
use players::{Player, HumanPlayer, ComputerPlayer};
use utils::log;
use pieces::ChessMove;
use utils::coord_to_rank_file;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// GameState. The interface between the javascript front end and the 
/// Rust backend. Stores the current game information such as the board
/// and the current players. 
#[wasm_bindgen]
pub struct GameState {
    board : Board,
    white_player : Box<dyn Player>,
    black_player : Box<dyn Player>,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        console_log!("GameState::new:");

        // For better rust error messages from the browser
        console_error_panic_hook::set_once();

        return GameState {
            board: Board::new(Position::StartPosition),
            white_player: Box::new(HumanPlayer::new()),
            black_player: Box::new(ComputerPlayer::new()),
        };
    }

    /// Called by js front end to update the board visible to the 
    /// human user after the engine makes a move / updates the board
    pub fn get_board(&mut self) -> Vec<u8> {
        console_log!("GameState::get_board: todo!");
        return self.board.get_current_position();
    }

    pub fn set_board(&mut self, fen_string: &str) {
        console_log!("GameState::set_board: todo!");
        self.board.set_board_from_fen_string(fen_string);
    }

    pub fn is_move_legal(&mut self, src_coords: &str, dest_coords: &str) -> u8 {
        console_log!("GameState::is_move_legal: todo!");

        let src_rank_file = coord_to_rank_file(src_coords);
        let dest_rank_file = coord_to_rank_file(dest_coords);
        let chess_move = ChessMove::new(&self.board, src_rank_file, dest_rank_file);
        return rules::is_move_legal(&self.board, &chess_move) as u8;
    }

    /// Enters the move entered by a human player
    pub fn make_move(&mut self, src_coords: &str, dest_coords: &str) {
        console_log!("GameState::make_move: todo!");

        let src_rank_file = coord_to_rank_file(src_coords);
        let dest_rank_file = coord_to_rank_file(dest_coords);
        let chess_move = ChessMove::new(&self.board, src_rank_file, dest_rank_file);
        self.board.make_move(chess_move);

    }

    /// Calculates and makes a move if it is a computer player's turn to move
    pub fn make_computer_move(&mut self) {
        console_log!("lib::GameState::make_computer_move:");
        let chess_move : ChessMove;
        if self.board.white_to_move() {
            chess_move = self.white_player.make_move(&self.board);
        } else {
            chess_move = self.black_player.make_move(&self.board);
        }
        self.board.make_move(chess_move);
    }

    /// Returns true if it is a computer's turn to move next
    pub fn is_computer_move(&self) -> bool {
        console_log!("GameState::is_computer_move: ");

        if rules::is_checkmate(&self.board) {
            return false;
        }

        let computer_move: bool;
        if self.board.white_to_move() {
            computer_move = self.white_player.is_computer();
        } else {
            computer_move = self.black_player.is_computer();
        }

        return computer_move;
    }

    /// Renders the current gamestate board in ascii text
    pub fn render_ascii(&self) {
        self.board.render();
    }

    /// Set the players from the js front end
    /// 0 = Human Player, 1 = Computer Player
    pub fn set_players(&mut self, white: i32, black: i32) {
        if white == 0 {
            self.white_player = Box::new(HumanPlayer::new());
        } else {
            self.white_player = Box::new(ComputerPlayer::new());
        }

        if black == 0 {
            self.black_player = Box::new(HumanPlayer::new());
        } else {
            self.black_player = Box::new(ComputerPlayer::new());
        }
    }
}