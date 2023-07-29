include!("chess.rs");
use std::path::Path;


// for now this main program only prints out the starting board
fn main() {
    let path = Path::new("src/positions/starting_pos.txt");
    let board : ChessBoard = init_board(path);
    print_board(&board);
}
