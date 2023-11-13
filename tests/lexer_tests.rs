#[cfg(test)]
mod lexer {
    use rust_interperter::lexer::{
        lex::LexerOps,
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
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let res = add(five, ten);
        "#;

        let mut lex = Lexer::new(input.to_string());

        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("five".to_string()),
                literal: "five".to_string(),
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
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("ten".to_string()),
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::INT(10),
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("add".to_string()),
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("y".to_string()),
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("y".to_string()),
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("res".to_string()),
                literal: "res".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("add".to_string()),
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("five".to_string()),
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("ten".to_string()),
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
        ];

        for expected_token in expected.iter() {
            let actual = lex.next_token();

            println!(
                "Expected: {:?}, Actual: {:?} \n ===============================================================================================",
                expected_token, actual
            );
            assert_eq!(
                *expected_token, actual,
                "\n\nExpected: {:?}, \nGot: {:?} \n\n",
                *expected_token, actual
            );
        }
    }
}
