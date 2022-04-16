// So I think I can profile the main funtion with 
// cargo flamegraph
// Just need to figure out how to import the library functions
// into the main.rs.. 

fn main() {
    eprintln!("athena-engine-main: profiled");

    let board = athena_engine::board::Board::new();
    board.render();

    let possible_moves = athena_engine::rules::all_possible_moves(&board);
    let num_possible_moves = possible_moves.len();

    eprintln!("    num possible moves = {}", num_possible_moves);
}
