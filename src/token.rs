use strum_macros::EnumString;
#[derive(Clone,Debug, PartialEq, EnumString)]
pub enum Token {
    #[strum(serialize = "import")]
    Import,
    #[strum(serialize = "fn")]
    Fn,
    #[strum(serialize = "main")]
    Main,
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "println")]
    Println,
    #[strum(serialize = "call")]
    Call,
    #[strum(serialize = "_ident")]
    Ident(String),
    #[strum(serialize = "_str")]
    Str(String),
    #[strum(serialize = "_integer")]
    Integer(i32),
    #[strum(serialize = "_float")]
    Float(f64),
    #[strum(serialize = "to_str")]
    ToStr,
    #[strum(serialize = "[")]
    LBracket,
    #[strum(serialize = "]")]
    RBracket,
    #[strum(serialize = "(")]
    LParen,
    #[strum(serialize = ")")]
    RParen,
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Star,
    #[strum(serialize = "/")]
    Slash,
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "nl")]
    Nl,
    #[strum(serialize = "local")]
    Local,
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
    #[strum(serialize = "=")]
    Equal,
    #[strum(serialize = "int")]
    IntType,
    #[strum(serialize = "float")]
    FloatType,
    #[strum(serialize = "let")]
    Let,
    #[strum(serialize = "for")]
    For,
    #[strum(serialize = "to")]
    To,
    #[strum(serialize = "step")]
    Step,
    #[strum(serialize = "next")]
    Next,
    #[strum(serialize = "break")]
    Break,
    #[strum(serialize = "_eof")]
    Eof,
}
