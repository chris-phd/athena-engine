use crate::console_log;
use crate::utils::log;
use crate::pieces::ChessMove;

/// The Chess Board. Stores the position of the chess pieces.
pub struct Board {
    squares : [char; 64],
    is_white_to_move : bool,
    en_passant_sq : [usize; 2],
    castle_king_side_white_avaliable : bool,
    castle_king_side_black_avaliable : bool,
    castle_queen_side_white_avaliable : bool,
    castle_queen_side_black_avaliable : bool,
}

impl Board {
    pub fn new(position: Position) -> Board {
        console_log!("board::Board::new: ");
        let set_squares: [char; 64];
        match position {
            Position::StartPosition => set_squares = start_position(),
            Position::TestQueen => set_squares = test_queen(),
            Position::TestKing => set_squares = test_king(),
            Position::_TestKnight => set_squares = test_knight(),
            Position::TestPawn => set_squares = test_pawn(),
        }

        return Board {
            squares: set_squares,
            is_white_to_move: true,
            en_passant_sq: [0, 0],
            castle_king_side_white_avaliable: true,
            castle_king_side_black_avaliable: true,
            castle_queen_side_white_avaliable: true,
            castle_queen_side_black_avaliable: true,
        };
    }

    /// Sets the squares from a fen string
    /// See https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn set_board_from_fen_string(&mut self, fen_string: &str) {
        self.clear_board();

        let mut rank = 8 as usize;
        let mut file = 1 as usize;
        let mut finished_piece_positions = false;
        for ch in fen_string.chars() {

            if !finished_piece_positions {
                if ch.is_ascii_digit() {

                } else if ch.is_ascii_alphabetic() {
                    rank -= 1 as usize;
                    file = 1 as usize;
                } else if ch == '/' {
                    let piece = ch;
                    self.set_piece(piece, [rank, file]);
                    file += 1 as usize;
                } else if ch == ' ' {
                    // Piece positions have been set, now deal with 
                    // who is to move and castle avaliability
                    finished_piece_positions = true;
                }
            } else {
                // handle the white to move, castle avaliable characters
            }
        }
    }

    /// Returns the current board position as an array of ints. 
    /// 0 = empty squares, odd num = black, even num = white
    /// 1, 2 = pawn. 3, 4 = knight. 5, 6 = bishop, 7, 8 = rook, 
    /// 9, 10 = queen. 11, 12 = king
    pub fn get_current_position(&self) -> Vec<u8> {
        console_log!("board::Board::get_current_position:");
        let mut current_position = vec![0 as u8; 64];
        for i in 0..64 {
            match self.squares[i] {
                'p' => current_position[i] = 1,
                'P' => current_position[i] = 2,
                'n' => current_position[i] = 3,
                'N' => current_position[i] = 4,
                'b' => current_position[i] = 5,
                'B' => current_position[i] = 6,
                'r' => current_position[i] = 7,
                'R' => current_position[i] = 8,
                'q' => current_position[i] = 9,
                'Q' => current_position[i] = 10,
                'k' => current_position[i] = 11,
                'K' => current_position[i] = 12,
                _   => current_position[i] = 0,
            }
        }
        return current_position;
    }

    pub fn make_move(&mut self, chess_move: ChessMove) {
        console_log!("board::Board::make_move: Finish implementing me!");

        let dest_index = self.square_index(chess_move.dest());
        let src_index = self.square_index(chess_move.src());
        self.squares[dest_index] = self.squares[src_index];
        self.squares[src_index] = '-';
        // todo! create the cases for the special moves (promotions, 
        // castles, en passant etc.)

        self.is_white_to_move = !self.is_white_to_move;
    }

    /// Returns the piece on the square specified by a rank and file. 
    pub fn get_piece_on_square(&self, rank_file: [usize; 2]) -> char {
        return self.squares[self.square_index(rank_file)];
    }

    /// IMPLEMENT ME! This will be important when validating that the 
    /// front end matches the back end...
    pub fn render(&self) {
        console_log!("board::Board::render: todo!");
    }

    pub fn white_to_move(&self) -> bool {
        return self.is_white_to_move;
    }

    /// Methods for checking if a square is free
    pub fn is_occupied(&self, rank_file: [usize; 2]) -> bool {
        let piece = self.get_piece_on_square(rank_file);
        return piece != '-';
    }

    pub fn is_occupied_by_white(&self, rank_file: [usize; 2]) -> bool {
        if !self.is_occupied(rank_file) {
            return false;
        }
        let is_white = self.get_piece_on_square(rank_file).is_uppercase();
        return is_white;
    }

    pub fn is_occupied_by_black(&self, rank_file: [usize; 2]) -> bool {
        if !self.is_occupied(rank_file) {
            return false;
        }
        let is_black = !self.get_piece_on_square(rank_file).is_uppercase();
        return is_black;
    }

    /// Sets the piece at the square. By convention, uppercase is white,
    /// lowercase is a black piece.
    fn set_piece(&mut self, piece: char, rank_file: [usize; 2]) {
        self.squares[self.square_index(rank_file)] = piece;
    }

    /// Clears the board of all pieces. Resets en passant square
    /// and castle avaliability
    fn clear_board(&mut self) {
        self.squares = ['-'; 64];
        self.is_white_to_move;
        self.en_passant_sq = [0, 0];
        self.castle_king_side_white_avaliable = true;
        self.castle_king_side_black_avaliable = true;
        self.castle_queen_side_white_avaliable = true;
        self.castle_queen_side_black_avaliable = true;
    }

    /// Convert the rank and file to the corresponding square index.
    /// Index from top left --> bottom right: a8, b8, c8 ... f1, g1, h1
    fn square_index(&self, rank_file : [usize; 2]) -> usize {
        return (8 - 1 - (rank_file[0]-1))*8 + (rank_file[1]-1);
    }

}

/// Chess board position pre-sets
/// TODO! Replace these with fenStrings and write a 
/// set_board_from_fen function. fenStrings will be used
/// to pass the state of the board back and forth between
/// the js front end and rust backend.

/// Then work on passing human moves back and forth between
/// the front end and get the board to update... Do this after
/// writing the stub for the LegalMoveEnforcer and the Engine??
pub enum Position {
    StartPosition,
    TestQueen,
    TestKing,
    _TestKnight,
    TestPawn,
}

fn start_position() -> [char; 64] {
    return ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r',
            'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P',
            'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
}

fn test_queen() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'r', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'q', '-', 'P', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',];
}

fn test_king() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'r', '-', 'P', 'P', 'P',
            'R', '-', '-', '-', 'K', '-', '-', 'R',];
}

fn test_knight() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', 'B',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', 'n', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', 'N', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', 'P',
            '-', '-', '-', 'r', '-', 'P', 'P', '-',
            '-', '-', 'R', '-', '-', '-', '-', '-',];
}

fn test_pawn() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', 'p', '-', '-', '-',
            '-', '-', '-', '-', 'K', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', 'n', '-', '-', '-', '-', '-',
            '-', 'P', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', 'P', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',];
}