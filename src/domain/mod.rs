use rand::Rng;
use std::{fmt::{self}};

pub enum Base {
    A,
    T,
    G,
    C
}

impl Base {
    pub fn from_char(c: char) ->  Result<Base, String> {
        match c {
            'a' | 'A' => Ok(Base::A),
            'c' | 'C' => Ok(Base::C),
            'g' | 'G' => Ok(Base::G),
            't' | 'T' => Ok(Base::T),
            _ => Err("oops".to_string()),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Base::A => 'A',
            Base::C => 'C',
            Base::G => 'G',
            Base::T => 'T',
        }
    }

    pub fn complement(&self) -> Base {
        match self {
            Base::A => Base::T,
            Base::C => Base::G,
            Base::G => Base::C,
            Base::T => Base::A,
        }
    }
}

pub fn random_base() -> Base {
    let random_n = rand::thread_rng().gen_range(0..4);
    match random_n {
        0 => Base::A,
        1 => Base::C,
        2 => Base::G,
        3 => Base::T,
        _ => panic!("Invalid Base")
    }
} 

// pub fn base_from_char(c: char) ->  Result<Base, String> {
//     match c {
//         'a' => Ok(Base::A),
//         'c' => Ok(Base::C),
//         'g' => Ok(Base::G),
//         't' => Ok(Base::T),
//         _ => Err("oops".to_string()),
//     }
// }

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}