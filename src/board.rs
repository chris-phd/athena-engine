use crate::console_log;
use crate::utils::log;
use crate::pieces::{ChessMove, MoveType, is_square_attacked, pieces_attacking_square, king_standard_moves};
use crate::rules::{all_possible_moves};

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

    pub fn get_en_passant_square(&self) -> [usize; 2] {
        return self.en_passant_sq;
    }
    
    /// Returns the current board position as an array of ints. 
    /// 0 = empty squares, odd num = black, even num = white
    /// 1, 2 = pawn. 3, 4 = knight. 5, 6 = bishop, 7, 8 = rook, 
    /// 9, 10 = queen. 11, 12 = king
    pub fn get_current_position(&self) -> Vec<u8> {
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

    pub fn is_checkmate(&self) -> bool {
        // Check if the king is in check
        let king_rank_file = self.get_king_rank_file();
        console_log!("king rank and file = {:?}", king_rank_file);
        let is_white = self.white_to_move();
        if !is_square_attacked(&self, king_rank_file, !is_white) {
            return false;
        }

        // Check if the king can move any where. need to modify the board for this
        let possible_moves = king_standard_moves(&self, king_rank_file, is_white, false);
        let mut board_copy = self.clone();
        board_copy.clear_square(king_rank_file);
        for possible_move in possible_moves {
            let dest = possible_move.dest;
            if !is_square_attacked(&board_copy, dest, !is_white) {            
                console_log!("king can move to safety on {:?}", dest);
                return false;
            }
        }

        // Check if the attacking piece can be captured, or if the attack can be
        // intercepted (by moving a friendly piece in front of the attack)
        let attacking_moves = pieces_attacking_square(&self, king_rank_file, !is_white);
        console_log!("num attacking moves = {}", attacking_moves.len());
        for attacking_move in attacking_moves {
            let moves_to_capture_attacker = pieces_attacking_square(&self, attacking_move.src, is_white);
            if moves_to_capture_attacker.len() > 0 {

                console_log!("move to capture attacker src = {:?}", moves_to_capture_attacker[0].src);
                console_log!("move to capture attacker dest = {:?}", moves_to_capture_attacker[0].dest);
                console_log!("move to capture attacker piece = {:?}", moves_to_capture_attacker[0].piece);

                // If this is the king capturing it's own attacker, make sure the king
                // did not move into check.
                if !(moves_to_capture_attacker.len() == 1 && 
                   moves_to_capture_attacker[0].piece.to_ascii_uppercase() == 'K' &&
                   is_square_attacked(&self, moves_to_capture_attacker[0].dest, !is_white) ) {

                    console_log!("attacking piece can be captured");
                    return false;
                }
            }

            if can_attack_be_intercepted(&self, attacking_move) {
                console_log!("attack can be intercepted");
                return false;
            }
        }

        return true;
    }

    pub fn is_draw(&self) -> bool {

        // Check for draw by three fold repetition


        // Check for draw by stalemate
        console_log!("is white to move = ");
        let occupied_squares = self.all_occupied_squares(self.is_white_to_move);
        console_log!("num occupied squares = {}", occupied_squares.len());
        for occupied_square in occupied_squares {
            console_log!("occupied square = {:?}", occupied_square);

            // If there are any moves in the current position, not a stalemate
            if all_possible_moves(&self, occupied_square).len() > 0 {
                console_log!("There is at least one possible move");
                return false;
            }
        }

        return true;
    }

    pub fn make_move(&mut self, chess_move: ChessMove) {
        if (self.is_white_to_move && !chess_move.piece.is_uppercase()) ||
           (!self.is_white_to_move && chess_move.piece.is_uppercase()) {
            return;
        }
        
        self.move_piece(chess_move.src, chess_move.dest);
        self.en_passant_sq = [0, 0];

        match chess_move.move_type {
            MoveType::Standard => { 

                // Check if an enpassant square becomes avaliable
                if chess_move.piece == 'P' && chess_move.src[0] == 2 && chess_move.dest[0] == 4 {
                    self.en_passant_sq = [3, chess_move.src[1]];
                } else if chess_move.piece == 'p' && chess_move.src[0] == 7 && chess_move.dest[0] == 5 {
                    self.en_passant_sq = [6, chess_move.src[1]];
                }


                // Check if the king loses castle rights.
                if chess_move.piece == 'K' {
                    self.castle_king_side_white_avaliable = false;
                    self.castle_queen_side_white_avaliable = false;
                } else if chess_move.piece == 'k' {
                    self.castle_king_side_black_avaliable = false;
                    self.castle_queen_side_black_avaliable = false;
                }

                if self.get_piece_on_square([1, 1]) != 'R' {
                    self.castle_queen_side_white_avaliable = false;
                } else if self.get_piece_on_square([1, 8]) != 'R' {
                    self.castle_king_side_white_avaliable = false;
                } else if self.get_piece_on_square([8, 1]) != 'r' {
                    self.castle_queen_side_black_avaliable = false;
                } else if self.get_piece_on_square([8, 8]) != 'r' {
                    self.castle_king_side_black_avaliable = false;
                }
            } , 
            MoveType::CastleKingSide =>  {
                let rook_src  : [usize; 2] = [chess_move.src[0], 8];
                let rook_dest : [usize; 2] = [rook_src[0], 6];
                assert_eq!(self.get_piece_on_square(rook_src).to_ascii_uppercase(), 'R');
                self.move_piece(rook_src, rook_dest);
                if chess_move.is_white_piece() {
                    self.castle_king_side_white_avaliable = false;
                    self.castle_queen_side_white_avaliable = false;
                } else {
                    self.castle_king_side_black_avaliable = false;
                    self.castle_queen_side_black_avaliable = false;
                }
            },
            MoveType::CastleQueenSide => {
                let rook_src  : [usize; 2] = [chess_move.src[0], 1];
                let rook_dest : [usize; 2] = [rook_src[0], 4];   
                assert_eq!(self.get_piece_on_square(rook_src).to_ascii_uppercase(), 'R');
                self.move_piece(rook_src, rook_dest);
                if chess_move.is_white_piece() {
                    self.castle_queen_side_white_avaliable = false;
                    self.castle_king_side_white_avaliable = false;
                } else {
                    self.castle_queen_side_black_avaliable = false;
                    self.castle_king_side_white_avaliable = false;
                }
            },
            MoveType::EnPassant => {
                if chess_move.piece == 'P' {
                    self.clear_square([chess_move.dest[0] - 1, chess_move.dest[1]]);
                } else if chess_move.piece == 'p' {
                    self.clear_square([chess_move.dest[0] + 1, chess_move.dest[1]]);
                } else {
                    assert!(false);
                }
            },
            MoveType::PromoteToQueen => {
                let promoted_piece : char;
                if chess_move.piece.is_ascii_uppercase() {
                    promoted_piece = 'Q';
                } else {
                    promoted_piece = 'q';
                }
                self.squares[self.square_index(chess_move.dest)] = promoted_piece;
            },
            MoveType::PromoteToRook => {
                let promoted_piece : char;
                if chess_move.piece.is_ascii_uppercase() {
                    promoted_piece = 'R';
                } else {
                    promoted_piece = 'r';
                }
                self.squares[self.square_index(chess_move.dest)] = promoted_piece;
            },
            MoveType::PromoteToBishop => {
                let promoted_piece : char;
                if chess_move.piece.is_ascii_uppercase() {
                    promoted_piece = 'B';
                } else {
                    promoted_piece = 'b';
                }
                self.squares[self.square_index(chess_move.dest)] = promoted_piece;
            },
            MoveType::PromoteToKnight => {
                let promoted_piece : char;
                if chess_move.piece.is_ascii_uppercase() {
                    promoted_piece = 'N';
                } else {
                    promoted_piece = 'n';
                }
                self.squares[self.square_index(chess_move.dest)] = promoted_piece;
            }
        }

        self.is_white_to_move = !self.is_white_to_move;
    }

    /// Returns the piece on the squar, specified by the square index
    pub fn get_piece_by_square_index(&self, square_inx : usize) -> char {
        assert!(square_inx < 64);
        return self.squares[square_inx];
    }
    
    /// Returns the piece on the square specified by a rank and file. 
    pub fn get_piece_on_square(&self, rank_file: [usize; 2]) -> char {
        return self.squares[self.square_index(rank_file)];
    }

    /// Render the board to the console. Only used when running the tests.
    pub fn render(&self) {
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

    /// Change the value of a square without making a move.
    pub fn clear_square(&mut self, rank_file: [usize; 2]) {
        self.squares[self.square_index(rank_file)] = '-';
    }

    /// Returns all the squares occupied by pieces of the specified
    /// colour.
    fn all_occupied_squares(&self, find_occupied_by_white: bool) -> Vec<[usize; 2]> {
        let mut occupied_squares : Vec<[usize; 2]> = vec![];
        for rank in (1 as usize)..(9 as usize) {
            for file in (1 as usize)..(9 as usize) {
                if ( find_occupied_by_white && self.is_occupied_by_white([rank, file]) ) ||
                   (!find_occupied_by_white && self.is_occupied_by_black([rank, file]) ) {
                    occupied_squares.push([rank, file]);
                }
            }
        }

        return occupied_squares;
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

/// Checks if a checkmating attack can be intercepted by another piece
fn can_attack_be_intercepted(board : &Board, attacking_move : ChessMove) -> bool {
    let piece_type = attacking_move.piece.to_ascii_uppercase();
    let is_white = attacking_move.piece.is_uppercase();

    // The attacked piece can not intercept the attack, so remove from the board when
    // checking for intercepts
    let mut board_copy = board.clone();
    board_copy.clear_square(attacking_move.dest);

    // Only moves from sliding pieces can be intercepted.
    if piece_type != 'R' && piece_type != 'B' && piece_type != 'Q' {
        return false;
    }

    console_log!("attacking piece = {}", attacking_move.piece);

    // todo, is there a way to reduce repetition with pieces::is_slide_clear_for_non_capture? 
    let rank_dir = (attacking_move.dest[0] as i32 - attacking_move.src[0] as i32).signum();
    let file_dir = (attacking_move.dest[1] as i32 - attacking_move.src[1] as i32).signum();
    let mut traversed = [(attacking_move.src[0] as i32 + rank_dir) as usize,
                        (attacking_move.src[1] as i32 + file_dir) as usize];

    while traversed[0] != attacking_move.dest[0] || traversed[1] != attacking_move.dest[1] {

        if is_square_attacked(&board_copy, traversed, !is_white) {
            // The sliding attack can be intercepted.
            console_log!("traversed = {:?}", traversed);
            return true;
        }

        traversed[0] = (traversed[0] as i32 + rank_dir) as usize;
        traversed[1] = (traversed[1] as i32 + file_dir) as usize;
    }

    return false;
}

/// Tracks the moves played during a game. Used to find three fold repetition.
struct MovesPlayed {

}

#[cfg(test)]
mod tests {
    use crate::console_log;
    use crate::board::Board;

    #[test]
    fn is_checkmate_test() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/6k1/8/8/8/8/5PPP/2r3K1");
        board.render();
        assert!( board.is_checkmate() );

        board.set_board_from_fen_string("8/6k1/8/8/8/7P/5PP1/2r3K1");
        assert!( !board.is_checkmate() );

        board.set_board_from_fen_string("8/6k1/8/8/8/4N3/5PPP/2r3K1");
        board.render();
        assert!( !board.is_checkmate() );

        board.set_board_from_fen_string("8/6k1/8/8/5B2/8/5PPP/2r3K1");
        assert!( !board.is_checkmate() );

        board.set_board_from_fen_string("5rkb/5pnn/7N/8/8/4K3/8/8");
        board.set_is_white_to_move(false);
        assert!( board.is_checkmate());
        
        board.set_board_from_fen_string("5rkb/5ppn/7N/8/8/4K3/8/8");
        board.set_is_white_to_move(false);
        assert!( !board.is_checkmate());

        // Fix this edge case. King cannot capture attacking piece if it 
        // involves theking moving into check
        board.set_board_from_fen_string("8/8/8/8/8/3K4/3Q4/3k4");
        board.set_is_white_to_move(false);
        board.render();
        assert!( board.is_checkmate() ); 
    }
    
    #[test]
    fn is_stalemate() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/8/p7/P7/5k2/6q1/8/7K");
        board.render();
        assert!( board.is_draw() );

        board.set_board_from_fen_string("8/8/p7/P7/5kq1/8/8/7K");
        assert!( !board.is_draw() );
    }
}