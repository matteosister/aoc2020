use thiserror::Error;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub mod utils;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
}

pub type AocResult<T> = Result<T, AocError>;
