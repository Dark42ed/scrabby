#![allow(dead_code)]
use crate::letter::Letter;
use colored::Colorize;
use std::borrow::Cow;

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

    pub fn enumerate_letters(&self) -> impl Iterator<Item = (usize, Letter)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|x| x.1.map(|y| (x.0, y)))
    }

    pub fn make_move(
        &mut self,
        mut row: usize,
        mut column: usize,
        word: &'static str,
        direction: Direction,
    ) {
        for char in word.chars() {
            self.set(row, column, Some(Letter::from_char(char)));
            match direction {
                Direction::Down => row += 1,
                Direction::Right => column += 1,
            };
        }

        self.moves.push(Word::new(
            self.convert_to_index(row, column),
            direction,
            Cow::Borrowed(word),
        ));
    }

    pub fn get(&self, row: usize, column: usize) -> Option<Option<Letter>> {
        self.inner.get(self.convert_to_index(row, column)).copied()
    }

    pub fn set(&mut self, row: usize, column: usize, letter: Option<Letter>) {
        let index = self.convert_to_index(row, column);
        self.inner[index] = letter;
    }

    pub fn get_index(&self, index: usize) -> Option<Letter> {
        self.inner[index]
    }

    pub fn set_index(&mut self, index: usize, letter: Option<Letter>) {
        self.inner[index] = letter;
    }

    pub fn convert_to_index(&self, row: usize, column: usize) -> usize {
        row * self.size + column
    }

    pub fn convert_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
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

    pub fn print_highlight(&self, highlight: &[(Letter, usize)]) {
        for (i, l) in self.inner.iter().enumerate() {
            if i % self.size == 0 {
                println!();
            }
            if let Some(x) = highlight.iter().find(|f| f.1 == i) {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Word {
    pub location: usize,
    pub direction: Direction,
    pub word: String,
}

impl Word {
    pub fn new(location: usize, direction: Direction, word: Cow<'_, str>) -> Word {
        Word {
            location,
            direction,
            word: word.into_owned(),
        }
    }

    /**
    Gets the score of a word on the board.
    Accounts for letter and word multipliers

    **TODO:**
    * Account for blank letters not having any score
    * Word extensions
    */
    pub fn get_score(&self, board: &Board) -> u32 {
        let location_change = match self.direction {
            Direction::Right => 1,
            Direction::Down => board.size(),
        };

        let mut current_location = self.location;
        let mut sum = 0;
        let mut word_mul = 1;
        for l in self.word.chars() {
            let mut letter_mul = 1;
            if board.get_index(current_location).is_none() {
                letter_mul = crate::letter::LETTER_MULT[current_location] as u32;
            }
            sum += Letter::from_char(l).score() as u32 * letter_mul;

            if let Some(mul) = crate::letter::WORD_MULT.get(current_location) {
                if board.get_index(current_location).is_none() {
                    word_mul *= *mul as u32;
                }
            }

            current_location += location_change;
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
