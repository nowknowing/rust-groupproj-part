use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use crate::parser::ast::SourceLocation;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    // pub position: SourceLocation,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "To be implemented")
    }
}