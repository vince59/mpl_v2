use crate::lexer::{LexError, Lexer, Position};
use crate::token::Token;

pub struct Parser {
    src_filename: String, // mpl source filename
}

#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    Unexpected {
        found: Token,
        expected: &'static str,
        pos: Position,
    }
}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        Self::Lex(e)
    }
}

// Format how a lex error is displayed
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lex(e) => write!(f, "{}", e),
            Self::Unexpected {
                found,
                expected,
                pos,
            } => write!(
                f,
                "Grammar error : Expected {}, found {:?} at {} line:col -> ({}:{})\n",
                expected, found, pos.file_name, pos.line, pos.col,
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(src_filename: String) -> Self {
        Self {
            src_filename,
        }
    }

    pub fn parse(&self) -> Result<(), ParseError>{
        let mut lex = Lexer::new(self.src_filename.clone());
        lex.tokenize()?;
        Ok(())
    }
}
