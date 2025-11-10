mod lexer;
mod token;
mod parser;

use std::env;
use parser::Parser;

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
    let main_src_filename = args.next().ok_or_else(|| "Usage: mpl <source_filename>")?; // get source filename
    let mut p = Parser::new();
    p.parse(main_src_filename)?;
    Ok(())
}
