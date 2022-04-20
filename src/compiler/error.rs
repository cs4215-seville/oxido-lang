use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use crate::parser::ast::SourceLocation;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub position: Option<SourceLocation>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.position {
            Some(position) =>
                write!(f, "Compilation error at {}:{} - {}", position.line, position.col, self.message),
            None => write!(f, "Compilation error - {}", self.message),
        }
    }
}