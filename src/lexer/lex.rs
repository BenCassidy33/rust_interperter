use std::{iter::Skip, ops::Index};

use super::types::{Lexer, Token, TokenType};

pub trait LexerOps {
    fn new(input: String) -> Self;
    fn next_char(&mut self);
    fn tokenize(tok: String) -> Token;
    fn all_tokens(&mut self) -> Vec<Token>;
}

impl LexerOps for Lexer {
    fn new(input: String) -> Self {
        let mut lex = Lexer {
            input,
            ch_pos: -1,
            next_ch_pos: 0,
            ch: 0,
        };

        lex.next_char();

        lex
    }

    fn next_char(&mut self) {
        if self.next_ch_pos >= self.input.len() as i32 {
            self.ch = 0;
        } else {
            self.ch = self.input.chars().nth(self.next_ch_pos as usize).unwrap() as u32;
            self.ch_pos += 1;
            self.next_ch_pos += 1;
        }
    }

    fn tokenize(tok: String) -> Token {
        println!("{:#?}", tok);

        let token = match tok.as_str() {
            "\n" | "" => TokenType::EOF,
            "let" => TokenType::LET,
            "fn" => TokenType::FUNCTION,
            "=" => TokenType::ASSIGN,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "+" => TokenType::PLUS,
            "," => TokenType::COMMA,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "<" => TokenType::LT,
            ">" => TokenType::GT,
            "<=" => TokenType::LTEQ,
            ">=" => TokenType::GTEQ,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "return" => TokenType::RETURN,
            _ => {
                if let Ok(itok) = tok.parse::<isize>() {
                    TokenType::INT(itok)
                } else {
                    TokenType::IDENT(tok.clone())
                }
            }
        };

        Token {
            token_type: token,
            literal: tok,
        }
    }

    fn all_tokens(&mut self) -> Vec<Token> {
        let mut char_toks: Vec<char> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut token: Vec<String> = Vec::new();

        while self.ch != 0 {
            let char = char::from_u32(self.ch).unwrap();
            char_toks.push(char);
            self.next_char();
        }

        char_toks.push(char::from_u32(self.ch).unwrap());

        println!("{:#?}", char_toks);

        for idx in 0..char_toks.len() {
            if char_toks[idx].is_ascii_whitespace() || idx == char_toks.len() - 1 {
                let tok = token.join("");

                if tok.ends_with(";") {
                    let (tok1, tok2) = tok.split_at(tok.len() - 1);
                    tokens.push(Lexer::tokenize(tok1.to_string()));
                    tokens.push(Lexer::tokenize(tok2.to_string()));
                    token.clear();
                } else {
                    tokens.push(Lexer::tokenize(token.join("")));
                    token.clear();
                }
            } else {
                token.push(char_toks[idx].to_string());
            }
        }

        tokens
    }
}
