#[cfg(test)]
mod test;

use crate::token;
use crate::token::Token;

pub struct Lexer<'a> {
  input: &'a str,
  bytes: Vec<u8>,
  /// current position in input (points to current char)
  pos: usize,
  /// current reading position in input (after current char)
  read_pos: usize,
  /// current char under examination
  ///
  /// The low-level representation of Rust's String type is `Vec<u8>`,
  /// see https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation
  ch: u8,
}

pub fn new(input: &str) -> Lexer {
  let mut lexer = Lexer {
    input,
    bytes: input.as_bytes().to_owned(),
    pos: 0,
    read_pos: 0,
    ch: token::CHAR_NUL_BYTE,
  };
  lexer.read_char();
  lexer
}

impl<'a> Lexer<'a> {
  fn read_char(&mut self) {
    self.ch = if self.read_pos >= self.input.len() {
      token::CHAR_NUL_BYTE
    } else {
      self.bytes[self.read_pos]
    };
    self.pos = self.read_pos;
    self.read_pos += 1;
  }

  fn next_token(&mut self) -> token::Token {
    let tok = match self.ch {
      b'=' => Token::Assign,
      b';' => Token::Semicolon,
      b'(' => Token::LParen,
      b')' => Token::RParen,
      b',' => Token::Comma,
      b'+' => Token::Plus,
      b'{' => Token::LBrace,
      b'}' => Token::RBrace,
      token::CHAR_NUL_BYTE => Token::EOF,
      _ => {
        panic!();
      }
    };
    self.read_char();
    tok
  }
}
