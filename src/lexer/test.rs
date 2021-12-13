use crate::token::Token;

#[test]
fn next_token() {
  let input = "let five = 5;
  let ten = 10;
  let add = fn(x, y) {
    x + y;
  };
  let result = add(five, ten);
  !-/*1;
  2 < 3 > 2;

  if (4 < 5) {
    return true;
  } else {
    return false;
  }
  ";

  let expected = vec![
    Token::Let,
    Token::Ident(String::from("five")),
    Token::Assign,
    Token::Int(5),
    Token::Semicolon,
    //
    Token::Let,
    Token::Ident(String::from("ten")),
    Token::Assign,
    Token::Int(10),
    Token::Semicolon,
    //
    Token::Let,
    Token::Ident(String::from("add")),
    Token::Assign,
    Token::Function,
    Token::LParen,
    Token::Ident(String::from("x")),
    Token::Comma,
    Token::Ident(String::from("y")),
    Token::RParen,
    Token::LBrace,
    //
    Token::Ident(String::from("x")),
    Token::Plus,
    Token::Ident(String::from("y")),
    Token::Semicolon,
    //
    Token::RBrace,
    Token::Semicolon,
    //
    Token::Let,
    Token::Ident(String::from("result")),
    Token::Assign,
    Token::Ident(String::from("add")),
    Token::LParen,
    Token::Ident(String::from("five")),
    Token::Comma,
    Token::Ident(String::from("ten")),
    Token::RParen,
    Token::Semicolon,
    //
    Token::Bang,
    Token::Minus,
    Token::Slash,
    Token::Asterisk,
    Token::Int(1),
    Token::Semicolon,
    //
    Token::Int(2),
    Token::Lt,
    Token::Int(3),
    Token::Gt,
    Token::Int(2),
    Token::Semicolon,
    //
    Token::If,
    Token::LParen,
    Token::Int(4),
    Token::Lt,
    Token::Int(5),
    Token::RParen,
    Token::LBrace,
    //
    Token::Return,
    Token::True,
    Token::Semicolon,
    //
    Token::RBrace,
    Token::Else,
    Token::LBrace,
    //
    Token::Return,
    Token::False,
    Token::Semicolon,
    //
    Token::RBrace,
    Token::EOF,
  ];

  let mut lexer = super::new(input);
  for tt in expected {
    let tok = lexer.next_token();

    assert_eq!(tt, tok);
  }
}
