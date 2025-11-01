use crate::token::Token;
use std::fs;
use std::str::FromStr;

// Lexer error
// Position in a source file
#[derive(Debug, Clone)]
pub struct Position {
    pub file_name: String, // source file name
    pub line: usize,       // line number
    pub col: usize,        // column number
}

impl Position {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            line: 1,
            col: 1,
        }
    }
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub pos: Position,
}

// Format how a lex error is displayed
impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " Token error : {}\n in file {}\n at line {}\n col {}\n",
            self.message, self.pos.file_name, self.pos.line, self.pos.col
        )
    }
}

impl std::error::Error for LexError {}

impl From<std::io::Error> for LexError {
    fn from(error: std::io::Error) -> Self {
        LexError {
            message: format!("IO error: {}", error),
            pos: Position::new(String::new()),
        }
    }
}

pub struct Lexer {
    src_filename: String, // mpl source filename
    src_text: String,
    token_stream: Vec<Token>, // all tokens found in the source file
    pos: Position,
    i: usize, // current index in the source file
}

impl Lexer {
    pub fn new(src_filename: String) -> Self {
        let filename = src_filename.clone();
        Self {
            src_filename,
            token_stream: Vec::new(),
            src_text: String::new(),
            pos: Position::new(filename),
            i: 0,
        }
    }

    // get the next char in the source file
    fn get_next_char(&mut self) -> char {
        let c = self.src_text.chars().nth(self.i).unwrap_or('\0');
        self.pos.col += 1;
        self.i += 1;
        if c == '\n' {
            self.pos.line += 1;
            self.pos.col = 0;
        }
        c
    }

    // skip n next char in the source file
    fn bump(&mut self, nb: usize) {
        for _ in 0..nb {
            self.get_next_char();
        }
    }

    // get the next word in the source file
    fn get_next_word(&mut self) -> Option<String> {
        let mut word = String::new();
        loop {
            let c = self.get_next_char();
            if c == '\0' || c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                break;
            }
            word.push(c);
        }
        if word.is_empty() { None } else { Some(word) }
    }

    // identify the token
    fn identify_token(&mut self, word: &String) -> Option<Token> {
        match Token::from_str(&*word) {
            Ok(token) => Some(token),
            Err(..) => None,
        }
    }

    // skip whitespace
    fn skip_whitespace(&mut self) {
        let mut c = self.get_next_char();
        while c == ' ' || c == '\n' || c == '\r' || c == '\t' {
            c = self.get_next_char();
        }
        self.pos.col -= 1;
        self.i -= 1;
    }

    // look ahead nb chars
    fn look_ahead(&mut self, nb: usize) -> Option<String> {
        let end = self.i + nb;
        if end > self.src_text.len() {
            return None;
        }
        Some(self.src_text.chars().skip(self.i).take(nb).collect())
    }

    // skip comment single line
    fn skip_comment_single_line(&mut self) {
        if let Some(look_ahead) = self.look_ahead(2) {
            if look_ahead == "//" {
                let mut c = self.get_next_char();
                while c != '\n' {
                    c = self.get_next_char();
                }
            }
        }
    }

    fn skip_comment_multiple_line(&mut self) -> Result<(), LexError> {
        let mut close = true; // by default, the comment is closed (case of no comment)
        if let Some(look_ahead) = self.look_ahead(2) {
            // look ahead 2 chars
            if look_ahead == "/*" {
                close=false; // the comment is not closed yet
                // comment start
                self.bump(2); // skip /*
                loop {
                    // loop until the comment is closed
                    match self.look_ahead(2) {
                        // look ahead 2 chars
                        Some(look_ahead) => {
                            // get something
                            if look_ahead == "*/" {
                                // yes ! comment end
                                self.bump(2); // skip /*
                                self.skip_whitespace();
                                close = true; // comment is closed
                                break; // exit loop
                            } else {
                                // no, it was not the end of the comment
                                self.get_next_char(); // get next char
                            }
                        }
                        None => {
                            // end of the file reached
                            close = false; // comment is not closed
                            break;
                        }
                    }
                }
            }
        }
        if close {
            Ok(())
        } else {
            Err(LexError {
                message: "Unclosed comment".to_string(),
                pos: self.pos.clone(),
            })
        }
    }

    pub fn parse(&mut self) -> Result<&Vec<Token>, LexError> {
        self.src_text = fs::read_to_string(&self.src_filename)?;
        loop {
            self.skip_whitespace();
            self.skip_comment_single_line();
            self.skip_comment_multiple_line()?;
            let word = self.get_next_word();
            if let Some(word_str) = word {
                match self.identify_token(&word_str) {
                    Some(token) => self.token_stream.push(token),
                    None => {
                        return Err(LexError {
                            message: format!("Unknown token {}", word_str),
                            pos: self.pos.clone(),
                        });
                    }
                }
            } else {
                break; // No more words to process
            }
        }
        Ok(&self.token_stream)
    }
}
