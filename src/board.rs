use crate::console_log;
use crate::utils::log;
use crate::pieces::{ChessMove, MoveType};

/// The Chess Board. Stores the position of the chess pieces.
#[derive(Clone, Copy)]
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
    pub fn new() -> Board {
        console_log!("board::Board::new: ");
        let set_squares: [char; 64] = ['-'; 64];

        let mut board = Board {
            squares: set_squares,
            is_white_to_move: true,
            en_passant_sq: [0, 0],
            castle_king_side_white_avaliable: true,
            castle_king_side_black_avaliable: true,
            castle_queen_side_white_avaliable: true,
            castle_queen_side_black_avaliable: true,
        };

        board.set_board_from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        return board;
    }

    /// Sets the squares from a fen string
    /// See https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn set_board_from_fen_string(&mut self, fen_string: &str) {
        self.squares = ['-'; 64];

        let mut rank = 8 as usize;
        let mut file = 1 as usize;
        let mut finished_piece_positions = false;
        for ch in fen_string.chars() {

            if !finished_piece_positions {
                if ch.is_ascii_digit() {
                    file += ch as usize - '0' as usize;
                } if ch.is_ascii_alphabetic() {
                    let piece = ch;
                    self.set_piece(piece, [rank, file]);
                    file += 1 as usize;
                } else if ch == '/' {
                    rank -= 1 as usize;
                    file = 1 as usize;
                } else if ch == ' ' {
                    // Piece positions have been set. 
                    // TODO! Add section to deal with 
                    // who is to move and castle avaliability
                    finished_piece_positions = true;
                }
            }
        }
    }

    pub fn is_castle_king_side_avaliable(&self, is_white: bool) -> bool {
        return (is_white && self.castle_king_side_white_avaliable) ||
                (!is_white && self.castle_king_side_black_avaliable);
    }

    pub fn is_castle_queen_side_avaliable(&self, is_white: bool) -> bool {
        return (is_white && self.castle_queen_side_white_avaliable) ||
                (!is_white && self.castle_queen_side_black_avaliable);
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
        if (self.is_white_to_move && !chess_move.piece.is_uppercase()) ||
           (!self.is_white_to_move && chess_move.piece.is_uppercase()) {
            return;
        }
        
        self.move_piece(chess_move.src, chess_move.dest);

        match chess_move.move_type {
            MoveType::Standard => { } , // TODO! check if an enpassant square becomes avaliable or if the king loses caste lrights 
            MoveType::CastleKingSide =>  {
                let rook_src  : [usize; 2] = [chess_move.src[0], 8];
                let rook_dest : [usize; 2] = [rook_src[0], 6];
                assert_eq!(self.get_piece_on_square(rook_src).to_ascii_uppercase(), 'R');
                self.move_piece(rook_src, rook_dest);
                if chess_move.is_white_piece() {
                    self.castle_king_side_white_avaliable = false;
                } else {
                    self.castle_king_side_black_avaliable = false;
                }
            },
            MoveType::CastleQueenSide => {
                let rook_src  : [usize; 2] = [chess_move.src[0], 1];
                let rook_dest : [usize; 2] = [rook_src[0], 4];   
                assert_eq!(self.get_piece_on_square(rook_src).to_ascii_uppercase(), 'R');
                self.move_piece(rook_src, rook_dest);
                if chess_move.is_white_piece() {
                    self.castle_queen_side_white_avaliable = false;
                } else {
                    self.castle_queen_side_black_avaliable = false;
                }
            },
        }

        self.is_white_to_move = !self.is_white_to_move;
    }

    /// Returns the piece on the square specified by a rank and file. 
    pub fn get_piece_on_square(&self, rank_file: [usize; 2]) -> char {
        return self.squares[self.square_index(rank_file)];
    }

    /// Render the board to the console. Only used when running the tests.
    pub fn render(&self) {
        console_log!("board::Board::render: todo!");
        for rank in (1..=8).rev() {
            for file in 1..=8 {
                eprint!(" {} ", self.get_piece_on_square([rank, file]));
            }
            eprintln!("");
        }
    }

    pub fn white_to_move(&self) -> bool {
        return self.is_white_to_move;
    }

    pub fn set_is_white_to_move(&mut self, is_white_to_move: bool) {
        self.is_white_to_move = is_white_to_move;
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

    pub fn is_occupied_by_king(&self, rank_file: [usize; 2]) -> bool {
        let piece = self.get_piece_on_square(rank_file);
        return piece == 'k' || piece == 'K';
    }

    pub fn get_king_rank_file(&self) -> [usize; 2] {
        for rank in (1 as usize)..9 {
            for file in (1 as usize)..9 {
                let piece = self.get_piece_on_square([rank, file]);
                if ( self.is_white_to_move && piece == 'K' )  ||
                   (!self.is_white_to_move && piece == 'k') {
                    return [rank, file];
                }
            }
        }

        // King not found. Return an invalid rank and file
        return [0, 0];
    }

    pub fn is_valid_rank_file(&self, rank_file: [usize; 2]) -> bool {
        return !(rank_file[0] > 8 || rank_file[0] < 1 || rank_file[1] > 8 || rank_file[1] < 1);
    }

    /// Change the value of a square without making a move. Should never be
    /// called to deal with a player move, used to remove pieces when validating checkmates.
    pub fn clear_square(&mut self, rank_file: [usize; 2]) {
        self.squares[self.square_index(rank_file)] = '-';
    }

    /// Moves the piece from src to dest, and leaves the src square empty
    fn move_piece(&mut self, src: [usize ; 2], dest: [usize; 2]) {
        let dest_index = self.square_index(dest);
        let src_index = self.square_index(src);
        self.squares[dest_index] = self.squares[src_index];
        self.squares[src_index] = '-';
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

// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
fn _start_position() -> [char; 64] {
    return ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r',
            'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P',
            'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
}

// 8/3r4/8/3q1P2/8/8/6np/5k1Q
fn _test_queen() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'r', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'q', '-', 'P', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', 'n', 'p',
            '-', '-', '-', '-', '-', 'k', '-', 'Q',];
}

// 8/8/8/8/8/8/3r1PPP/R3K2R 
fn _test_king() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'r', '-', 'P', 'P', 'P',
            'R', '-', '-', '-', 'K', '-', '-', 'R',];
}

// 8/8/8/8/2N5/P7/1P1r2pp/7n
fn _test_knight() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', 'N', '-', '-', '-', '-', '-',
            'P', '-', '-', '-', '-', '-', '-', '-',
            '-', 'P', '-', 'r', '-', '-', 'p', 'p',
            '-', '-', '-', '-', '-', '-', '-', 'n',];
}

// 8/4p2p/4K3/8/2n5/1P6/6P1/8
fn _test_pawn() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', 'p', '-', '-', 'p',
            '-', '-', '-', '-', 'K', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', 'n', '-', '-', '-', '-', '-',
            '-', 'P', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', 'P', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',];
}

// 8/3p4/8/3r1P2/8/8/6np/5Q1R
fn _test_rook() -> [char; 64] {
    return ['-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'p', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', 'r', '-', 'P', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', '-', '-',
            '-', '-', '-', '-', '-', '-', 'n', 'p',
            '-', '-', '-', '-', '-', 'Q', '-', 'R',];
}