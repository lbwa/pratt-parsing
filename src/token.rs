#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Illegal,
  EOF,

  Ident,
  Int,

  Assign,
  Plus,

  Comma,
  Semicolon,

  LParen,
  RParen,
  LBrace,
  RBrace,

  Function,
  Let,
}

pub const CHAR_NUL: &str = "\u{0000}";

pub const CHAR_NUL_BYTE: u8 = CHAR_NUL.as_bytes()[0];

pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

pub fn new(token_type: TokenType, ch: u8) -> Token {
  Token {
    token_type,
    literal: String::from_utf8(vec![ch]).unwrap(),
  }
}
