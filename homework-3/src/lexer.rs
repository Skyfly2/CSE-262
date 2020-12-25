pub struct Lexer {}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn lex(&mut self, source: String) -> Vec<Token> {
        let mut vector: Vec<Token> = Vec::new();
        for byte in source.bytes() {
            let token = tokenize(byte);
            vector.push(token);
        }
        vector
    }
}

pub fn tokenize(byte: u8) -> Token {
    match byte {
        0..=32 => Token {
            r#type: TokenType::Whitespace,
            byte: byte,
        },
        33..=39 | 42..=47 | 58..=59 | 61 | 63..=64 | 92 | 124 | 94..=96 | 126u8..=std::u8::MAX => {
            Token {
                r#type: TokenType::Symbol,
                byte: byte,
            }
        }
        48..=57 => Token {
            r#type: TokenType::Digit,
            byte: byte,
        },
        65..=90 | 97..=122 => Token {
            r#type: TokenType::Alpha,
            byte: byte,
        },
        91 | 93 | 40 | 41 | 123 | 125 | 60 | 62 => Token {
            r#type: TokenType::Grouping,
            byte: byte,
        },
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub byte: u8,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Alpha,
    Digit,
    Whitespace,
    Grouping,
    Symbol,
}
