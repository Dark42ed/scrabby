pub mod board;
pub mod computer;
pub mod letter;

pub use board::{Board, Direction, Word};
pub use letter::Letter;

const DEFAULT_WORD_TEXT: &'static str = include_str!("../words.txt");
lazy_static::lazy_static! {
    pub static ref DEFAULT_WORD_LIST: Vec<&'static str> = {
        DEFAULT_WORD_TEXT.lines().collect::<Vec<_>>()
    };
}
