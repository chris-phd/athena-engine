use wasm_bindgen::prelude::*;

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

    pub fn is_move_legal(&mut self, src_rank: i32, src_file: i32, 
        dest_rank: i32, dest_file: i32) -> u8 {
        console_log!("GameState::is_move_legal: todo!");

        let src_rank_file = [src_rank as usize, src_file as usize];
        let dest_rank_file = [dest_rank as usize, dest_file as usize];
        let mut chess_move = ChessMove::new();
        chess_move.set_move(&self.board, src_rank_file, dest_rank_file);

        return rules::is_move_legal(&self.board, &chess_move) as u8;
    }

    pub fn make_move(&mut self, src_rank: i32, src_file: i32, 
        dest_rank: i32, dest_file: i32) {
        console_log!("GameState::make_move: todo!");

        let _src_rank_file = [src_rank as usize, src_file as usize];
        let _dest_rank_file = [dest_rank as usize, dest_file as usize];
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