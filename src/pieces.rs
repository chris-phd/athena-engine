use crate::console_log;
use crate::utils::log;
use crate::board::Board;

pub struct ChessMove {
    src_rank_file: [usize; 2],
    dest_rank_file: [usize; 2],
}

impl ChessMove {
    pub fn new() -> ChessMove {
        return ChessMove {
            src_rank_file: [0, 0],
            dest_rank_file: [0, 0],
        }
    }

    pub fn set_move(&mut self, _board: &Board, src_rank_file: [usize; 2], 
        dest_rank_file: [usize; 2]) {
        console_log!("pieces::ChessMove::set_move: todo!");
        
        self.src_rank_file = src_rank_file;
        self.dest_rank_file = dest_rank_file;

        // TODO, check the current board position to see if this move
        // is a special move. en_passant / castle move / promotion.
    }

    pub fn src(&self) -> [usize; 2] {
        return self.src_rank_file;
    }

    pub fn dest(&self) -> [usize; 2] {
        return self.dest_rank_file;
    }
}