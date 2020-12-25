extern crate lexer;
use lexer::Lexer;
use std::env;
use std::path::Path;

fn main() {
    let mut lexer = Lexer::new();
    let mut args: String = String::from("");

    // Get command line args
    for argument in env::args() {
        args = String::from(argument);
    }
    args.push_str(".asa");
    let mut content: String = String::from("");
    let mut tokens: Vec<lexer::Token> = Vec::new();

    // Open and read the given file
    match lexer::read_file(Path::new(&args)) {
        Ok(val) => {
            content = val;
            // lex the file
            tokens = lexer.lex(content);
        }
        // File doesn't exist
        Err(err) => print!("Error: {}", err),
    }
}
