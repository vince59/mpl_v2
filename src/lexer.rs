use crate::token::Token;
use std::fs;
use std::fmt;
use std::path::{Path, MAIN_SEPARATOR};
use std::cmp::Reverse;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LexToken {
    pub token: Token,
    pub pos: Position,
}

impl fmt::Display for LexToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{} [{:?}]\n", self.pos.file_name, self.pos.line, self.pos.col, self.token)
    }
}

pub struct TokenStream {
    pub tokens: Vec<LexToken>,
}

// Display all tokens in the token stream
impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, token) in self.tokens.iter().enumerate() {
            write!(f, "{} -> {}", i+1, token)?;
        }
        Ok(())
    }
}

// Lexer error
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
            "Token error : [{}] at {} ({}:{})\n",
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
    pos: Position,
    i: usize, // current index in the source file
}

impl Lexer {
    pub fn new(src_filename: String) -> Self {
        let filename = src_filename.clone();
        Self {
            src_filename,
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
            self.pos.col = 1;
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
        let (mut i_tmp, mut col_tmp, mut line_tmp) = self.save_state();
        loop {
            let c = self.get_next_char();
            if c == '\0' || c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                break;
            }
            match self.identify_token(&c.to_string()) {
                Some(_) => { self.restore_state( (i_tmp, col_tmp, line_tmp) ); break; },
                None => {word.push(c); (i_tmp, col_tmp, line_tmp) = self.save_state();}
            }
        }
        if word.is_empty() { None } else { Some(word) }
    }

    // try to identify a symbol (one char only)
    fn try_symbol(&mut self) -> Option<Token> {
        let (i_tmp, col_tmp, line_tmp) = self.save_state();
        let mut word = String::new();
        let c = self.get_next_char();
        word.push(c);
        match self.identify_token(&word) {
            None => {
                self.restore_state((i_tmp, col_tmp, line_tmp));
                None
            }
            Some(token) => Some(token),
        }
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

    // skip comment multiple line
    fn skip_comment_multiple_line(&mut self) -> Result<(), LexError> {
        let mut close = true; // by default, the comment is closed (case of no comment)
        if let Some(look_ahead) = self.look_ahead(2) {
            // look ahead 2 chars
            if look_ahead == "/*" {
                // Removing the attribute from the expression
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
        if ch == '.' { true } else { ch.is_ascii_digit() }
    }

    fn try_number(&mut self) -> Option<String> {
        let mut word = String::new();
        let (i_tmp, col_tmp, line_tmp) = self.save_state();
        let c = self.get_next_char();
        let (mut i_tmp2, mut col_tmp2, mut line_tmp2) = self.save_state();
        if Self::is_digit(c) {
            let mut c = c; // Use the first character we already read
            while c != '\0' {
                if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
                    break;
                }
                match self.identify_token(&c.to_string()) {
                    Some(_) => { self.restore_state( (i_tmp2, col_tmp2, line_tmp2) ); break; },
                    None => {word.push(c); (i_tmp2, col_tmp2, line_tmp2) = self.save_state();}
                }
                c = self.get_next_char();
            }
            Some(word)
        } else {
            self.restore_state((i_tmp, col_tmp, line_tmp));
            None
        }
    }

    // try to identify a string
    fn try_string(&mut self) -> Result<Option<String>, LexError> {
        let mut str = String::new();
        let (i_tmp, col_tmp, line_tmp) = self.save_state();
        let c = self.get_next_char();
        if c == '"' {
            let mut c = self.get_next_char();
            while c != '\0' {
                if c == '\n' || c == '\r' {
                    return Err(LexError {
                        message: "Unclosed string".to_string(),
                        pos: self.pos.clone(),
                    });
                }
                if c == '"' {
                    //self.get_next_char();
                    return Ok(Some(str));
                } else {
                    str.push(c);
                    c = self.get_next_char();
                }
            }
        }
        self.restore_state((i_tmp, col_tmp, line_tmp));
        Ok(None)
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
                if !c.is_ascii_alphabetic() {
                    valid = false;
                }
            }
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

    fn get_import_list(tokens: &Vec<LexToken>) -> Result<Vec<(usize,String)>, LexError> {
        let mut imports: Vec<(usize, String)> = Vec::new();
        let mut k=0usize;
        for (i,t) in tokens.windows(2).enumerate() {
            let (cur, next) = (&t[0], &t[1]);
            if cur.token == Token::Import {
                if let Token::Str(ref s) = next.token {
                    if !imports.iter().any(|(_, exist)| *exist == *s) {
                        if k > 0 {
                            let previous_import_index = imports[k - 1].0;
                            let imports_are_not_consecutive = (previous_import_index + 2) != i;
                            if imports_are_not_consecutive {
                                return Err(LexError {
                                    message: format!("import can't be after instruction"),
                                    pos: next.pos.clone(),
                                });
                            }
                        }
                        k+=1;
                        imports.push((i,s.clone()));
                    } else {
                        return Err(LexError {
                                message: format!("import {} already defined", s),
                                pos: next.pos.clone(),
                            }
                        );
                    }
                } else {
                    return Err(LexError {
                        message: format!("import must be a string"),
                        pos: next.pos.clone(),
                    });
                }
            }
        }
       Ok(imports)
    }

    fn parse_file(filename: &String,pos: Option<Position>) -> Result<Vec<LexToken>, LexError> {
        let mut lexer = Lexer::new(filename.clone());
        let tokens = lexer.parse(pos)?;
        Ok(tokens)
    }

    fn parse(&mut self, pos: Option<Position>) -> Result<Vec<LexToken>, LexError> {
        self.src_text = fs::read_to_string(&self.src_filename).map_err(|e|LexError{
            message:format!("File not found {}",self.src_filename.clone()),
            pos:pos.unwrap()
        })?;

        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            self.skip_comment_single_line();
            self.skip_comment_multiple_line()?;
            let pos = self.pos.clone();
            // end of file
            if self.eof() {
                tokens.push(LexToken { token: Token::Eof, pos });
                break;
            }
            // identify string
            if let Some(str) = self.try_string()? {
                tokens.push(LexToken { token: Token::Str(str), pos });
                continue;
            }
            // identify symbols
            match self.try_symbol() {
                Some(token) => {
                    tokens.push(LexToken { token, pos });
                    continue;
                }
                _ => {}
            }
            // identify number
            if let Some(word_str) = self.try_number() {
                if word_str.contains('.') {
                    tokens
                        .push(LexToken {
                            token: Token::Float(word_str.parse::<f64>().map_err(|_| {
                                LexError {
                                    message: format!("invalid float number format [{}]", word_str),
                                    pos: pos.clone(),
                                }
                            })?),
                            pos,
                        });
                } else {
                    tokens
                        .push(LexToken {
                            token: Token::Integer(word_str.parse::<i32>().map_err(|_| {
                                LexError {
                                    message: format!("invalid integer format [{}]", word_str),
                                    pos: pos.clone(),
                                }
                            })?),
                            pos,
                        });
                }
                continue;
            }
            // identify keyword or an identifier
            let word = self.get_next_word();
            if let Some(word_str) = word {
                match self.identify_token(&word_str) {
                    Some(token) => {
                        tokens.push(LexToken { token: token, pos });
                        continue;
                    }
                    None => {
                        if self.is_ident_valid(&word_str) {
                            tokens.push(LexToken { token: Token::Ident(word_str), pos });
                            continue;
                        } else {
                            return Err(LexError {
                                message: format!("Unknown token [{}]", word_str),
                                pos,
                            });
                        }
                    }
                }
            }
        }
        Ok(tokens)
    }

    fn dir_with_sep(path: &str) -> Option<String> {
        let mut s = Path::new(path).parent()?.to_string_lossy().into_owned();
        if !s.ends_with(MAIN_SEPARATOR) {
            s.push(MAIN_SEPARATOR);
        }
        Some(s)
    }
    pub fn tokenize(&mut self) -> Result<TokenStream, LexError> {
        let mut tokens = Self::parse_file(&self.src_filename,None)?; // Parse the main file
        let working_path=Self::dir_with_sep(&self.src_filename).unwrap_or_else(|| ".".to_string());
        let mut imports = Self::get_import_list(&tokens)?; // Check imports
        imports.sort_by_key(|(i, _)| Reverse(*i));// Sort imports by index from the largest index to the smallest
        for (i,import_filename) in imports { // Tokenize each imported file
            let import_name = working_path.clone() + &import_filename;
            let mut imp_tokens = Self::parse_file(&import_name, Some(tokens[i].pos.clone()))?;
            imp_tokens.pop(); // remove the eof token
            // remove import and file name from the main program and insert all the tokens in the import file
            tokens.splice(i..=i+1, imp_tokens);
        }
        let token_stream = TokenStream { tokens };
        println!("{}", token_stream);
        Ok(token_stream)
    }
}
