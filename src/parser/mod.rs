#[cfg(test)]
mod test;

use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
  /// An instance of the lexer, on which we repeatedly call next_token() to get
  /// the next token in the input.
  lexer: Lexer<'a>,
  current_token: Token,
  /// next token, to decide whether we are at the end of the line or if we are
  /// at just the start of an arithmetic expression.
  next_token: Token,
}

pub fn new(lexer: Lexer<'_>) -> Parser {
  let mut parser = Parser {
    lexer,
    current_token: Token::Eof,
    next_token: Token::Eof,
  };

  // read 2 tokens, so current_token and next_token are both set.
  for _ in 0..=1 {
    parser.move_to_next();
  }

  parser
}

impl<'a> Parser<'a> {
  /// move to next token
  fn move_to_next(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.next_token();
  }

  /// loop which is used to parse statement until we encounter a Eof character
  pub fn parse(&mut self) -> ast::Program {
    let mut program: ast::Program = vec![];

    while !self.current_token_is(Token::Eof) {
      if let Some(stmt) = self.parse_stmt() {
        program.push(stmt);
      }
      self.move_to_next();
    }
    program
  }

  fn parse_stmt(&mut self) -> Option<ast::Statement> {
    match self.current_token {
      Token::Let => self.parse_let_stmt(),
      _ => None,
    }
  }

  fn parse_let_stmt(&mut self) -> Option<ast::Statement> {
    match &self.next_token {
      Token::Ident(_) => self.move_to_next(),
      _ => return None,
    };

    let name = match self.parse_ident() {
      Some(name) => name,
      None => return None,
    };

    if !self.expect_peek_is(Token::Assign) {
      return None;
    }

    self.move_to_next();

    // TODO: We're skipping the expression until we encounter a semicolon
    if self.next_token_is(Token::Semicolon) {
      self.move_to_next();
    }
    Some(ast::Statement::Let(name))
  }

  fn parse_ident(&self) -> Option<ast::Ident> {
    match &self.current_token {
      Token::Ident(ident) => Some(ast::Ident(ident.clone())),
      _ => None,
    }
  }

  fn expect_peek_is(&mut self, tok: Token) -> bool {
    if self.next_token_is(tok) {
      self.move_to_next();
      true
    } else {
      false
    }
  }

  fn current_token_is(&self, tok: Token) -> bool {
    self.current_token == tok
  }

  fn next_token_is(&self, tok: Token) -> bool {
    self.next_token == tok
  }
}
