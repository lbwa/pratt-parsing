use crate::token::Token;

#[test]
fn tokenize() {
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

  12 == 12;
  11 != 12;
  ";

  let expected = vec![
    Token::Let,
    Token::Ident("five"),
    Token::Assign,
    Token::Int(5),
    Token::Semicolon,
    //
    Token::Let,
    Token::Ident("ten"),
    Token::Assign,
    Token::Int(10),
    Token::Semicolon,
    //
    Token::Let,
    Token::Ident("add"),
    Token::Assign,
    Token::Function,
    Token::LParen,
    Token::Ident("x"),
    Token::Comma,
    Token::Ident("y"),
    Token::RParen,
    Token::LBrace,
    //
    Token::Ident("x"),
    Token::Plus,
    Token::Ident("y"),
    Token::Semicolon,
    //
    Token::RBrace,
    Token::Semicolon,
    //
    Token::Let,
    Token::Ident("result"),
    Token::Assign,
    Token::Ident("add"),
    Token::LParen,
    Token::Ident("five"),
    Token::Comma,
    Token::Ident("ten"),
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
    Token::LessThan,
    Token::Int(3),
    Token::GreaterThan,
    Token::Int(2),
    Token::Semicolon,
    //
    Token::If,
    Token::LParen,
    Token::Int(4),
    Token::LessThan,
    Token::Int(5),
    Token::RParen,
    Token::LBrace,
    //
    Token::Return,
    Token::Bool(true),
    Token::Semicolon,
    //
    Token::RBrace,
    Token::Else,
    Token::LBrace,
    //
    Token::Return,
    Token::Bool(false),
    Token::Semicolon,
    //
    Token::RBrace,
    //
    Token::Int(12),
    Token::Equal,
    Token::Int(12),
    Token::Semicolon,
    Token::Int(11),
    Token::NotEqual,
    Token::Int(12),
    Token::Semicolon,
    Token::Eof,
  ];

  let mut lexer = super::Lexer::new(input);
  for tt in expected {
    let tok = lexer.move_to_next_tok();

    assert_eq!(tt, tok);
  }
}
