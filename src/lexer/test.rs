use crate::token;
use crate::token::TokenType;

#[test]
fn next_token() {
  let input = "=+(){},;";

  let expected = vec![
    (TokenType::Assign, "="),
    (TokenType::Plus, "+"),
    (TokenType::LParen, "("),
    (TokenType::RParen, ")"),
    (TokenType::LBrace, "{"),
    (TokenType::RBrace, "}"),
    (TokenType::Comma, ","),
    (TokenType::Semicolon, ";"),
    (TokenType::EOF, token::CHAR_NUL),
  ];

  let mut lexer = super::new(input);
  for (tt, literal) in expected {
    let tok = lexer.next_token();

    assert_eq!(tok.token_type, tt);
    assert_eq!(tok.literal, literal);
  }
}
