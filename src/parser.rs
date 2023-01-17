
use crate::domain;
use nom;
use nom::{
    branch::alt,
    character::complete,
    IResult, Parser
};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ParseError;

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("already destroyed", f)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessError").finish()
    }
}

impl Error for ParseError {}

pub fn parse_base(s: &str) -> IResult<&str, char> {
    alt((
        complete::char('A'),
        complete::char('C'),
        complete::char('G'),
        complete::char('T')
    ))
    .parse(s)
}

pub fn parse_base_bin(bytes: Vec<u8>) -> Result<Vec<domain::Base>, ParseError> {
    let mut remainder = bytes;
    let mut bases:Vec<domain::Base> = Vec::new();
    loop {
        match remainder.pop() {
            None => break,
            Some(b) => {

            },
        }
    }
    Ok(bases)
}

pub fn write_to_bytes(dna: Vec<domain::Base>) -> Vec<i8> {
    Vec::new()
}