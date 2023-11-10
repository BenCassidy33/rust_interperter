#[cfg(test)]
mod lexer {
    use rust_interperter::lexer::{
        lex::{self, LexerOps},
        types::{Lexer, Token, TokenType},
    };
    #[test]

    fn new_lexer() {
        let expected = Lexer {
            input: "Let x = 5".to_string(),
            ch_pos: 0,
            next_ch_pos: 1,
            ch: 'L' as u32,
        };
        let lex = Lexer::new("Let x = 5".into());
        assert_eq!(lex, expected);
    }

    #[test]
    fn read_char() {
        let mut lex = Lexer::new("Let x = 5".to_string());
        lex.next_char();

        let expected = Lexer {
            input: "Let x = 5".to_string(),
            ch_pos: 1,
            next_ch_pos: 2,
            ch: 'e' as u32,
        };

        assert_eq!(lex, expected);
    }

    #[test]
    fn next_token() {
        let expected = Token {
            token_type: TokenType::LET,
            literal: "let".to_string(),
        };

        let tok = Lexer::tokenize("let".to_string());

        assert_eq!(tok, expected);
    }

    #[test]
    fn tokenize() {
        let expected = Token {
            token_type: TokenType::INT(5),
            literal: "5".to_string(),
        };

        let test = Lexer::tokenize("5".to_string());
        assert_eq!(test, expected);
    }

    #[test]
    fn all_tokens() {
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::INT(5),
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
        ];

        let mut lex = Lexer::new("let x = 5;".to_string());
        let res = lex.all_tokens();

        assert_eq!(res, expected);
    }
}
