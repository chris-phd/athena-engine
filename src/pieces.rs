use crate::console_log;
use crate::utils::log;
use crate::board::Board;

///
pub struct Piece {
    is_white: bool,
}

pub struct King {
    is_white: bool,
}

pub struct Pawn {
    is_white: bool,
}

/// DeltaRankFile. Defines one possible piece movement as a 
/// change in rank and file from the current square. 
pub struct DeltaRankFile {
    delta_rank: i32, 
    delta_file: i32,
}

impl DeltaRankFile {
    pub fn dest_from_src(&self, src_rank_file: [usize; 2]) -> [usize; 2] {
        return [(src_rank_file[0] as i32 + self.delta_rank) as usize,
                (src_rank_file[1] as i32 + self.delta_file) as usize];
    }
}

/// Movement. Defines how each of the pieces move. Common for all pieces
pub trait Movement {
    /// Returns a piece's moveset for most considitions
    fn standard_moves(&self) -> Vec<DeltaRankFile>;

    /// True for pawns and kings. Deals with pawn captures and king castle movements.
    fn has_special_moves(&self) -> bool;

    /// True for bishop, rook and queen
    fn is_sliding_piece(&self) -> bool;
}

impl Movement for King {
    fn standard_moves(&self) -> Vec<DeltaRankFile> {
        let standard_moves = vec![
            DeltaRankFile { delta_rank: -1, delta_file:  1},
            DeltaRankFile { delta_rank: -1, delta_file:  0},
            DeltaRankFile { delta_rank: -1, delta_file: -1},
            DeltaRankFile { delta_rank:  0, delta_file: -1},
            DeltaRankFile { delta_rank:  0, delta_file:  1},
            DeltaRankFile { delta_rank:  1, delta_file: -1},
            DeltaRankFile { delta_rank:  1, delta_file:  0},
            DeltaRankFile { delta_rank:  1, delta_file:  1},
        ];
        return standard_moves;
    }

    fn has_special_moves(&self) -> bool {
        // King castle moves are not in the standard moveset
        return true;
    }

    fn is_sliding_piece(&self) -> bool {
        return false;
    }
}

impl Movement for Pawn {
    fn standard_moves(&self) -> Vec<DeltaRankFile> {
        let standard_moves;
        if self.is_white {
            standard_moves = vec![
                DeltaRankFile {delta_rank:  1, delta_file:  0},
                DeltaRankFile {delta_rank:  2, delta_file:  0},
            ];
        } else {
            standard_moves = vec![
                DeltaRankFile {delta_rank:  -1, delta_file:  0},
                DeltaRankFile {delta_rank:  -2, delta_file:  0},
            ];
        }
    
        return standard_moves;
    }

    fn has_special_moves(&self) -> bool {
        // Pawn captures are not in the standard moveset
        return true;
    }

    fn is_sliding_piece(&self) -> bool {
        // Since a pawn can move two squares on the first move.
        return true;
    }
}

// Implementation specific to each piece
impl King {
    fn castle_moves(&self) -> Vec<DeltaRankFile> {
        let castle_movements = vec![
            DeltaRankFile { delta_rank:  0, delta_file: -3},
            DeltaRankFile { delta_rank:  0, delta_file: -2},
            DeltaRankFile { delta_rank:  0, delta_file:  2},
            DeltaRankFile { delta_rank:  0, delta_file:  3},
        ];
        return castle_movements;
    }
}

impl Pawn {
    fn capture_moves(&self) -> Vec<DeltaRankFile> {
        let capture_movements;
        if self.is_white {
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
        return capture_movements;
    }
}

/// ChessMove, represents a move made by a player
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

/// Returns all possible pawn moves from a given square
pub fn pawn_moves(board: &Board, rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
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
        if is_capture(&board, is_white, dest_rank_file) {
            let mut possible_move = ChessMove::new();
            possible_move.set_move(&board, rank_file, dest_rank_file);
            non_capture_moves.push(possible_move);
        }
    }

    return non_capture_moves;
}

/// Returns all possible non capture pawn moves from a given square. 
fn pawn_non_capture_moves(board: &Board, rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
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
        let dest_rank_file = non_capture_movement.dest_from_src(rank_file);
        if board.is_occupied(dest_rank_file) {
            continue;
        }
        if is_slide_clear_for_non_capture(&board, rank_file, dest_rank_file) {
            let mut possible_move = ChessMove::new();
            possible_move.set_move(&board, rank_file, dest_rank_file);
            non_capture_moves.push(possible_move);
        }
    }

    return non_capture_moves;
}

/// Checks if a square is occupied by a piece of a different colour.
fn is_capture(board : &Board, is_white: bool, dest_rank_file: [usize; 2]) -> bool {
    return (is_white && board.is_occupied_by_black(dest_rank_file)) ||
           (!is_white && board.is_occupied_by_white(dest_rank_file));
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
        if traversed[0] > 8 || traversed[0] < 1 || traversed[1] > 8 || traversed[1] < 1 {
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

/// Tests to see that the rules are working
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
    }
}