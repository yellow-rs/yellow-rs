use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Pos {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Pos {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        Pos { start, end }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Error {
    message: String,
    position: Pos,
    error_type: ErrorType,
}

impl Error {
    pub fn new(message: String, error_type: ErrorType, position: Pos) -> Self {
        Error {
            message,
            error_type,
            position
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ErrorType {
    SyntaxError,
    LexError,
    TypeError,
    RuntimeError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorType::SyntaxError => "SyntaxError",
                ErrorType::LexError => "LexError",
                ErrorType::TypeError => "TypeError",
                ErrorType::RuntimeError => "RuntimeError"
            }
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: col {}-{}: {}",
            self.error_type, self.position.start, self.position.end, self.message
        )
    }
}
