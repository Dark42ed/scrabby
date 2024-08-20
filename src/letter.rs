lazy_static::lazy_static! {
    pub static ref WORD_MULT: &'static [u8] = Box::leak("
        4......3.....3......4
        .2......2...2......2.
        ..2......2.2......2..
        ...3......3......3...
        ....2...........2....
        .....2.........2.....
        ......2.......2......
        3......2.....2......3
        .2.................2.
        ..2...............2..
        ...3......2......3...
        ..2...............2..
        .2.................2.
        3......2.....2......3
        ......2.......2......
        .....2.........2.....
        ....2...........2....
        ...3......3......3...
        ..2......2.2......2..
        .2......2...2......2.
        4......3.....3......4
    ".as_bytes().iter().filter(|&&x| x != b'\n' && x != b'\r' && x != b' ').map(|&c| if c == b'.' {1} else {c - b'0'}).collect::<Vec<u8>>().into_boxed_slice());

    pub static ref LETTER_MULT: &'static [u8] = Box::leak("
        ...2......2......2...
        ....3...........3....
        .....4.........4.....
        2.....2.......2.....2
        .3......3...3......3.
        ..4......2.2......4..
        ...2......2......2...
        .....................
        ....3...3...3...3....
        .....2...2.2...2.....
        2.....2.......2.....2
        .....2...2.2...2.....
        ....3...3...3...3....
        .....................
        ...2......2......2...
        ..4......2.2......4..
        .3......3...3......3.
        2.....2.......2.....2
        .....4.........4.....
        ....3...........3....
        ...2......2......2...
    ".as_bytes().iter().filter(|&&x| x != b'\n' && x != b'\r' && x != b' ').map(|&c| if c == b'.' {1} else {c - b'0'}).collect::<Vec<u8>>().into_boxed_slice());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[allow(unused)]
#[repr(u8)]
pub enum Letter {
    A = b'A',
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Blank,
}

impl Letter {
    pub fn from_char(c: char) -> Letter {
        match c {
            ' ' => Self::Blank,
            _ => {
                assert!(c.is_ascii_uppercase());
                unsafe { core::mem::transmute(c as u8) }
            }
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Blank => ' ',
            _ => self as u8 as char,
        }
    }

    pub fn raw_score(self) -> u8 {
        match self {
            Letter::A => 1,
            Letter::B => 3,
            Letter::C => 3,
            Letter::D => 2,
            Letter::E => 1,
            Letter::F => 4,
            Letter::G => 2,
            Letter::H => 4,
            Letter::I => 1,
            Letter::J => 8,
            Letter::K => 5,
            Letter::L => 1,
            Letter::M => 3,
            Letter::N => 1,
            Letter::O => 1,
            Letter::P => 3,
            Letter::Q => 10,
            Letter::R => 1,
            Letter::S => 1,
            Letter::T => 1,
            Letter::U => 1,
            Letter::V => 4,
            Letter::W => 4,
            Letter::X => 8,
            Letter::Y => 4,
            Letter::Z => 10,
            Letter::Blank => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn letter_from_char() {
        assert_eq!(Letter::from_char('J'), Letter::J);
    }

    #[test]
    pub fn char_from_letter() {
        assert_eq!(Letter::H.to_char(), 'H');
    }
}
