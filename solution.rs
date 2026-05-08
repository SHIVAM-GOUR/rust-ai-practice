use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "input is empty"),
            ParseError::InvalidNumber(name) => write!(f, "invalid input: {}", name),
        }
    }
}

impl Error for ParseError {}

fn main() {
    println!("{:?}", parse_positive(""));
    println!("{:?}", parse_positive("abc"));
    println!("{:?}", parse_positive("42"));
}

fn parse_positive(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() {
        Err(ParseError::EmptyInput)?;
        // return Err(ParseError::EmptyInput);
        // Err::<u32, ParseError>(ParseError::EmptyInput)?;
    }

    let val = s
        .parse()
        .map_err(|e: std::num::ParseIntError| ParseError::InvalidNumber(e.to_string()))?;
    Ok(val)
}