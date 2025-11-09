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
/*
    pub fn get_import_list(&self) -> Result<Vec<String>, ParseError> {
        let mut imports = Vec::new();
        for t in self.token_stream.tokens.windows(2) {
            let (cur, next) = (&t[0], &t[1]);
            if cur.token == Token::Import {
                if let Token::Str(s) = &next.token {
                    if imports.contains(s) {
                        return Err(Lex(
                            LexError {
                                message: format!("import {} already defined", s),
                                pos: next.pos.clone(),
                            }
                        ));
                    } else {imports.push(s.clone())}
                } else {
                    return Err(ParseError::Unexpected {
                        found: next.token.clone(),
                        expected: "string (path to import)",
                        pos: next.pos.clone(),
                    });
                }
            }
        }
        Ok(imports)
    }
    */
}
