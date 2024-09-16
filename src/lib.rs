pub mod board;
pub mod computer;
pub mod letter;

pub use board::{Board, Direction, Position, Word};
pub use letter::Letter;

pub const DEFAULT_WORD_LIST: &[&str] = &include!(concat!(env!("OUT_DIR"), "/words.rs"));
