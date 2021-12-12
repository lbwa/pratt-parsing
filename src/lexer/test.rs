use crate::token::Token;

#[test]
fn next_token() {
  let input = "let five = 5;";

  let expected = vec![
    Token::Let,
    Token::Ident(String::from("five")),
    Token::Assign,
    Token::Int(5),
    Token::Semicolon,
    Token::EOF,
  ];

  let mut lexer = super::new(input);
  for tt in expected {
    let tok = lexer.next_token();

    assert_eq!(tt, tok);
  }
}
