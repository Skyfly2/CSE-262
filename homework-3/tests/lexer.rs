extern crate lexer;
use lexer::tokenize;
use lexer::Lexer;
use lexer::Token;
use lexer::TokenType;
use std::fs;
use std::path::Path;

#[test]
fn question_3_lexer_new() {
    let lexer: Lexer = Lexer::new();
}

#[test]
fn question_4_read_file_ok() {
    let path = Path::new("test.asa");
    let value = fs::read_to_string(path).unwrap();
    let res = lexer::read_file(path).unwrap();
    // Check that each get the same result
    assert_eq!(res, value);
}

#[test]
fn question_4_read_file_err() {
    let path = Path::new("notfound.asa");
    let res = lexer::read_file(path);
    assert_eq!(res.is_err(), true);
}

#[test]
fn question_6_token_alpha() {
    let byte = 0x6a;
    let token = lexer::tokenize(byte);
    assert_eq!(
        token,
        lexer::Token {
            byte: byte,
            r#type: TokenType::Alpha
        }
    );
}

#[test]
fn question_6_token_digit() {
    let byte = 0x31;
    let token = lexer::tokenize(byte);
    assert_eq!(
        token,
        lexer::Token {
            byte: byte,
            r#type: TokenType::Digit
        }
    );
}

#[test]
fn question_6_token_whitespace() {
    let byte = 0x20;
    let token = lexer::tokenize(byte);
    assert_eq!(
        token,
        lexer::Token {
            byte: byte,
            r#type: TokenType::Whitespace
        }
    );
}

#[test]
fn question_6_token_grouping() {
    let byte = 0x28;
    let token = lexer::tokenize(byte);
    assert_eq!(
        token,
        lexer::Token {
            byte: byte,
            r#type: TokenType::Grouping
        }
    );
}

#[test]
fn question_6_token_symbol() {
    let byte = 0x2B;
    let token = lexer::tokenize(byte);
    assert_eq!(
        token,
        lexer::Token {
            byte: byte,
            r#type: TokenType::Symbol
        }
    );
}

#[test]
fn question_7_lex_1() {
    let mut lexer = Lexer::new();
    let result: Vec<Token> = lexer.lex(String::from("Hello World!"));
    let mut actual: Vec<Token> = Vec::new();
    actual.push(lexer::tokenize(0x48));
    actual.push(lexer::tokenize(0x65));
    actual.push(lexer::tokenize(0x6C));
    actual.push(lexer::tokenize(0x6C));
    actual.push(lexer::tokenize(0x6F));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x57));
    actual.push(lexer::tokenize(0x6F));
    actual.push(lexer::tokenize(0x72));
    actual.push(lexer::tokenize(0x6C));
    actual.push(lexer::tokenize(0x64));
    actual.push(lexer::tokenize(0x21));
    let mut count = 0;
    for token in result {
        assert_eq!(token.byte, actual[count].byte);
        assert_eq!(token.r#type, actual[count].r#type);
        count += 1;
    }
}

#[test]
fn question_7_lex_2() {
    let mut lexer = Lexer::new();
    let result: Vec<Token> = lexer.lex(String::from("Hola"));
    let mut actual: Vec<Token> = Vec::new();
    actual.push(lexer::tokenize(0x48));
    actual.push(lexer::tokenize(0x6F));
    actual.push(lexer::tokenize(0x6C));
    actual.push(lexer::tokenize(0x61));
    let mut count = 0;
    for token in result {
        assert_eq!(token.byte, actual[count].byte);
        assert_eq!(token.r#type, actual[count].r#type);
        count += 1;
    }
}

#[test]
fn question_7_lex_3() {
    let mut lexer = Lexer::new();
    let result: Vec<Token> = lexer.lex(String::from("Rust is fun"));
    let mut actual: Vec<Token> = Vec::new();
    actual.push(lexer::tokenize(0x52));
    actual.push(lexer::tokenize(0x75));
    actual.push(lexer::tokenize(0x73));
    actual.push(lexer::tokenize(0x74));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x69));
    actual.push(lexer::tokenize(0x73));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x66));
    actual.push(lexer::tokenize(0x75));
    actual.push(lexer::tokenize(0x6E));
    let mut count = 0;
    for token in result {
        assert_eq!(token.byte, actual[count].byte);
        assert_eq!(token.r#type, actual[count].r#type);
        count += 1;
    }
}

#[test]
fn question_7_lex_4() {
    let mut lexer = Lexer::new();
    let result: Vec<Token> = lexer.lex(String::from("let x = 5;"));
    let mut actual: Vec<Token> = Vec::new();
    actual.push(lexer::tokenize(0x6C));
    actual.push(lexer::tokenize(0x65));
    actual.push(lexer::tokenize(0x74));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x78));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x3D));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x35));
    actual.push(lexer::tokenize(0x3B));
    let mut count = 0;
    for token in result {
        assert_eq!(token.byte, actual[count].byte);
        assert_eq!(token.r#type, actual[count].r#type);
        count += 1;
    }
}

#[test]
fn question_7_lex_5() {
    let mut lexer = Lexer::new();
    let result: Vec<Token> = lexer.lex(String::from("(5 + 3 == 8)"));
    let mut actual: Vec<Token> = Vec::new();
    actual.push(lexer::tokenize(0x28));
    actual.push(lexer::tokenize(0x35));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x2B));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x33));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x3D));
    actual.push(lexer::tokenize(0x3D));
    actual.push(lexer::tokenize(0x20));
    actual.push(lexer::tokenize(0x38));
    actual.push(lexer::tokenize(0x29));
    let mut count = 0;
    for token in result {
        assert_eq!(token.byte, actual[count].byte);
        assert_eq!(token.r#type, actual[count].r#type);
        count += 1;
    }
}
