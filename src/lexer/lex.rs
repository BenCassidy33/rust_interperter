use super::types::{Lexer, Token, TokenType, IDENTS};

pub trait LexerOps {
    fn new(input: String) -> Self;
    fn next_char(&mut self);
    fn next_token(&mut self) -> Token;
    fn read_indent(&mut self) -> String;
    fn ident_lookup(ident: String) -> TokenType;
    fn eat_whitespace(&mut self);
    fn read_number(&mut self) -> String;
    fn peek_char(&self) -> Result<char, ()>;
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

    fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let token: Token = match char::from_u32(self.ch).unwrap().to_string().as_str() {
            "=" => Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            ";" => Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            "(" => Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            ")" => Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            "{" => Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            "}" => Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            "[" => Token {
                token_type: TokenType::LBRACKET,
                literal: "[".to_string(),
            },
            "]" => Token {
                token_type: TokenType::RBRACKET,
                literal: "]".to_string(),
            },
            "+" => Token {
                token_type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            "," => Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            "<" => Token {
                token_type: TokenType::LT,
                literal: "<".to_string(),
            },
            ">" => Token {
                token_type: TokenType::GT,
                literal: ">".to_string(),
            },
            "" => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },

            ch => {
                println!("{}", ch);
                if ch.chars().into_iter().collect::<Vec<char>>().len() > 1 {
                    panic!("Something went wrong checking for char");
                } else if ch.chars().into_iter().nth(0).unwrap().is_ascii_alphabetic() {
                    let lit = self.read_indent();
                    Token {
                        literal: lit.clone(),
                        token_type: Lexer::ident_lookup(lit),
                    }
                } else if ch.parse::<isize>().is_ok() {
                    let val = self.read_number();
                    Token {
                        token_type: TokenType::INT(val.parse::<isize>().unwrap()),
                        literal: val.to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenType::ILLEGAL,
                        literal: ch.to_string(),
                    }
                }
            }
        };

        self.next_char();

        token
    }

    fn read_number(&mut self) -> String {
        let pos = self.ch_pos;

        while self
            .input
            .split("")
            .collect::<Vec<&str>>()
            .into_iter()
            .nth(self.next_ch_pos as usize + 1)
            .unwrap()
            .parse::<isize>()
            .is_ok()
        {
            self.next_char();
        }

        self.input[pos as usize..self.ch_pos as usize + 1].to_string()
    }

    fn peek_char(&self) -> Result<char, ()> {
        if self.next_ch_pos as usize >= self.input.len() {
            return Err(());
        } else {
            Ok(self
                .input
                .clone()
                .split("")
                .collect::<Vec<&str>>()
                .into_iter()
                .nth(self.next_ch_pos as usize - 1)
                .unwrap()
                .chars()
                .nth(0)
                .unwrap())
        }
    }

    fn read_indent(&mut self) -> String {
        let pos = self.ch_pos;

        while char::from_u32(self.ch).unwrap().is_ascii_alphabetic()
            || char::from_u32(self.ch).unwrap() == '_'
            || char::from_u32(self.ch).unwrap() == '!'
        {
            self.next_char();
        }
        self.input[pos as usize..self.ch_pos as usize].to_string()
    }

    fn ident_lookup(ident: String) -> TokenType {
        return match IDENTS.get(ident.as_str()) {
            Some(ident) => ident.clone(),
            None => TokenType::IDENT(ident),
        };
    }

    fn eat_whitespace(&mut self) {
        while char::from_u32(self.ch).unwrap().is_ascii_whitespace() {
            self.next_char()
        }
    }
}
