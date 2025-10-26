use std::fmt::{Display, Formatter};
use std::io::{self, Write};

pub fn read_input_string() -> Result<String, GameError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| GameError::IoError)?;
    Ok(input.trim().to_string())
}

pub fn read_string_to_int() -> Result<i32, GameError> {
    Ok(read_input_string()?.trim().parse().map_err(|_| GameError::ParseError)?)
}

pub fn flush_stdout() {
    io::stdout().flush().unwrap();
}

#[derive(Debug)]
pub enum GameError {
    IoError,
    ParseError,
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            GameError::IoError => write!(f, "读取输入错误"),
            GameError::ParseError => write!(f, "请输入正整数"),
        }
    }
}