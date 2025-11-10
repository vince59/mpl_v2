use std::{fmt,error};

use crate::lexer::{LexError, LexToken, Lexer, Position, TokenStream};
use crate::token::Token;


pub struct Parser {
    tokens: Vec<LexToken>
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

// Format how a parsing error is displayed
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl error::Error for ParseError {}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new()
        }
    }

    fn parse_program(&mut self) -> Result<(), ParseError> {
        
        Ok(())
    }

    pub fn parse(&mut self, main_src_filename: String) -> Result<(), ParseError>{
        let mut lex = Lexer::new(main_src_filename);
        self.tokens=lex.tokenize()?;
        self.parse_program()?;
        Ok(())
    }
}
