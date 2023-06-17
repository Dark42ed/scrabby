use scrabby::{
    Board, Direction, Letter, computer
};

#[test]
pub fn common_test() {
    scrabby::set_word_list(&scrabby::DEFAULT_WORD_LIST);
    // Create a board
    let mut board = Board::new(Board::DEFAULT_SS_BOARD_SIZE);
    
    // Make a move
    board.make_move(11, 11, "HELLO", Direction::Right);

    // Get the best moves with a given rack
    let best_moves = computer::best_moves(&board, &"AOEPDOI".chars().map(|c| Letter::from_char(c)).collect::<Vec<_>>());
    println!("There are {} moves we can make", best_moves.count());
}