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

    //save the state of the lexer
    fn save_state(&self) -> (usize, usize, usize) {
        (self.i, self.pos.col, self.pos.line)
    }

    // restore the state of the lexer
    fn restore_state(&mut self, (i, col, line): (usize, usize, usize)) {
        self.i = i;
        self.pos.col = col;
        self.pos.line = line;
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

    // get the next word in the source file, but only if it is of the given length (do not consume the word)
    fn try_next_word_len(&mut self, nb : usize) -> Option<String> {
        let (i_tmp, col_tmp, line_tmp) = self.save_state();
        let mut word = String::new();
        for _ in 0..nb {
            let c = self.get_next_char();
            if c == '\0' || c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                break;
            }
            word.push(c);
        }
        self.restore_state( (i_tmp, col_tmp, line_tmp) );
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

    // check if a char is a digit or a dot
    #[inline]
    fn is_digit(ch: char) -> bool {
        if ch == '.' {true} else  {ch.is_ascii_digit()}
    }

    fn try_number(&mut self) -> Option<String> {
        let mut word = String::new();
        let symbol = self.try_next_word_len(1);
        if let Some(symbol) = symbol {
            if let Some(c) = symbol.chars().next() {
                if Self::is_digit(c) {
                    loop {
                        let c = self.get_next_char();
                        if Self::is_digit(c) {
                            word.push(c);
                        } else {
                            return Some(word);
                        }
                    }
                }
                else {
                    return None;
                }
            }
        } else {
            return None;
        }
        None
    }

    fn try_string(&mut self) -> Result<Option<String>, LexError> {
        let symbol = self.try_next_word_len(1);
        if let Some(symbol) = symbol {
            if symbol == "\"" {
                self.bump(1);
                let mut str = String::new();
                loop {
                    let c = self.get_next_char();
                    match c {   
                        '\0' => {
                            return Err(LexError {
                                message: "Unclosed string".to_string(),
                                pos: self.pos.clone(),
                            });
                        },
                        '"' => {
                            self.bump(1);
                            break;
                        },
                        _ => {
                            str.push(c);
                        }   
                    }
                }
                Ok(Some(str))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    // check if the end of the file is reached
    #[inline]
    fn eof(&self) -> bool {
        self.i >= self.src_text.len()
    }

    // check if the word is a valid identifier, must start with a letter
    #[inline]
    fn is_ident_valid(&self, word: &String) -> bool {
        let mut valid = true;
        match word.chars().next() {
            Some(c) => {
                if !c.is_ascii_alphabetic()  {
                    valid = false;
                }
            },
            None => {
                valid = false;
            }
        }
        for c in word.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                valid = false;
                break;
            }
        }
        valid
    }

    pub fn parse(&mut self) -> Result<&Vec<Token>, LexError> {
        self.src_text = fs::read_to_string(&self.src_filename)?;
        loop {
            self.skip_whitespace();
            self.skip_comment_single_line();
            self.skip_comment_multiple_line()?;
            // end of file
            if self.eof() {
                self.token_stream.push(Token::Eof);
                break;
            }
            // identify string
            if let Some(str) = self.try_string()? {
                self.token_stream.push(Token::Str(str));
                continue;
            }
            // identify symbols
            let symbol = self.try_next_word_len(1);
            if let Some(word_str) = symbol {
                match self.identify_token(&word_str) {
                    Some(token) => {self.token_stream.push(token); self.bump(1); continue},
                    _ => {}
                }
            }
            // identify number
            if let Some(word_str) = self.try_number() {
                if word_str.contains('.') {
                    self.token_stream.push(Token::Float(word_str.parse::<f64>().map_err(|_| LexError {
                        message: format!("invalid float number format [{}]", word_str),
                        pos: self.pos.clone(),
                    })?));
                } else {
                    self.token_stream.push(Token::Integer(word_str.parse::<i32>().map_err(|_| LexError {
                        message: format!("invalid integer format [{}]", word_str),
                        pos: self.pos.clone(),
                    })?));
                }
                continue;
            }
            // identify keyword
            let word = self.get_next_word();
            if let Some(word_str) = word {
                match self.identify_token(&word_str) {
                    Some(token) => {self.token_stream.push(token); continue},
                    None => { if self.is_ident_valid(&word_str) {
                        self.token_stream.push(Token::Ident(word_str));
                        continue;
                    } else {
                        return Err(LexError {
                            message: format!("Unknown token [{}]", word_str),
                            pos: self.pos.clone(),
                        });}
                    }
                }
            }
        }
        Ok(&self.token_stream)
    }
}
