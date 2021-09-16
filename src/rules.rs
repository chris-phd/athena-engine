use crate::console_log;
use crate::utils::log;
use crate::pieces::{self, ChessMove};
use crate::board::Board;

/// Checks if the requested move is legal based on the current position
pub fn is_move_legal(board: &Board, requested_move: &ChessMove) -> bool {
    console_log!("rules::is_move_legal: ");
    
    let possible_moves = all_possible_moves(board, requested_move.src);

    let mut is_legal = false;
    for possible_move in possible_moves {
        if requested_move.is_the_same_as(&possible_move) {
            is_legal = true;
            break;
        }
    }

    if is_legal {
        console_log!("    move IS legal");
    } else {
        console_log!("    move is NOT legal");
    }

    return is_legal;
}

/// Checks if the current position is checkmate
pub fn is_checkmate(board: &Board) -> bool {

    // Check if the king is in check
    let king_rank_file = board.get_king_rank_file();
    console_log!("king rank and file = {:?}", king_rank_file);
    let is_white = board.white_to_move();
    if !pieces::is_square_attacked(&board, king_rank_file, !is_white) {
        return false;
    }

    // Check if the king can move any where. need to modify the board for this
    let possible_moves = pieces::king_standard_moves(&board, king_rank_file, is_white, false);
    let mut board_copy = board.clone();
    board_copy.clear_square(king_rank_file);
    for possible_move in possible_moves {
        let dest = possible_move.dest;
        if !pieces::is_square_attacked(&board_copy, dest, !is_white) {            
            console_log!("king can move to safety on {:?}", dest);
            return false;
        }
    }

    // Check if the attacking piece can be captured, or if the attack can be
    // intercepted (by moving a friendly piece in front of the attack)
    let attacking_moves = pieces::pieces_attacking_square(&board, king_rank_file, !is_white);
    console_log!("num attacking moves = {}", attacking_moves.len());
    for attacking_move in attacking_moves {
        if pieces::is_square_attacked(&board, attacking_move.src, is_white) {
            console_log!("attacking piece can be captured");
            return false;
        }

        if can_attack_be_intercepted(&board, attacking_move) {
            console_log!("attack can be intercepted");
            return false;
        }
    }

    return true;
}

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

        if pieces::is_square_attacked(&board_copy, traversed, !is_white) {
            // The sliding attack can be intercepted.
            console_log!("traversed = {:?}", traversed);
            return true;
        }

        traversed[0] = (traversed[0] as i32 + rank_dir) as usize;
        traversed[1] = (traversed[1] as i32 + file_dir) as usize;
    }

    return false;
}

/// all_possible_moves: Given a chess board and a square, 
/// generates all possible chess moves for the piece on that square.
pub fn all_possible_moves(board: &Board, rank_file: [usize; 2]) -> Vec<ChessMove> {

    let piece = board.get_piece_on_square(rank_file);
    let is_white = piece.is_uppercase();
    let piece_type = piece.to_ascii_uppercase();

    let moves: Vec<ChessMove>;
    match piece_type {
        'K' => moves = pieces::king_moves(&board, rank_file, is_white),
        'Q' => moves = pieces::queen_moves(&board, rank_file, is_white),
        'R' => moves = pieces::rook_moves(&board, rank_file, is_white),
        'B' => moves = pieces::bishop_moves(&board, rank_file, is_white),
        'N' => moves = pieces::knight_moves(&board, rank_file, is_white),
        'P' => moves = pieces::pawn_moves(&board, rank_file, is_white),
        _ => panic!(),
    }

    // return moves;
    return moves;
} 

/// Tests to see that the rules are working
#[cfg(test)]
mod tests {
    use crate::pieces::ChessMove;
    use crate::board::Board;
    use crate::rules::{is_move_legal, is_checkmate};


    #[test]
    fn king_legal_capture() {

        let mut board = Board::new();
        board.set_board_from_fen_string("8/8/8/8/8/8/3r1PPP/R3K2R");
        board.render();
        let mut requested_move = ChessMove::new_empty_move();

        eprintln!("Test a regular capture");
        requested_move.set_move(&board, [1 as usize, 5 as usize], [2 as usize, 4 as usize]);
        assert!( is_move_legal(&board, &requested_move) );

    }

    #[test]
    fn is_checkmate_test() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/6k1/8/8/8/8/5PPP/2r3K1");
        board.render();
        assert!( is_checkmate(&board) );

        board.set_board_from_fen_string("8/6k1/8/8/8/7P/5PP1/2r3K1");
        assert!( !is_checkmate(&board) );

        board.set_board_from_fen_string("8/6k1/8/8/8/4N3/5PPP/2r3K1");
        board.render();
        assert!( !is_checkmate(&board) );

        board.set_board_from_fen_string("8/6k1/8/8/5B2/8/5PPP/2r3K1");
        assert!( !is_checkmate(&board) );

        board.set_board_from_fen_string("5rkb/5pnn/7N/8/8/4K3/8/8");
        board.set_is_white_to_move(false);
        assert!( is_checkmate(&board));
        
        board.set_board_from_fen_string("5rkb/5ppn/7N/8/8/4K3/8/8");
        board.set_is_white_to_move(false);
        assert!( !is_checkmate(&board));
    }
}