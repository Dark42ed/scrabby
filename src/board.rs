#![allow(dead_code)]
use crate::letter::Letter;
use core::fmt;
use std::{borrow::Cow, ops::Add};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Board {
    inner: Vec<Option<Letter>>,
    moves: Vec<Word>,
    size: usize,
}

impl Board {
    pub const DEFAULT_SS_BOARD_SIZE: usize = 21;
    pub fn new(size: usize) -> Board {
        Board {
            inner: vec![None; size * size],
            moves: Vec::new(),
            size,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn moves(&self) -> &[Word] {
        &self.moves
    }

    pub fn iter_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.inner.iter().filter_map(|x| *x)
    }

    pub fn enumerate_letters(&self) -> impl Iterator<Item = (Position, Letter)> + '_ {
        self.inner.iter().enumerate().filter_map(|x| {
            x.1.map(|y| {
                (
                    Position {
                        board_size: self.size,
                        index: x.0,
                    },
                    y,
                )
            })
        })
    }

    pub fn make_move(&mut self, mut position: Position, word: &str, direction: Direction) {
        let start = position;
        for char in word.chars() {
            self.set(position, Some(Letter::from_char(char)));
            position = position.add_direction(direction, 1);
        }

        self.moves
            .push(Word::new(start, direction, Cow::Borrowed(word).to_owned()));
    }

    pub fn get(&self, position: Position) -> Option<Letter> {
        self.inner.get(position.index).cloned().flatten()
    }

    pub fn set(&mut self, position: Position, letter: Option<Letter>) {
        self.inner[position.index] = letter;
    }

    pub fn print(&self) {
        for i in 0..self.size {
            for l in self.inner[i * self.size..(i + 1) * self.size]
                .iter()
                .map(|x| match x {
                    Some(y) => y.to_char(),
                    None => '.',
                })
            {
                print!("{} ", l);
            }
            println!();
        }
    }

    #[cfg(feature = "pretty-print")]
    pub fn print_highlight(&self, highlight: &[(Letter, Position)]) {
        use colored::Colorize;
        for (i, l) in self.inner.iter().enumerate() {
            if i % self.size == 0 {
                println!();
            }
            if let Some(x) = highlight.iter().find(|f| f.1.index == i) {
                print!("{} ", x.0.to_char().to_string().red())
            } else if let Some(letter) = l {
                print!("{} ", letter.to_char());
            } else {
                print!(". ",);
            }
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Position {
    index: usize,
    board_size: usize,
}

impl Position {
    pub fn new(board_size: usize, row: usize, column: usize) -> Position {
        Position {
            board_size,
            index: row * board_size + column,
        }
    }

    pub fn from_index(&self, board_size: usize, index: usize) -> Position {
        Position { board_size, index }
    }

    pub fn as_row_column(&self) -> (usize, usize) {
        (self.index % self.board_size, self.index / self.board_size)
    }

    pub fn as_index(&self) -> usize {
        self.index
    }

    pub fn add_row(self, amount: isize) -> Position {
        Position {
            board_size: self.board_size,
            index: self
                .index
                .checked_add_signed(amount * self.board_size as isize)
                .unwrap(),
        }
    }

    pub fn add_column(self, amount: isize) -> Position {
        Position {
            board_size: self.board_size,
            index: self.index.checked_add_signed(amount).unwrap(),
        }
    }

    pub fn add_direction(self, direction: Direction, amount: isize) -> Position {
        Position {
            board_size: self.board_size,
            index: self
                .index
                .checked_add_signed(direction.offset(self.board_size) as isize * amount)
                .unwrap(),
        }
    }

    pub fn try_add_direction(self, direction: Direction, amount: isize) -> Option<Position> {
        let new_index = self
            .index
            .checked_add_signed(direction.offset(self.board_size) as isize * amount)
            .unwrap();
        if new_index > self.board_size.pow(2)
            || (direction == Direction::Right
                && self.index / self.board_size != new_index / self.board_size)
        {
            None
        } else {
            Some(Position {
                board_size: self.board_size,
                index: new_index,
            })
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (row, col) = self.as_row_column();
        write!(f, "({}, {})", row, col)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Word {
    pub position: Position,
    pub direction: Direction,
    pub word: String,
}

impl Word {
    pub fn new(position: Position, direction: Direction, word: Cow<'_, str>) -> Word {
        Word {
            position,
            direction,
            word: word.into_owned(),
        }
    }

    /**
    Gets the score of a word on the board.
    Accounts for letter and word multipliers

    **TODO:**
    * Word extensions
    * Account for blank letters not having any score
    */
    pub fn get_score(&self, board: &Board) -> u32 {
        let mut current_location = self.position;
        let mut sum = 0;
        let mut word_mul = 1;
        for l in self.word.chars() {
            let mut letter_mul = 1;
            if board.get(current_location).is_none() {
                letter_mul = crate::letter::LETTER_MULT
                    .get(current_location.index)
                    .cloned()
                    .unwrap_or(0) as u32;
            }
            sum += Letter::from_char(l).score() as u32 * letter_mul;

            if let Some(mul) = crate::letter::WORD_MULT.get(current_location.index) {
                if board.get(current_location).is_none() {
                    word_mul *= *mul as u32;
                }
            }

            current_location = current_location.add_direction(self.direction, 1);
        }
        sum *= word_mul;

        if self.word.len() == 8 {
            sum += 50;
        }

        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum Direction {
    Right,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Right,
        }
    }

    pub fn offset(&self, board_size: usize) -> usize {
        match self {
            Self::Right => 1,
            Self::Down => board_size,
        }
    }
}
