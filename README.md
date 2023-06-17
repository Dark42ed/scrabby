# Scrabby

The Super Scrabble Engine

---

> **NOTE: The Super Scrabble board is different from normal Scrabble, so the best move on a Super Scrabble board may be different than on a normal Scrabble board.**

> **NOTE: Word placements and point calculations may not be fully correct yet, so always double check.**

## Usage

```bash
cargo add --git https://github.com/Dark42ed/scrabby.git
```

## Getting started

```rust
use scrabby::{
    Board, Direction, Letter, computer
};

pub fn main() {
    // Create a board
    let mut board = Board::new(Board::DEFAULT_SS_BOARD_SIZE);
    
    // Make a move
    board.make_move(11, 11, "HELLO", Direction::Right);

    // Get the best moves with a given rack
    let best_moves = computer::best_moves(&board, &"AOEPDOI".chars().map(|c| Letter::from_char(c)).collect::<Vec<_>>());
    println!("There are {} moves we can make", best_moves.count());
}
```