#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
  Illegal,
  Eof,

  // identifier + literal
  Ident(&'a str),
  Int(i64),
  Bool(bool),

  // statements
  If,
  Else,

  // operators
  /// `=`
  Assign,
  /// `+`
  Plus,
  /// `-`
  Minus,
  /// `!`
  Bang,
  /// `*`
  Asterisk,
  /// `/`
  Slash,
  /// `<`
  LessThan,
  /// `>`
  GreaterThan,
  /// `=`
  Equal,
  /// `≠`
  NotEqual,
  /// `≤`
  LessThanEqual,
  /// `≥`
  GreaterThanEqual,

  // delimiters
  /// ,
  Comma,
  /// ;
  Semicolon,
  /// (
  LParen,
  /// )
  RParen,
  /// {
  LBrace,
  /// }
  RBrace,

  // reserved words
  Function,
  Let,
  Return,
}

const CHAR_NUL: &str = "\u{0000}";

pub const CHAR_NUL_BYTE: u8 = CHAR_NUL.as_bytes()[0];
