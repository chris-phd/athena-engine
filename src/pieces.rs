use crate::console_log;
use crate::utils::{log, coord_to_rank_file};
use crate::board::Board;

/// DeltaRankFile. Defines one possible piece movement as a 
/// change in rank and file from the current square. 
pub struct DeltaRankFile {
    delta_rank: i32, 
    delta_file: i32,
}

impl DeltaRankFile {
    /// Returns an invalid square [0, 0] if destination square is off the board
    pub fn dest_from_src(&self, src: [usize; 2]) -> [usize; 2] {
        let dest = [(src[0] as i32 + self.delta_rank),
                   (src[1] as i32 + self.delta_file)];
        if dest[0] > 8 || dest[0] < 1 || dest[1] > 8 || dest[1] < 1 {
            return [0, 0];
        }
        
        return [dest[0] as usize, dest[1] as usize];
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveType {
    Standard,
    CastleKingSide,
    CastleQueenSide,
    EnPassant,
    PromoteToQueen,
    PromoteToRook,
    PromoteToBishop,
    PromoteToKnight,
    Invalid,
}

/// ChessMove, represents a move made by a player
#[derive(Clone, Copy)]
pub struct ChessMove {
    pub src: [usize; 2],
    pub dest: [usize; 2],
    pub piece: char,
    pub move_type: MoveType,
}

impl ChessMove {
    pub fn new(board: &Board, src: [usize; 2], dest: [usize; 2]) -> ChessMove {
        let mut new_move = ChessMove::new_empty_move();
        new_move.set_move(&board, src, dest);
        return new_move;
    }

    pub fn new_empty_move() -> ChessMove {
        return ChessMove {
            src: [0, 0],
            dest: [0, 0],
            piece: '-',
            move_type: MoveType::Standard,
        }
    }

    pub fn new_promotion(board: &Board, src: [usize; 2], dest: [usize; 2], promotion_enum: i32) -> ChessMove {
        let mut new_move = ChessMove::new(&board, src, dest);
        if promotion_enum == 1 {
            new_move.move_type = MoveType::PromoteToQueen;
        } else if promotion_enum == 2 {
            new_move.move_type = MoveType::PromoteToRook;
        } else if promotion_enum == 3 {
            new_move.move_type = MoveType::PromoteToBishop;
        } else if promotion_enum == 4 {
            new_move.move_type = MoveType::PromoteToKnight;
        } else {
            // SHould never call this method without a valid promotion.
            assert!(false);
        }
        return new_move; 
    }

    pub fn set_move(&mut self, board: &Board, src: [usize; 2], dest: [usize; 2]) {
        
        self.src = src;
        self.dest = dest;
        self.piece = board.get_piece_on_square(src);

        if (self.piece == 'k' || self.piece == 'K') && src[1] == 5 {
            if dest[1] == 7 {
                self.move_type = MoveType::CastleKingSide;
            } else if dest[1] == 3 {
                self.move_type = MoveType::CastleQueenSide;
            }
        }

        if (self.piece == 'p' || self.piece == 'P') && 
            dest[0] == board.get_en_passant_square()[0] && 
            dest[1] == board.get_en_passant_square()[1] {
            self.move_type = MoveType::EnPassant;
        }
    }

    pub fn set_move_from_pgn_notation(&mut self, board: &Board, pgn_notation: &String, white_to_move: bool) {
        
        if pgn_notation == "O-O" {
            self.move_type = MoveType::CastleKingSide;
            if white_to_move {
                self.piece = 'K';
                self.src = [1, 5];
                self.dest = [1, 7];
            } else {
                self.piece = 'k';
                self.src = [8, 5];
                self.dest = [8, 7];
            }
        } else if pgn_notation == "O-O-O" {
            self.move_type = MoveType::CastleQueenSide;
            if white_to_move {
                self.piece = 'K';
                self.src = [1, 5];
                self.dest = [1, 3];
            } else {
                self.piece = 'k';
                self.src = [8, 5];
                self.dest = [8, 3];
            }
        } else if (pgn_notation.as_bytes()[0] as char).is_ascii_lowercase() {
            // Pawn Moves

            // Ignore checks
            let mut mut_move_notation = pgn_notation.clone();
            if mut_move_notation.ends_with('+') {
                mut_move_notation.pop();
            }

            if (pgn_notation.as_bytes()[1] as char).is_ascii_digit() {
                // Normal pawn moves

                self.move_type = MoveType::Standard;
                self.dest = coord_to_rank_file(&pgn_notation[..2]);
                if white_to_move {
                    self.piece = 'P';
                    self.src = [self.dest[0] - 1, self.dest[1]];
                    if !board.is_occupied(self.src) {
                        self.src[0] -= 1;
                    }
                } else {
                    self.piece = 'p';
                    self.src = [self.dest[0] + 1, self.dest[1]];
                    if !board.is_occupied(self.src) {
                        self.src[0] += 1;
                    }
                }
            } else if (pgn_notation.as_bytes()[1] as char) == 'x' {
                // Pawn capture moves

                let n = mut_move_notation.len();
                let clarified_file = ( mut_move_notation.as_bytes()[0] - b'a' + 1) as usize;
                self.dest = coord_to_rank_file(String::from_utf8(mut_move_notation.as_bytes()[n-2..n].to_vec()).unwrap().as_str());
                
                if (self.dest[0] == board.get_en_passant_square()[0] &&
                    self.dest[1] == board.get_en_passant_square()[1]) {
    
                    self.move_type = MoveType::EnPassant;
                } else {
                    self.move_type = MoveType::Standard;
                }

                if white_to_move {
                    self.src = [self.dest[0] - 1, clarified_file];
                    self.piece = 'P';
                } else {
                    self.src = [self.dest[0] + 1, clarified_file];
                    self.piece = 'p';
                }

            } else {
                console_log!("  UNRECOGNISED PAWN MOVE = {:?}", pgn_notation);
                return;
            }
            
            if pgn_notation.contains('=') {
                // Promotions

                let promotion = mut_move_notation.pop().unwrap();
                if promotion == 'Q' {
                    self.move_type = MoveType::PromoteToQueen;
                } else if promotion == 'R' {
                    self.move_type = MoveType::PromoteToRook;
                } else if promotion == 'B' {
                    self.move_type = MoveType::PromoteToBishop;
                } else if promotion == 'N' {
                    self.move_type = MoveType::PromoteToKnight;
                }
            }

        } else if (pgn_notation.as_bytes()[0] as char).is_ascii_uppercase() {
            // Knights, Bishops, Rooks, Kings, Queens

            if white_to_move {
                self.piece = pgn_notation.as_bytes()[0] as char;
            } else {
                self.piece = (pgn_notation.as_bytes()[0] as char).to_ascii_lowercase();
            }

            let mut mut_move_notation = pgn_notation.clone();

            // Ignore checks
            if mut_move_notation.ends_with('+') {
                mut_move_notation.pop();
            }
            let mut n = mut_move_notation.len();
            self.dest = coord_to_rank_file(String::from_utf8(mut_move_notation.as_bytes()[n-2..n].to_vec()).unwrap().as_str());

            // Remove the destination coords and ignore capture mark
            mut_move_notation.pop();
            mut_move_notation.pop();
            if mut_move_notation.ends_with('x') {
                mut_move_notation.pop();
            }

            // Need to disambiguate white piece is being specified
            let mut clarified_rank : usize = 0;
            let mut clarified_file : usize = 0;
            n = mut_move_notation.len();
            if n == 2 && (mut_move_notation.as_bytes()[n-1] as char).is_ascii_digit() {
                // disambiguate by rank
                clarified_rank = ( mut_move_notation.as_bytes()[n-1] - b'0' ) as usize;
            } else if n == 2 && (mut_move_notation.as_bytes()[n-1] as char).is_ascii_lowercase() {
                // disambiguate by file
                clarified_file = ( mut_move_notation.as_bytes()[n-1] - b'a' + 1) as usize;
            } else if n == 3 {
                // disambiguate by rank and file
                clarified_rank = ( mut_move_notation.as_bytes()[n-1] - b'0' ) as usize;
                clarified_file = ( mut_move_notation.as_bytes()[n-2] - b'a' + 1) as usize;
            }

            let attacking_moves = pieces_attacking_square(&board, self.dest, white_to_move);
            for attacking_move in attacking_moves {
                if attacking_move.piece == self.piece && (
                    (clarified_file == 0 && clarified_rank == 0 ) ||
                    (clarified_file == 0 && clarified_rank == attacking_move.src[0]) ||
                    (clarified_rank == 0 && clarified_file == attacking_move.src[1]) ||
                    (clarified_file == attacking_move.src[0] && clarified_rank == attacking_move.src[1])) {
                    
                    self.src = attacking_move.src;
                    break;
                }
            }

            if self.src[0] != 0 && self.src[1] != 0 {
                // Otherwise src square was not found, move remains invalid
                self.move_type = MoveType::Standard;
            }

        } else {
            console_log!("  UNRECOGNISED PGN NOTATION = {:?}", pgn_notation);
        }

        console_log!("    MOVE: {:?}", pgn_notation);
        console_log!("      piece: {}", self.piece);
        console_log!("      src  : {:?}", self.src);
        console_log!("      dest : {:?}", self.dest);
        console_log!("      type : {:?}", self.move_type);
    }

    pub fn is_the_same_as(&self, that: &ChessMove) -> bool {

        let is_same_squares = self.src == that.src && 
                            self.dest == that.dest;
        let is_same_piece = self.piece == that.piece;
        let is_same_move_type = self.move_type == that.move_type;
        return is_same_piece && is_same_squares && is_same_move_type;
    }

    pub fn is_white_piece(&self) -> bool {
        return self.piece.is_uppercase();
    }
}

/// Returns all possible pawn moves from a given square
pub fn pawn_moves(board: &Board, rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let mut possible_moves_from_square = pawn_capture_moves(&board, rank_file, is_white);
    possible_moves_from_square.append(&mut pawn_non_capture_moves(&board, rank_file, is_white));
    return possible_moves_from_square;
}

/// Returns all possible pawn captures from a given square
fn pawn_capture_moves(board: &Board, src_rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let mut capture_moves : Vec<ChessMove> = vec![];

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
        let dest_rank_file = capture_movement.dest_from_src(src_rank_file);
        if board.is_valid_rank_file(dest_rank_file) &&
           (is_capture(&board, dest_rank_file, is_white) || 
            (dest_rank_file[0] == board.get_en_passant_square()[0] && 
            dest_rank_file[1] == board.get_en_passant_square()[1])) {

            // Is this move a promotion?
            if ( is_white && dest_rank_file[0] == 8 ) ||
               ( !is_white && dest_rank_file[0] == 1 ) {
 
                let mut promotions = vec![ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 1),
                                          ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 2),
                                          ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 3),
                                          ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 4)];
                capture_moves.append(&mut promotions);
            } else {
                let standard_move = ChessMove::new(&board, src_rank_file, dest_rank_file);
                capture_moves.push(standard_move);
            }
        }
    }

    return capture_moves;
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

        if !board.is_valid_rank_file(dest_rank_file) ||
           board.is_occupied(dest_rank_file) {
            continue;
        }
        if is_slide_clear_for_non_capture(&board, src_rank_file, dest_rank_file, is_white, false) {

            // Is this move a promotion?
            if ( is_white && dest_rank_file[0] == 8 ) ||
               ( !is_white && dest_rank_file[0] == 1 ) {

                let mut promotions = vec![ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 1),
                                          ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 2),
                                          ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 3),
                                          ChessMove::new_promotion(&board, src_rank_file, dest_rank_file, 4)];
                non_capture_moves.append(&mut promotions);
            } else {
                let standard_move = ChessMove::new(&board, src_rank_file, dest_rank_file);
                non_capture_moves.push(standard_move);
            }
        }
    }

    return non_capture_moves;
}

/// Returns all knight moves from a given square
pub fn knight_moves(board: &Board, src_rank_file: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let mut possible_moves_from_square : Vec<ChessMove> = vec![];

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
        if board.is_valid_rank_file(src_rank_file) && board.is_valid_rank_file(dest_rank_file) && 
           (!board.is_occupied(dest_rank_file) || is_capture(&board, dest_rank_file, is_white) ) {
            let possible_move = ChessMove::new(&board, src_rank_file, dest_rank_file);
            possible_moves_from_square.push(possible_move);
        }
    }

    return possible_moves_from_square;
}

/// Returns all the bishop moves from a given square
pub fn bishop_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
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
    let movements = vec![
        DeltaRankFile { delta_rank: -1, delta_file:  0, },
        DeltaRankFile { delta_rank:  0, delta_file:  1, },
        DeltaRankFile { delta_rank:  0, delta_file: -1, },
        DeltaRankFile { delta_rank:  1, delta_file:  0, },
    ];

    return slide_moves(movements, &board, src, is_white);
}


/// Generates all possible moves from a square by sliding in the given directions.
/// Used to generate the bishop and rook moves
fn slide_moves(movements: Vec<DeltaRankFile>, board: &Board, src: [usize; 2], is_white: bool)-> Vec<ChessMove> {
    let mut possible_moves_from_square : Vec<ChessMove> = vec![];

    for movement in movements {
        let mut dest = movement.dest_from_src(src);
        while board.is_valid_rank_file(src) && board.is_valid_rank_file(dest) && 
              (!board.is_occupied(dest) || is_capture(&board, dest, is_white) ) {

            let possible_move = ChessMove::new(&board, src, dest);
            possible_moves_from_square.push(possible_move);
            
            if is_capture(&board, dest, is_white) {
                break;
            }

            dest = movement.dest_from_src(dest);
        }
    }

    return possible_moves_from_square;
}


/// Returns all possible queen moves from a given square
pub fn queen_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let mut possible_moves_from_square = rook_moves(&board, src, is_white);
    possible_moves_from_square.append(&mut bishop_moves(&board, src, is_white));
    return possible_moves_from_square;
}

/// Returns all possible queen moves from a given square
pub fn king_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let move_into_check_allowed = false;
    let mut possible_moves_from_square = king_standard_moves(board, src, is_white, move_into_check_allowed);
    possible_moves_from_square.append(&mut king_castle_moves(board, src, is_white));
    return possible_moves_from_square;
}

/// Returns legal standard king moves from the current position
pub fn king_standard_moves(board: &Board, src: [usize; 2], is_white: bool, move_into_check_allowed: bool) -> Vec<ChessMove> {
    let mut standard_moves : Vec<ChessMove> = vec![];
    let movements = vec![
        DeltaRankFile { delta_rank: -1, delta_file:  0, },
        DeltaRankFile { delta_rank: -1, delta_file:  1, },
        DeltaRankFile { delta_rank:  0, delta_file:  1, },
        DeltaRankFile { delta_rank:  1, delta_file:  1, },
        DeltaRankFile { delta_rank:  1, delta_file:  0, },
        DeltaRankFile { delta_rank:  1, delta_file: -1, },
        DeltaRankFile { delta_rank:  0, delta_file: -1, },
        DeltaRankFile { delta_rank: -1, delta_file: -1, },
    ];

    if !board.is_valid_rank_file(src) {
        return standard_moves;
    }

    // Create a copy of the board without the king, otherwise the king's current position can
    // cover potential checks.
    let mut board_copy = board.clone();
    board_copy.clear_square(src);

    for movement in movements {
        let dest = movement.dest_from_src(src);
        if !board.is_valid_rank_file(dest) {
            continue;
        }

        if !move_into_check_allowed && is_square_attacked(&board_copy, dest, !is_white) {
            // King should not move into check
            continue;
        }
        
        if !board.is_occupied(dest) || is_capture_including_king_capture(&board_copy, dest, is_white) {
            let possible_move = ChessMove::new(&board, src, dest);
            standard_moves.push(possible_move);
        }
    }

    return standard_moves;
}

/// Returns legal castle moves from the current position
pub fn king_castle_moves(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let mut possible_castle_moves : Vec<ChessMove> = vec![];

    if is_square_attacked(&board, src, !is_white) {
        // no castle moves avaliable when king is in check
        return possible_castle_moves;        
    }

    // Check if the king 
    if board.is_castle_king_side_avaliable(is_white) { 
        let dest: [usize; 2];
        if is_white {
            dest = [1, 7];
        } else {
            dest = [8, 7];
        }
        
        let is_king = true;
        if is_slide_clear_for_non_capture(&board, src, dest, is_white, is_king) {
            let castle_move = ChessMove::new(&board, src, dest);
            possible_castle_moves.push(castle_move);
        }

    }
    
    if board.is_castle_queen_side_avaliable(is_white) {
        let dest: [usize; 2];
        if is_white {
            dest = [1, 3];
        } else {
            dest = [8, 3];
        }

        let is_king = true;
        if is_slide_clear_for_non_capture(&board, src, dest, is_white, is_king) {
            let castle_move = ChessMove::new(&board, src, dest);
            possible_castle_moves.push(castle_move);
        }
    }
    return possible_castle_moves;
}

/// Returns all possible king standard moves without checking if the king moves into check.
/// Used to ensure the kings will never be next to each other
fn king_moves_move_into_check_allowed(board: &Board, src: [usize; 2], is_white: bool) -> Vec<ChessMove> {
    let move_into_check_allowed = true;
    return king_standard_moves(&board, src, is_white, move_into_check_allowed);
}

/// Checks if a square is occupied by a piece of a different colour, and if it is 
/// occupied by the king. The king can never be captured.
fn is_capture(board : &Board, dest_rank_file: [usize; 2], is_white: bool) -> bool {
    return ((is_white && board.is_occupied_by_black(dest_rank_file)) ||
           (!is_white && board.is_occupied_by_white(dest_rank_file)) ) && 
           !board.is_occupied_by_king(dest_rank_file);
}

/// Performs the same checks as is_capture, but allows the king to be captured. Used to
/// check which squares are under attack in the is_square_attacked function
fn is_capture_including_king_capture(board : &Board, dest_rank_file: [usize; 2], is_white: bool) -> bool {
    return ( is_white && board.is_occupied_by_black(dest_rank_file)) ||
           (!is_white && board.is_occupied_by_white(dest_rank_file)); 
}

/// Checks if a slide move is clear of other pieces. Slide moves
/// handle movements for rooks, bishops, queens and king castles
fn is_slide_clear_for_non_capture(board: &Board, src: [usize; 2], dest: [usize; 2], 
    is_white: bool, is_king: bool) -> bool {
    let rank_dir = (dest[0] as i32 - src[0] as i32).signum();
    let file_dir = (dest[1] as i32 - src[1] as i32).signum();
    let mut traversed = [(src[0] as i32 + rank_dir) as usize,
                        (src[1] as i32 + file_dir) as usize];

    while traversed[0] != dest[0] || traversed[1] != dest[1] {

        if board.is_occupied(traversed) {
            return false;
        }

        // The king cannot enter check
        if is_king && is_square_attacked(&board, traversed, !is_white) {
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


    // Make sure the destination square doesn't put the king in check
    if is_king && is_square_attacked(&board, dest, !is_white) {
        return false;
    }


    return true;                           
}

/// Checks that the slide move is clear without checking the
/// destination square. 
fn is_slide_clear_for_capture(board: &Board, src: [usize; 2], dest: [usize; 2], is_white: bool) -> bool {
    if src[0] == dest[0] && src[1] == dest[1] {
        return true;
    }
    
    let rank_dir = (dest[0] as i32 - src[0] as i32).signum();
    let file_dir = (dest[1] as i32 - src[1] as i32).signum();
    let new_dest = [(dest[0] as i32 - rank_dir) as usize,
                              (dest[1] as i32 - file_dir) as usize];

    return is_slide_clear_for_non_capture(&board, src, new_dest, is_white, false);
}

/// Returns true if a square is attacked by a piece of a specified colour.
pub fn is_square_attacked(board : &Board, rank_file : [usize; 2], is_attacked_by_white : bool) -> bool {
    let num_attackers = pieces_attacking_square(&board, rank_file, is_attacked_by_white).len();
    return num_attackers > 0;
}

/// Returns the possible moves capable of attacking the target rank_file.
pub fn pieces_attacking_square(board : &Board, rank_file : [usize; 2], is_attacked_by_white : bool) -> Vec<ChessMove> {

    // Order of the pieces in the move_functions vector must match the order
    // in the piece types vector.
    // We do not need to check queen moves explicitly, because they will be checked
    // by the bishop and rook moves.
    let move_functions : Vec<&dyn Fn(&Board, [usize; 2], bool) -> Vec<ChessMove>> = 
        vec![&bishop_moves, &knight_moves, &rook_moves, &pawn_capture_moves, 
        &king_moves_move_into_check_allowed];
    let piece_types = vec!['B', 'N', 'R', 'P', 'K'];

    let mut attacking_squares : Vec<ChessMove> = vec![]; 
    for i in 0..piece_types.len() {

        let is_white = !is_attacked_by_white;
        let piece_moves = move_functions[i](&board, rank_file, is_white);
        for piece_move in piece_moves {
            let attacking_square = piece_move.dest;

            if !board.is_occupied(attacking_square) {
                continue;
            }

            let piece_on_attacking_square = board.get_piece_on_square(attacking_square);
            let piece_type = piece_on_attacking_square.to_ascii_uppercase();
            if piece_type == piece_types[i] ||
               (piece_type == 'Q' && (piece_types[i] == 'B' || piece_types[i] == 'R')) {
                let mut attacking_move = piece_move;
                attacking_move.dest = piece_move.src;
                attacking_move.src = piece_move.dest;
                attacking_move.piece = piece_on_attacking_square;
                attacking_squares.push(attacking_move);
            }
        }
    }

    return attacking_squares;
}

//
// Helper functions for parsing chess notation
//


#[cfg(test)]
mod tests {
    use crate::console_log;
    use crate::board::Board;
    use crate::pieces::{self};

    #[test]
    fn is_slide_clear() {

        console_log!("top of is slide clear");
        let mut board = Board::new();
        console_log!("created board");
        board.set_board_from_fen_string("8/3r4/8/3q1P2/8/8/6np/5k1Q ");
        console_log!("set board");
        let src = [5 as usize, 4 as usize];
        let mut dest = [8 as usize, 4 as usize];
        assert!( !pieces::is_slide_clear_for_non_capture(&board, src, dest, false, false) );

        dest = [5 as usize, 6 as usize];
        assert!( pieces::is_slide_clear_for_capture(&board, src, dest, false) );
    }

    #[test]
    fn possible_pawn_moves() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/4p2p/4K3/8/2n5/1P6/6P1/8 ");
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

        // Test promotions
        board.set_board_from_fen_string("7k/2P5/8/8/8/8/8/K7");
        src = [7 as usize, 3 as usize];
        is_white = true;
        assert_eq!( pieces::pawn_moves(&board, src, is_white).len(), 4 );

        board.set_board_from_fen_string("3q3k/2P5/8/8/8/8/8/K7");
        src = [7 as usize, 3 as usize];
        is_white = true;
        assert_eq!( pieces::pawn_moves(&board, src, is_white).len(), 8 );
    }

    #[test]
    fn possible_knight_moves() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/8/8/8/2N5/P7/1P1r2pp/7n");
        let mut src = [4 as usize, 3 as usize];
        let mut is_white = true;
        assert_eq!( pieces::knight_moves(&board, src, is_white).len(), 6);

        src = [1 as usize, 8 as usize];
        is_white = true;
        assert_eq!( pieces::knight_moves(&board, src, is_white).len(), 2);
    }

    #[test]
    fn possible_queen_moves() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/3r4/8/3q1P2/8/8/6np/5k1Q");
        let mut src = [5 as usize, 4 as usize];
        let mut is_white = false;
        assert_eq!( pieces::queen_moves(&board, src, is_white).len(), 1+3+2+2+4+3+3+3);

        src = [1 as usize, 8 as usize];
        is_white = true;
        assert_eq!( pieces::queen_moves(&board, src, is_white).len(), 3);
    }

    #[test]
    fn possible_rook_moves() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/3p4/8/3r1P2/8/8/6np/5Q1R");
        let mut src = [5 as usize, 4 as usize];
        let mut is_white = false;
        assert_eq!( pieces::rook_moves(&board, src, is_white).len(), 1+2+3+4);

        src = [1 as usize, 8 as usize];
        is_white = true;
        assert_eq!( pieces::rook_moves(&board, src, is_white).len(), 2);
    }

    #[test]
    fn is_square_attacked() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/3r4/8/3q1P2/8/8/6np/5k1Q");
        let mut attacked_square = [3 as usize, 2 as usize];
        let mut is_attacked_by_white = false;
        assert!( pieces::is_square_attacked(&board, attacked_square, is_attacked_by_white) );
        is_attacked_by_white = true;
        assert!( !pieces::is_square_attacked(&board, attacked_square, is_attacked_by_white) );

        attacked_square = [8 as usize, 8 as usize];
        is_attacked_by_white = true;
        assert!( !pieces::is_square_attacked(&board, attacked_square, is_attacked_by_white) );


        board.set_board_from_fen_string("8/4p2p/4K3/8/2n5/1P6/6P1/8");
        attacked_square = [6 as usize, 6 as usize];
        is_attacked_by_white = true;
        assert!( pieces::is_square_attacked(&board, attacked_square, is_attacked_by_white) );
        is_attacked_by_white = false;
        assert!( pieces::is_square_attacked(&board, attacked_square, is_attacked_by_white) );
    }

    #[test]
    fn possible_king_moves() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/8/8/8/8/8/3r1PPP/R3K2R");
        let src = [1 as usize, 5 as usize];
        let is_white = true;
        assert_eq!( pieces::king_moves(&board, src, is_white).len(), 3);

        board.set_board_from_fen_string("8/8/p7/P7/5k2/6q1/8/7K");
        board.render();
        let src = [1 as usize, 8 as usize];
        let is_white = true;
        assert_eq!( pieces::king_moves(&board, src, is_white).len(), 0);

        board.set_board_from_fen_string("2q2K2/8/8/1k6/8/8/8/8");
        let src = [8 as usize, 6 as usize];
        let is_white = true;
        assert_eq!( pieces::king_moves(&board, src, is_white).len(), 3);        
    }
}