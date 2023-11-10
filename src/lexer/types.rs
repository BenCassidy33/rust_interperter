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

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(isize),

    LET,
    ASSIGN,
    PLUS,
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
}
