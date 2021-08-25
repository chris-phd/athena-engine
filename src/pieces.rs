use crate::console_log;
use crate::utils::log;
use crate::board::Board;

/// DeltaRankFile. Defines one possible piece movement as a 
/// change in rank and file from the current square. 
pub struct DeltaRankFile {
    delta_rank: i32, 
    delta_file: i32,
}

impl DeltaRankFile {
    /// Returns an invalid square [0, 0] if destination square is off the board
    pub fn dest_from_src(&self, src_rank_file: [usize; 2]) -> [usize; 2] {
        let dest = [(src_rank_file[0] as i32 + self.delta_rank),
                   (src_rank_file[1] as i32 + self.delta_file)];
        if dest[0] > 8 || dest[0] < 1 || dest[1] > 8 || dest[1] < 1 {
            return [0, 0];
        }
        
        return [dest[0] as usize, dest[1] as usize];
    }
}

/// ChessMove, represents a move made by a player
#[derive(Clone, Copy)]
pub struct ChessMove {
    src_rank_file: [usize; 2],
    dest_rank_file: [usize; 2],
    piece: char,
}

impl ChessMove {
    pub fn new(board: &Board, src: [usize; 2], dest: [usize; 2]) -> ChessMove {
        let mut new_move = ChessMove::new_empty_move();
        new_move.set_move(&board, src, dest);
        return new_move;
    }

    pub fn new_empty_move() -> ChessMove {
        return ChessMove {
            src_rank_file: [0, 0],
            dest_rank_file: [0, 0],
            piece: '-',
        }
    }

    pub fn set_move(&mut self, board: &Board, src_rank_file: [usize; 2], 
        dest_rank_file: [usize; 2]) {
        console_log!("pieces::ChessMove::set_move: todo!");
        
        self.src_rank_file = src_rank_file;
        self.dest_rank_file = dest_rank_file;
        self.piece = board.get_piece_on_square(src_rank_file);

        // TODO, check the current board position to see if this move
        // is a special move. en_passant / castle move / promotion.
    }

    pub fn src(&self) -> [usize; 2] {
        return self.src_rank_file;
    }

    pub fn dest(&self) -> [usize; 2] {
        return self.dest_rank_file;
    }

    pub fn is_the_same_as(&self, that: &ChessMove) -> bool {

        let is_same_squares = self.src_rank_file == that.src_rank_file && 
                            self.dest_rank_file == that.dest_rank_file;
        let is_same_piece = self.piece == that.piece;
        return is_same_piece && is_same_squares;
    }
}

/// Returns all possible pawn moves from a given square
pub fn pawn_moves(board: &Board, rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    console_log!("pieces::pawn_moves:");
    let mut all_possible_moves = pawn_capture_moves(&board, rank_file, is_white);
    all_possible_moves.append(&mut pawn_non_capture_moves(&board, rank_file, is_white));
    return all_possible_moves;
}

/// Returns all possible pawn captures from a given square
fn pawn_capture_moves(board: &Board, rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {

    let mut non_capture_moves : Vec<ChessMove> = vec![];

    let capture_movements;
    if is_white {
        capture_movements = vec![
            DeltaRankFile {delta_rank:  1, delta_file: -1},
            DeltaRankFile {delta_rank:  1, delta_file:  1},
        ];
    } else {
        capture_movements = vec![
            DeltaRankFile {delta_rank:  -1, delta_file: -1},
            DeltaRankFile {delta_rank:  -1, delta_file:  1},
        ];
    }

    for capture_movement in capture_movements {
        let dest_rank_file = capture_movement.dest_from_src(rank_file);
        if board.is_valid_rank_file(dest_rank_file) &&
           is_capture(&board, dest_rank_file, is_white) {
            let possible_move = ChessMove::new(&board, rank_file, dest_rank_file);
            non_capture_moves.push(possible_move);
        }
    }

    return non_capture_moves;
}

/// Returns all possible non capture pawn moves from a given square. 
fn pawn_non_capture_moves(board: &Board, src_rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let mut non_capture_moves : Vec<ChessMove> = vec![];

    let non_capture_movements;
    if is_white {
        non_capture_movements = vec![
            DeltaRankFile {delta_rank:  1, delta_file:  0},
            DeltaRankFile {delta_rank:  2, delta_file:  0},
        ];
    } else {
        non_capture_movements = vec![
            DeltaRankFile {delta_rank:  -1, delta_file:  0},
            DeltaRankFile {delta_rank:  -2, delta_file:  0},
        ];
    }

    for non_capture_movement in non_capture_movements {
        let dest_rank_file = non_capture_movement.dest_from_src(src_rank_file);

        console_log!("    pawn move dest = {:?}", dest_rank_file);

        if !board.is_valid_rank_file(dest_rank_file) ||
           board.is_occupied(dest_rank_file) {
            continue;
        }
        if is_slide_clear_for_non_capture(&board, src_rank_file, dest_rank_file) {
            let possible_move = ChessMove::new(&board, src_rank_file, dest_rank_file);
            non_capture_moves.push(possible_move);
        }
    }

    return non_capture_moves;
}

/// Returns all knight moves from a given square
pub fn knight_moves(board: &Board, src_rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    console_log!("pieces::knight_moves:");
    let mut all_possible_moves : Vec<ChessMove> = vec![];

    let movements = vec![
        DeltaRankFile { delta_rank: -1, delta_file:  2, },
        DeltaRankFile { delta_rank:  1, delta_file:  2, },
        DeltaRankFile { delta_rank:  1, delta_file: -2, },
        DeltaRankFile { delta_rank: -1, delta_file: -2, },
        DeltaRankFile { delta_rank: -2, delta_file:  1, },
        DeltaRankFile { delta_rank:  2, delta_file:  1, },
        DeltaRankFile { delta_rank:  2, delta_file: -1, },
        DeltaRankFile { delta_rank: -2, delta_file: -1, },
    ];

    for movement in movements {
        let dest_rank_file = movement.dest_from_src(src_rank_file);
        if board.is_valid_rank_file(dest_rank_file) && 
           (!board.is_occupied(dest_rank_file) || is_capture(&board, dest_rank_file, is_white) ) {
            let possible_move = ChessMove::new(&board, src_rank_file, dest_rank_file);
            all_possible_moves.push(possible_move);
        }
    }

    return all_possible_moves;
}

/// Returns all the bishop moves from a given square
pub fn bishop_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    console_log!("pieces::bishop_moves: ");
    let movements = vec![
        DeltaRankFile { delta_rank: -1, delta_file:  1, },
        DeltaRankFile { delta_rank:  1, delta_file:  1, },
        DeltaRankFile { delta_rank:  1, delta_file: -1, },
        DeltaRankFile { delta_rank: -1, delta_file: -1, },
    ];

    return slide_moves(movements, &board, src, is_white);
}

/// Returns all possible rook moves from a given square
pub fn rook_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    console_log!("pieces::rook_moves: ");
    let movements = vec![
        DeltaRankFile { delta_rank: -1, delta_file:  0, },
        DeltaRankFile { delta_rank:  0, delta_file:  1, },
        DeltaRankFile { delta_rank:  0, delta_file: -1, },
        DeltaRankFile { delta_rank:  1, delta_file:  0, },
    ];

    return slide_moves(movements, &board, src, is_white);
}

/// Returns all possible queen moves from a given square
pub fn queen_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    console_log!("pieces::queen_moves:");
    let mut all_possible_moves = rook_moves(&board, src, is_white);
    all_possible_moves.append(&mut bishop_moves(&board, src, is_white));
    return all_possible_moves;
}

/// Generates all possible moves from a square by sliding in the given directions.
/// Used to generate the bishop and rook moves
fn slide_moves(movements: Vec<DeltaRankFile>, board: &Board, src: [usize; 2], is_white: bool)-> Vec<ChessMove> {
    let mut all_possible_moves : Vec<ChessMove> = vec![];

    for movement in movements {
        let mut dest = movement.dest_from_src(src);
        while board.is_valid_rank_file(dest) && 
              (!board.is_occupied(dest) || is_capture(&board, dest, is_white) ) {

            let possible_move = ChessMove::new(&board, src, dest);
            all_possible_moves.push(possible_move);
            
            if (is_capture(&board, dest, is_white)) {
                break;
            }

            dest = movement.dest_from_src(dest);
        }
    }

    return all_possible_moves;
}

/// Checks if a square is occupied by a piece of a different colour, and if it is 
/// occupied by the king. The king can never be captured.
fn is_capture(board : &Board, dest_rank_file: [usize; 2], is_white: bool) -> bool {
    return ((is_white && board.is_occupied_by_black(dest_rank_file)) ||
           (!is_white && board.is_occupied_by_white(dest_rank_file)) ) && 
           !board.is_occupied_by_king(dest_rank_file);
}


/// Checks if a slide move is clear of other pieces. Slide moves
/// handle movements for rooks, bishops, queens. 
fn is_slide_clear_for_non_capture(board: &Board, src: [usize; 2], dest: [usize; 2]) -> bool {
    let rank_dir = (dest[0] as i32 - src[0] as i32).signum();
    let file_dir = (dest[1] as i32 - src[1] as i32).signum();
    let mut traversed = [(src[0] as i32 + rank_dir) as usize,
                        (src[1] as i32 + file_dir) as usize];

    while traversed[0] != dest[0] || traversed[1] != dest[1] {

        if board.is_occupied(traversed) {
            return false;
        }

        // Check if we have traversed past the edge of the board
        if !board.is_valid_rank_file(traversed) {
            console_log!("[pieces::is_slide_clear]: ERROR! Tried to check a square off the edge of the board.");
            return false;
        }

        traversed[0] = (traversed[0] as i32 + rank_dir) as usize;
        traversed[1] = (traversed[1] as i32 + file_dir) as usize;
    }

    return true;                           
}

/// Checks that the slide move is clear without checking the
/// destination square. 
fn is_slide_clear_for_capture(board: &Board, src: [usize; 2], dest: [usize; 2]) -> bool {
    if src[0] == dest[0] && src[1] == dest[1] {
        return true;
    }
    
    let rank_dir = (dest[0] as i32 - src[0] as i32).signum();
    let file_dir = (dest[1] as i32 - src[1] as i32).signum();
    let new_dest = [(dest[0] as i32 - rank_dir) as usize,
                              (dest[1] as i32 - file_dir) as usize];

    return is_slide_clear_for_non_capture(&board, src, new_dest);
}

#[cfg(test)]
mod tests {
    use crate::console_log;
    use crate::pieces::ChessMove;
    use crate::board::{Board, Position};
    use crate::pieces::{self};

    #[test]
    fn is_slide_clear() {

        let board = Board::new(Position::TestQueen);
        let src = [5 as usize, 4 as usize];
        let mut dest = [8 as usize, 4 as usize];
        assert!( !pieces::is_slide_clear_for_non_capture(&board, src, dest) );

        dest = [5 as usize, 6 as usize];
        assert!( pieces::is_slide_clear_for_capture(&board, src, dest) );
    }


    #[test]
    fn possible_pawn_moves() {
        let board = Board::new(Position::TestPawn);
        let mut src = [5 as usize, 4 as usize];
        let mut is_white = true;
        assert_eq!( pieces::pawn_moves(&board, src, is_white).len(), 2);

        src = [2 as usize, 7 as usize];
        is_white = true;
        assert_eq!( pieces::pawn_moves(&board, src, is_white).len(), 2);

        src = [7 as usize, 5 as usize];
        is_white = false;
        assert_eq!( pieces::pawn_moves(&board, src, is_white).len(), 0);


        src = [7 as usize, 8 as usize];
        is_white = false;
        assert_eq!( pieces::pawn_moves(&board, src, is_white).len(), 2);
    }

    #[test]
    fn possible_knight_moves() {
        let board = Board::new(Position::TestKnight);
        let mut src = [4 as usize, 3 as usize];
        let mut is_white = true;
        assert_eq!( pieces::knight_moves(&board, src, is_white).len(), 6);

        src = [1 as usize, 8 as usize];
        is_white = true;
        assert_eq!( pieces::knight_moves(&board, src, is_white).len(), 2);
    }

    #[test]
    fn possible_queen_moves() {
        let board = Board::new(Position::TestQueen);
        let mut src = [5 as usize, 4 as usize];
        let mut is_white = false;
        assert_eq!( pieces::queen_moves(&board, src, is_white).len(), 1+3+2+2+4+3+3+3);

        src = [1 as usize, 8 as usize];
        is_white = true;
        assert_eq!( pieces::queen_moves(&board, src, is_white).len(), 3);
    }

    #[test]
    fn possible_rook_moves() {
        let board = Board::new(Position::TestRook);
        let mut src = [5 as usize, 4 as usize];
        let mut is_white = false;
        assert_eq!( pieces::rook_moves(&board, src, is_white).len(), 1+2+3+4);

        src = [1 as usize, 8 as usize];
        is_white = true;
        assert_eq!( pieces::rook_moves(&board, src, is_white).len(), 2);
    }
}