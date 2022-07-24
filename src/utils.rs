use std::{error::Error, fmt::Display};

use lazy_static::lazy_static;
use regex::Regex;

#[macro_export]
macro_rules! result {
    (C) => {
        Correctness::Correct
    };
    (M) => {
        Correctness::Misplaced
    };
    (W) => {
        Correctness::Wrong
    };
    ($($item: tt)*) => {
        [
        $(result!($item)),*
        ]
    };
}

lazy_static! {
    static ref REGEX: Regex = Regex::new("^[a-z]+$").unwrap();
}

#[derive(Debug)]
pub enum ValidationError {
    TooShort,
    TooLong,
    InvalidCharacters,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TooShort => {
                write!(f, "Input too short. Please supply a 5 letter word")
            }
            ValidationError::TooLong => write!(f, "Input too long. Please supply a 5 letter word"),
            ValidationError::InvalidCharacters => {
                write!(f, "Input contains invalid characters. Please only use a-z")
            }
        }
    }
}

impl Error for ValidationError {}

pub fn validate(input: &str) -> Result<(), ValidationError> {
    if input.len() < 5 {
        return Err(ValidationError::TooShort);
    }

    if input.len() > 5 {
        return Err(ValidationError::TooLong);
    }

    if !REGEX.is_match(&input) {
        return Err(ValidationError::InvalidCharacters);
    }

    Ok(())
}
