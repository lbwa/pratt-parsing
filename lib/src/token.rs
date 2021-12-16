#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Illegal,
  Eof,

  // identifier + literal
  Ident(String),
  Int(i64),

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
  Lt,
  /// `>`
  Gt,
  Eq,
  NotEq,

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
  True,
  False,
  If,
  Else,
  Return,
}

const CHAR_NUL: &str = "\u{0000}";

pub const CHAR_NUL_BYTE: u8 = CHAR_NUL.as_bytes()[0];
