use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    pub input: String,
    pub ch_pos: i32,
    pub next_ch_pos: i32,
    pub ch: u32,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(isize),

    LET,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    COMMA,
    SEMICOLON,
    FUNCTION,
    IF,
    ELSE,

    LT,
    GT,
    LTEQ,
    GTEQ,

    TRUE,
    FALSE,

    RETURN,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    UNKNOWN,
}

lazy_static! {
    pub static ref IDENTS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("let", TokenType::LET);
        map.insert("fn", TokenType::FUNCTION);
        map.insert("if", TokenType::IF);
        map.insert("else", TokenType::ELSE);
        map.insert("return", TokenType::RETURN);
        map.insert("true", TokenType::TRUE);
        map.insert("false", TokenType::FALSE);
        map
    };
}
