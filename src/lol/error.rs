use crate::lol::span::Span;

use std::fmt;

#[derive(Debug)]
pub enum LolError {

    Lexer {

        message: String,

        span: Span,

    },

    Parser {

        message: String,

        span: Span,

    },

    Validation {

        message: String,

    },

    Io(std::io::Error),

}

pub type Result<T> = std::result::Result<T, LolError>;

impl fmt::Display for LolError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {

            Self::Lexer { message, span } => {
                write!(f, "Lexer Error [{}]: {}", span, message)
            }

            Self::Parser { message, span } => {
                write!(f, "Parser Error [{}]: {}", span, message)
            }

            Self::Validation { message } => {
                write!(f, "Validation Error: {}", message)
            }

            Self::Io(err) => {
                write!(f, "IO Error: {}", err)
            }

        }

    }

}

impl std::error::Error for LolError {}

impl From<std::io::Error> for LolError {

    fn from(err: std::io::Error) -> Self {

        Self::Io(err)

    }

}
