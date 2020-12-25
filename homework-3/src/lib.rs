pub mod lexer;
pub use self::lexer::Lexer;
pub use self::lexer::Token;
pub use self::lexer::TokenType;
pub use self::lexer::tokenize;
use std::fs::File;
use std::io::prelude::*;
use std::io::Stdin;
use std::io::{self, BufReader};
use std::path::Path;

pub fn read_file(path: &std::path::Path) -> std::io::Result<String> {
    // Open file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    // Loop through lines until EOF marker is reached (return 0)
    let mut length = 1;
    while length != 0 {
        length = reader.read_line(&mut content)?;
    }
    Ok(content)
}
