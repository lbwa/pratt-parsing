#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Illegal,
  EOF,

  // identifier + literal
  Ident(String),
  Int(i64),

  // statements
  Assign,
  Plus,

  // delimiters
  Comma,
  Semicolon,
  LParen,
  RParen,
  LBrace,
  RBrace,

  // reserved words
  Function,
  Let,
}

pub const CHAR_NUL: &str = "\u{0000}";

pub const CHAR_NUL_BYTE: u8 = CHAR_NUL.as_bytes()[0];
