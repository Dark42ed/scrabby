pub mod board;
pub mod computer;
pub mod letter;

pub use board::{
    Board,
    Direction,
    Word
};
pub use letter::Letter;

use std::sync::Mutex;

const DEFAULT_WORD_TEXT: &'static str = include_str!("../words.txt");
lazy_static::lazy_static! {
    pub static ref DEFAULT_WORD_LIST: Vec<&'static str> = {
        DEFAULT_WORD_TEXT.lines().collect::<Vec<_>>()
    };
}

static WORD_LIST: Mutex<Option<&'static [&'static str]>> = Mutex::new(None);

pub fn set_word_list(list: &'static [&'static str]) {
    *WORD_LIST.lock().expect(
        "Word list mutex is poisoned"
    ) = Some(list);
}