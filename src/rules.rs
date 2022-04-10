use crate::console_log;
use crate::utils::log;
use crate::pieces::{self, ChessMove};
use crate::board::Board;

/// Checks if the requested move is legal based on the current position
pub fn is_move_legal(board: &Board, requested_move: &ChessMove) -> bool {
    
    let possible_moves = possible_moves_from_square(board, requested_move.src);

    let mut is_legal = false;
    for possible_move in possible_moves {
        if requested_move.is_the_same_as(&possible_move) {
            is_legal = true;
            break;
        }
    }

    return is_legal;
}

/// all_possible_moves: Returns all the possible moves from the current
/// position.
pub fn all_possible_moves(board : &Board) -> Vec<ChessMove> {

    if board.is_checkmate() || board.is_draw() {
        // No possible moves if the game is over
        return vec![];
    }

    let occupied_squares = board.all_occupied_squares(board.white_to_move());

    let mut all_possible_moves : Vec<ChessMove> = vec![];
    for square in occupied_squares {
        all_possible_moves.append(&mut possible_moves_from_square(&board, square));
    }

    return all_possible_moves;
}

/// possible_moves_from_square: Given a chess board and a square, 
/// generates all possible chess moves for the piece on that square.
pub fn possible_moves_from_square(board: &Board, rank_file: [usize; 2]) -> Vec<ChessMove> {

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

    // Cull any moves that end in the king in check
    let mut moves_without_check : Vec<ChessMove> = vec![];
    for chess_move in moves {
        let mut board_after_move = board.clone();
        board_after_move.make_move(chess_move);
        board_after_move.set_is_white_to_move(board.white_to_move());
        if !board_after_move.is_check() {
            moves_without_check.push(chess_move);
        }
    }
    return moves_without_check;
} 

/// Tests to see that the rules are working
#[cfg(test)]
mod tests {
    use crate::pieces::ChessMove;
    use crate::board::Board;
    use crate::rules::{is_move_legal, all_possible_moves, possible_moves_from_square};

    #[test]
    fn remove_king_from_check() {
        let mut board = Board::new();
        board.set_board_from_fen_string("8/k7/8/3q1N2/8/8/2P1P3/3K4");
        
        assert_eq!(all_possible_moves(&board).len(), 3);
    }

    #[test]
    fn move_piece_pinned_to_king() {
        let mut board = Board::new();
        board.set_board_from_fen_string("rnbqk1nr/pppp1ppp/4p3/8/1b1P4/2N5/PPP1PPPP/R1BQKBNR");
        let illegal_move = ChessMove::new(&board, [3, 3], [5, 2]);
        let legal_move = ChessMove::new(&board, [1, 3], [2, 4]);

        assert!( !is_move_legal(&board, &illegal_move) );
        assert!( is_move_legal(&board, &legal_move) )
    }
}