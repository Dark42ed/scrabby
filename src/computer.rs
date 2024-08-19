use std::borrow::Cow;

use crate::board::Board;
use crate::board::Direction;
use crate::board::Position;
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
pub fn best_moves<'a>(
    board: &'a Board,
    letters: &[Letter],
    word_list: &'a [&str],
) -> impl Iterator<Item = Word> + 'a {
    let mut rack = Vec::from(letters);

    let mut best: Vec<(u32, Word)> = Vec::new();
    for (location, letter) in board.enumerate_letters() {
        rack.push(letter);

        let words = word_list.iter().filter(|word| can_create_word(&rack, word));

        for word in words {
            let move_positions = get_move_positions(board, location, word);
            best.extend(
                move_positions
                    .iter()
                    .map(|x| (x.get_score(board), (*x).clone())),
            );
        }

        rack.pop();
    }

    best.sort_unstable_by_key(|x| x.0);
    best.into_iter()
        .rev()
        .filter(move |m| verify_move(&board, &m.1, word_list))
        .map(move |m| m.1)
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
    computer, Position
};

let mut board = Board::new(Board::DEFAULT_SS_BOARD_SIZE);
board.make_move(Position::new(board.size(), 11, 11), "HELLO", Direction::Right);

// "EWE" can be orientated 4 different ways around the 'E' in "HELLO".
// 2 from the first 'E' and 2 from the second 'E'. 2 vertically and 2 horizontally.
let positions = computer::get_move_positions(&board, Position::new(board.size(), 11, 12), "EWE");
assert_eq!(positions.len(), 4);
```
*/
pub fn get_move_positions(board: &Board, location: Position, word: &str) -> Vec<Word> {
    let mut good_ones = Vec::new();

    for direction in [Direction::Down, Direction::Right] {
        for letter in word
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|x| Letter::from_char(*x.1 as char) == board.get(location).unwrap())
        {
            let string_position = letter.0;
            let starting_position = location.add_direction(direction, -(string_position as isize));
            good_ones.push(Word::new(starting_position, direction, Cow::Borrowed(word)));
        }
    }

    good_ones
}

/**
Verify if a move is able to be played on the board.

# Checks
* Does a word go off the board
* Does a word replace letters in another word
* Does a word make valid words in all the places it is parallel to a different word

**TODO:**
* Verify move extensions
*/
pub fn verify_move(board: &Board, board_move: &Word, word_list: &[&str]) -> bool {
    // Verify the word is in bounds
    if board_move
        .position
        .try_add_direction(board_move.direction, board_move.word.len() as isize)
        .is_none()
    {
        return false;
    }

    for (i, word_letter) in board_move.word.as_bytes().iter().enumerate() {
        let test_position = board_move
            .position
            .add_direction(board_move.direction, i as isize);

        // Check if there is already a different letter
        let test = board.get(test_position);
        if let Some(test_inner) = test {
            if test_inner != Letter::from_char(*word_letter as char) {
                return false;
            }
        }

        // Check that all words formed are valid
        for direction in [Direction::Right, Direction::Down] {
            let new_word = find_boundary_word(
                board,
                test_position,
                *word_letter as char,
                board_move,
                direction,
            );

            if !new_word.is_empty()
                && !(word_list.contains(&&*new_word)
                    || board
                        .moves()
                        .iter()
                        .map(|mov| &mov.word)
                        .find(|word| **word == new_word)
                        .is_some())
            {
                return false;
            }
        }
    }

    return true;
}

fn find_boundary_word(
    board: &Board,
    start: Position,
    start_char: char,
    word: &Word,
    direction: Direction,
) -> String {
    let (start_bound, end_bound) = (
        find_bound(board, start, direction, true),
        find_bound(board, start, direction, false),
    );

    let mut new_word = String::new();
    let mut i = start_bound;
    if start_bound != end_bound {
        while i <= end_bound {
            // TODO: Fix for the entire word
            new_word.push(if i == start {
                start_char
            } else {
                board.get(i).unwrap().to_char()
            });
            i = i.add_direction(direction, 1);
        }
    }

    return new_word;
}

fn find_bound(
    board: &Board,
    start: Position,
    direction: Direction,
    direction_flip: bool,
) -> Position {
    let mut bound = start;
    loop {
        let next = bound.try_add_direction(direction, if direction_flip { -1 } else { 1 });
        match next {
            None => return bound,
            Some(next) if board.get(next).is_none() => return bound,
            Some(next) => bound = next,
        }
    }
}
