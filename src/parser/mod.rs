use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;
use error::*;

mod error;
#[cfg(test)]
mod test;

pub struct Parser<'a> {
  /// An instance of the lexer, on which we repeatedly call next_token() to get
  /// the next token in the input.
  lexer: Lexer<'a>,
  current_token: Token,
  /// next token, to decide whether we are at the end of the line or if we are
  /// at just the start of an arithmetic expression.
  next_token: Token,
  errors: ParseErrors,
}

pub fn new(lexer: Lexer<'_>) -> Parser {
  let mut parser = Parser {
    lexer,
    current_token: Token::Eof,
    next_token: Token::Eof,
    errors: vec![],
  };

  // read 2 tokens, so current_token and next_token are both set.
  for _ in 0..=1 {
    parser.move_to_next_tok();
  }

  parser
}

impl<'a> Parser<'a> {
  /// It' s used to move pointer to next token, and usually work with `self.parse_*` methods.
  fn move_to_next_tok(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.move_to_next_tok();
  }

  /// We're using a loop to parse statements until we encounter a Eof character
  pub fn parse(&mut self) -> ast::Program {
    let mut program: ast::Program = vec![];

    while !self.current_token_is(Token::Eof) {
      if let Some(stmt) = self.parse_stmt() {
        program.push(stmt);
      }
      self.move_to_next_tok();
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
    // This is equivalent to self.expect_next_is(Token::Ident(...))
    match &self.next_token {
      Token::Ident(_) => self.move_to_next_tok(),
      _ => {
        self.next_token_error(Token::Ident(String::from("<Identifier literal>")));
        return None;
      }
    };

    let name = match self.parse_ident() {
      Some(name) => name,
      None => return None,
    };

    if !self.expect_next_is(Token::Assign) {
      return None;
    }

    self.move_to_next_tok();

    // TODO: We're skipping the expression until we encounter a semicolon
    if self.next_token_is(&Token::Semicolon) {
      self.move_to_next_tok();
    }
    Some(ast::Statement::Let(name))
  }

  fn parse_ident(&self) -> Option<ast::Ident> {
    match &self.current_token {
      Token::Ident(ident) => Some(ast::Ident(ident.clone())),
      _ => None,
    }
  }

  fn expect_next_is(&mut self, tok: Token) -> bool {
    if self.next_token_is(&tok) {
      self.move_to_next_tok();
      true
    } else {
      self.next_token_error(tok);
      false
    }
  }

  fn current_token_is(&self, tok: Token) -> bool {
    self.current_token == tok
  }

  fn next_token_is(&self, tok: &Token) -> bool {
    self.next_token == *tok
  }

  fn next_token_error(&mut self, tok: Token) {
    self.errors.push(ParseError::new(
      ParseErrorKind::UnexpectedToken,
      format!(
        "expected next token to be {:?}, got {:?} instead.",
        tok, self.next_token
      ),
    ));
  }

  fn get_errors(&self) -> ParseErrors {
    self.errors.clone()
  }
}
