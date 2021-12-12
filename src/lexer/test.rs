use crate::token::Token;

#[test]
fn next_token() {
  let input = "=+(){},;";

  let expected = vec![
    Token::Assign,
    Token::Plus,
    Token::LParen,
    Token::RParen,
    Token::LBrace,
    Token::RBrace,
    Token::Comma,
    Token::Semicolon,
    Token::EOF,
  ];

  let mut lexer = super::new(input);
  for tt in expected {
    let tok = lexer.next_token();

    assert_eq!(tok, tt);
  }
}
