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

impl<'a> Lexer<'a> {
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

  /// similar to `self.read_char`, except that it doesn't increment `self.pos`
  /// and `self.read_pos`. We only want to "peek" ahead in the input and not
  /// move around it.
  fn peek_char(&mut self) -> u8 {
    if self.read_pos >= self.input.len() {
      token::CHAR_NUL_BYTE
    } else {
      self.bytes[self.read_pos]
    }
  }

  fn read_char(&mut self) {
    self.ch = if self.read_pos >= self.input.len() {
      token::CHAR_NUL_BYTE
    } else {
      self.bytes[self.read_pos]
    };
    self.pos = self.read_pos;
    self.read_pos += 1;
  }

  fn read_identifier(&mut self) -> Token<'a> {
    let from = self.pos;

    while let b'a'..=b'z' | b'A'..=b'Z' | b'_' = self.ch {
      self.read_char();
    }

    let literal = &self.input[from..self.pos];

    match literal {
      "fn" => Token::Function,
      "let" => Token::Let,
      "true" => Token::Bool(true),
      "false" => Token::Bool(false),
      "if" => Token::If,
      "else" => Token::Else,
      "return" => Token::Return,
      _ => Token::Ident(literal),
    }
  }

  fn read_number(&mut self) -> Token<'a> {
    let from = self.pos;

    while let b'0'..=b'9' = self.ch {
      self.read_char();
    }
    match self.input[from..self.pos].parse::<i64>() {
      Ok(value) => Token::Int(value),
      _ => Token::Illegal,
    }
  }

  pub fn move_to_next_tok(&mut self) -> token::Token<'a> {
    self.skip_whitespace();
    let tok = match self.ch {
      // operators
      b'=' => {
        if self.peek_char() == b'=' {
          self.read_char();
          Token::Equal
        } else {
          Token::Assign
        }
      }
      b'+' => Token::Plus,
      b'-' => Token::Minus,
      b'!' => {
        if self.peek_char() == b'=' {
          self.read_char();
          Token::NotEqual
        } else {
          Token::Bang
        }
      }
      b'/' => Token::Slash,
      b'*' => Token::Asterisk,
      b'<' => Token::LessThan,
      b'>' => Token::GreaterThan,

      // delimiters
      b';' => Token::Semicolon,
      b',' => Token::Comma,
      b'(' => Token::LParen,
      b')' => Token::RParen,
      b'{' => Token::LBrace,
      b'}' => Token::RBrace,

      b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
        // NOTE: terminate next_token function evaluation and
        // use read_identifier's returned value as next_token returned value
        return self.read_identifier();
      }
      b'0'..=b'9' => {
        // NOTE: terminate next_token function evaluation and
        // use read_identifier's returned value as next_token returned value
        return self.read_number();
      }

      token::CHAR_NUL_BYTE => Token::Eof,
      _ => Token::Illegal,
    };
    self.read_char();
    tok
  }

  fn skip_whitespace(&mut self) {
    while let b' ' | b'\t' | b'\n' | b'\r' = self.ch {
      self.read_char();
    }
  }
}

fn is_number(ch: u8) -> bool {
  (b'0'..=b'9').contains(&ch)
}
