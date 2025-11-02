mod lexer;
mod token;

use std::env;
use lexer::Lexer;

fn main() {
    if let Err(e) = real_main() {
        // Use Display, not Debug
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn real_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _program = args.next(); // skip program name
    let src_filename = args.next().ok_or_else(|| "Usage: mpl <source_filename>")?; // get source filename
    let mut lex  = Lexer::new(src_filename); // create lexer
    let token_stream = lex.get_all_token()?; // get tokens from lexer
    println!("{}", token_stream);
    Ok(())
}
