use crate::board::Board;
use crate::board::Direction;
use crate::board::Word;
use crate::letter::Letter;

/**
Returns an iterator over the best moves to play, with the moves
getting progressively weaker.

Move verification is done lazily.
Move generation must be done beforehand so we can sort it by
the word score. Since generating the score is relatively cheap,
we can generate it even for the invalid moves, and prune them
out later when we iterate through them.
*/
pub fn best_moves<'a>(board: &'a Board, letters: &[Letter]) -> impl Iterator<Item = Word> + 'a {
    let mut rack = Vec::from(letters);

    let mut best: Vec<(u32, Word)> = Vec::new();
    for (location, letter) in board.enumerate_letters() {
        rack.push(letter);

        let words = get_createable_words(&rack);

        for word in words {
            let move_positions = get_move_positions(board, location, word);
            best.extend(move_positions.iter().map(|x| (x.get_score(board), (*x).clone())));
        }

        rack.pop();
    }

    best.sort_unstable_by_key(|x| x.0);
    best.into_iter().rev().filter(move |m| verify_move(&board, &m.1)).map(move |m| m.1)
}

/// TODO: Word extensions?
pub fn get_createable_words(rack: &[Letter]) -> impl Iterator<Item = &&'static str> + '_ {
    crate::WORD_LIST.lock().expect("Word list mutex is poisoned").expect("No word list set").iter().filter(|word| can_create_word(rack, word))
}

/**
Returns if you can create the word `word` using the letters in `rack` 
*/
pub fn can_create_word(rack: &[Letter], word: &str) -> bool {
    let mut rack = Vec::from(rack);
    let mut blank_count = rack.iter().filter(|&&x| x == Letter::Blank).count();

    'outer: for ch in word.chars() {
        for (i, letter) in rack.iter().enumerate() {
            if *letter == Letter::from_char(ch) {
                rack[i] = Letter::Blank;
                continue 'outer;
            }
        }
        if blank_count > 0 {
            blank_count -= 1;
            continue 'outer;
        }
        return false;
    }
    true
}

/**
**THIS DOES NOT GUARANTEE VALID MOVES.**

This function takes in a location (the letter to build off of), and the word, and returns
a list of [`Word`] structs (positions and orientations) that specify how a word can be positioned
and orientated around the starting letter.

# Example
```
use scrabby::{
    Board, Direction,
    computer
};

let mut board = Board::new(Board::DEFAULT_SS_BOARD_SIZE);
board.make_move(11, 11, "HELLO", Direction::Right);

// "EWE" can be orientated 4 different ways around the 'E' in "HELLO".
// 2 from the first 'E' and 2 from the second 'E'. 2 vertically and 2 horizontally.
let positions = computer::get_move_positions(&board, board.convert_to_index(11, 12), "EWE");
assert_eq!(positions.len(), 4);
```
*/
pub fn get_move_positions(board: &Board, location: usize, word: &'static str) -> Vec<Word> {
    // Consider using smallvec for performance?
    let mut good_ones = Vec::new();

    for direction in [Direction::Down, Direction::Right] {
        for letter in word.as_bytes().iter().enumerate().filter(|x| Letter::from_char(*x.1 as char) == board.get_index(location).unwrap()) {
            let string_position = letter.0;
            let starting_position = match direction {
                Direction::Down => location - (string_position * board.get_size()),
                Direction::Right => location - string_position
            };

            good_ones.push(Word::new(
                starting_position,
                direction,
                word
            ));
        }
    }

    good_ones
}

/**
Verify if a move is able to be played on the board.

# Checks
* Does a word go off the board
* Does a word replace letters in another word

**TODO:**
* Does a word make valid words in all the places it is parallel to a different word?
*/
pub fn verify_move(board: &Board, board_move: &Word) -> bool {
    let (starting_column, starting_row) = board.convert_from_index(board_move.location);

    for (i, word_letter) in board_move.word.as_bytes().iter().enumerate() {
        let test_position = match board_move.direction {
            Direction::Down => board_move.location.wrapping_add(i * board.get_size()),
            Direction::Right => board_move.location.wrapping_add(i)
        };

        let (current_column, current_row) = board.convert_from_index(test_position);
        if match board_move.direction {
            Direction::Down => current_column != starting_column,
            Direction::Right => current_row != starting_row
        } {
            return false;
        }

        if test_position >= board.get_size().pow(2) {
            return false;
        }
        
        let test = board.get_index(test_position);
        if let Some(test_inner) = test {
            if test_inner != Letter::from_char(*word_letter as char) {
                return false;
            }
        }
    }

    return true;
}