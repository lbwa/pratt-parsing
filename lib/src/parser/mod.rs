mod error;
#[cfg(test)]
mod test;

use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;
use error::*;

/// The parsing approach is called Top Down Operator Precedence Parsing, or
/// Pratt parsing, was invented as an alternative to parsers based on
/// context-free grammars and Backus-Naur-Form.
///
/// And That is also the main difference: instead of associating parsing
/// functions with grammar rules (defined in BNF or EBNF), Pratt associates
/// these functions (called "semantic code") with single token types. A crucial
/// part of this idea is that each token type can have **2** parsing functions
/// associated with it, depending on the token's position - infix or prefix.
pub struct Parser<'input> {
  /// An instance of the lexer, on which we repeatedly call next_token() to get
  /// the next token in the input.
  lexer: Lexer<'input>,
  // The last of tokens we have read.
  current_token: Token<'input>,
  /// next token, to decide whether we are at the end of the line or if we are
  /// at just the start of an arithmetic expression.
  next_token: Token<'input>,
  errors: ParseErrors,
  stmts: ast::Program<'input>,
}

impl Parser<'_> {
  pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
      lexer,
      current_token: Token::Eof,
      next_token: Token::Eof,
      errors: vec![],
      stmts: vec![],
    };
    // read 2 tokens, so current_token and next_token are both set.
    for _ in 0..=1 {
      parser.move_to_next_tok();
    }
    parser
  }

  /// We're using a loop to parse statements until we encounter a Eof character
  pub fn parse(&mut self) -> &Self {
    while !self.current_token_is(&Token::Eof) {
      if let Some(stmt) = self.parse_stmt() {
        self.stmts.push(stmt);
      }
      self.move_to_next_tok();
    }
    self
  }

  pub fn get_errors(&self) -> ParseErrors {
    self.errors.clone()
  }

  pub fn get_stmts(&self) -> ast::Program {
    self.stmts.clone()
  }
}

// token
impl<'input> Parser<'input> {
  /// It' s used to move pointer to next token, and usually work with `self.parse_*` methods.
  fn move_to_next_tok(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.move_to_next_tok();
  }

  /// try to match next token, and move on.
  fn expect_next_is(&mut self, tok: Token) -> bool {
    if self.next_token_is(&tok) {
      self.move_to_next_tok();
      true
    } else {
      self.error_next_token(tok);
      false
    }
  }

  fn current_token_is(&self, tok: &Token) -> bool {
    self.current_token == *tok
  }

  fn next_token_is(&self, tok: &Token) -> bool {
    self.next_token == *tok
  }

  fn error_next_token(&mut self, tok: Token) {
    self.errors.push(ParseError::new(
      ParseErrorKind::UnexpectedToken,
      format!(
        "expected next token to be {:?}, got {:?} instead.",
        tok, self.next_token
      ),
    ));
  }

  fn parse_ident(&self) -> Option<ast::Ident<'input>> {
    match &self.current_token {
      Token::Ident(ident) => Some(ast::Ident(*ident)),
      _ => None,
    }
  }
}

// statements
impl<'input> Parser<'input> {
  fn parse_stmt(&mut self) -> Option<ast::Statement<'input>> {
    match self.current_token {
      Token::Let => self.parse_let_stmt(),
      Token::Return => self.parse_return_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  fn parse_let_stmt(&mut self) -> Option<ast::Statement<'input>> {
    // This is equivalent to self.expect_next_is(Token::Ident(...))
    match &self.next_token {
      // The reason we don't call self.expect_next_is function is we need
      // wildcard matching.
      Token::Ident(_) => self.move_to_next_tok(),
      _ => {
        self.error_next_token(Token::Ident("<Identifier literal>"));
        return None;
      }
    };

    let name = self.parse_ident()?;

    if !self.expect_next_is(Token::Assign) {
      return None;
    }

    self.move_to_next_tok();

    let value_expr = self.parse_expr(ast::Precedence::Lowest)?;
    if self.next_token_is(&Token::Semicolon) {
      self.move_to_next_tok();
    }
    Some(ast::Statement::Let(name, value_expr))
  }

  fn parse_return_stmt(&mut self) -> Option<ast::Statement<'input>> {
    self.move_to_next_tok();
    let value_expr = self.parse_expr(ast::Precedence::Lowest)?;
    while !self.current_token_is(&Token::Semicolon) {
      self.move_to_next_tok();
    }
    Some(ast::Statement::Return(value_expr))
  }

  fn parse_expr_stmt(&mut self) -> Option<ast::Statement<'input>> {
    if let Some(expr) = self.parse_expr(ast::Precedence::Lowest) {
      // expression semicolon is optional
      if self.next_token_is(&Token::Semicolon) {
        self.move_to_next_tok();
      }
      Some(ast::Statement::Expr(expr))
    } else {
      None
    }
  }

  fn parse_block_stmt(&mut self) -> Option<ast::BlockStatement<'input>> {
    self.move_to_next_tok();
    let mut stmts: ast::BlockStatement<'input> = vec![];
    while !self.current_token_is(&Token::RBrace) && !self.current_token_is(&Token::Eof) {
      if let Some(stmt) = self.parse_stmt() {
        stmts.push(stmt);
      }
      self.move_to_next_tok();
    }

    Some(stmts)
  }
}

// expressions
impl<'input> Parser<'input> {
  fn parse_expr(&mut self, precedence: ast::Precedence) -> Option<ast::Expr<'input>> {
    // handle prefix
    let mut left_expr = match self.current_token {
      Token::Ident(_) => self.parse_ident_expr(),
      Token::Int(_) => self.parse_int_expr(),
      Token::Minus | Token::Plus | Token::Bang => self.parse_prefix_expr(),
      Token::Bool(_) => self.parse_bool_expr(),
      Token::LParen => self.parse_grouped_expr(),
      Token::If => self.parse_if_expr(),
      Token::Function => self.parse_function_literal(),
      _ => {
        // unexpected token type
        self.error_no_prefix_parser();
        None
      }
    };

    // handle infix
    while !self.next_token_is(&Token::Semicolon) && precedence < self.next_token_precedence() {
      match self.next_token {
        Token::Plus
        | Token::Minus
        | Token::Slash
        | Token::Asterisk
        | Token::Equal
        | Token::NotEqual
        | Token::LessThan
        | Token::LessThanEqual
        | Token::GreaterThan
        | Token::GreaterThanEqual => {
          self.move_to_next_tok();
          left_expr = if let Some(expr) = left_expr {
            self.parse_infix_expr(expr)
          } else {
            None
          }
        }
        Token::LParen => {
          self.move_to_next_tok();
          left_expr = if let Some(expr) = left_expr {
            self.parse_call_expr(expr)
          } else {
            None
          }
        }
        _ => return left_expr,
      };
    }

    left_expr
  }

  fn error_no_prefix_parser(&mut self) {
    self.errors.push(ParseError::new(
      ParseErrorKind::UnexpectedToken,
      format!(
        "no prefix parse function for {:?} found",
        self.current_token
      ),
    ))
  }

  fn parse_ident_expr(&self) -> Option<ast::Expr<'input>> {
    self.parse_ident().map(ast::Expr::Ident)
  }

  fn parse_int_expr(&self) -> Option<ast::Expr<'input>> {
    match self.current_token {
      Token::Int(literal) => Some(ast::Expr::Literal(ast::Literal::Int(literal))),
      _ => None,
    }
  }

  fn parse_bool_expr(&self) -> Option<ast::Expr<'input>> {
    match self.current_token {
      Token::Bool(literal) => Some(ast::Expr::Literal(ast::Literal::Bool(literal))),
      _ => None,
    }
  }

  fn parse_prefix_expr(&mut self) -> Option<ast::Expr<'input>> {
    let prefix = match self.current_token {
      Token::Bang => ast::Prefix::Bang,
      Token::Minus => ast::Prefix::Minus,
      Token::Plus => ast::Prefix::Plus,
      _ => return None,
    };

    self.move_to_next_tok();

    self
      .parse_expr(ast::Precedence::Prefix)
      .map(|expr| ast::Expr::Prefix(prefix, Box::new(expr)))
  }

  fn parse_infix_expr(&mut self, left_expr: ast::Expr<'input>) -> Option<ast::Expr<'input>> {
    let infix = match self.current_token {
      Token::Plus => ast::Infix::Plus,
      Token::Minus => ast::Infix::Minus,
      Token::Slash => ast::Infix::Divide,
      Token::Asterisk => ast::Infix::Multiply,
      Token::Equal => ast::Infix::Equal,
      Token::NotEqual => ast::Infix::NotEqual,
      Token::LessThan => ast::Infix::LessThan,
      Token::LessThanEqual => ast::Infix::LessThanEqual,
      Token::GreaterThan => ast::Infix::GreaterThan,
      Token::GreaterThanEqual => ast::Infix::GreaterThanEqual,
      _ => return None,
    };

    let precedence = self.current_token_precedence();
    self.move_to_next_tok();
    self
      .parse_expr(precedence)
      .map(|expr| ast::Expr::Infix(Box::new(left_expr), infix, Box::new(expr)))
  }

  fn parse_grouped_expr(&mut self) -> Option<ast::Expr<'input>> {
    self.move_to_next_tok();
    let expr = self.parse_expr(ast::Precedence::Lowest);

    if self.expect_next_is(Token::RParen) {
      expr
    } else {
      None
    }
  }

  fn parse_if_expr(&mut self) -> Option<ast::Expr<'input>> {
    if !self.expect_next_is(Token::LParen) {
      return None;
    }

    self.move_to_next_tok();
    let condition = self.parse_expr(ast::Precedence::Lowest);
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
    condition.as_ref()?;

    if !self.expect_next_is(Token::RParen) {
      return None;
    }

    if !self.expect_next_is(Token::LBrace) {
      return None;
    }

    let consequence = if let Some(stmt) = self.parse_block_stmt() {
      stmt
    } else {
      vec![]
    };

    let mut alternative: Option<ast::BlockStatement<'input>> = None;
    if self.next_token_is(&Token::Else) {
      self.move_to_next_tok();

      if !self.expect_next_is(Token::LBrace) {
        self.error_next_token(Token::LBrace);
        return None;
      }

      alternative = self.parse_block_stmt();
    }

    Some(ast::Expr::If {
      condition: Box::new(condition.unwrap()),
      consequence,
      alternative,
    })
  }

  fn parse_call_expr(&mut self, func: ast::Expr<'input>) -> Option<ast::Expr<'input>> {
    let arguments = self.parse_call_args()?;
    Some(ast::Expr::Call {
      function: Box::new(func),
      arguments,
    })
  }

  fn parse_call_args(&mut self) -> Option<Vec<ast::Expr<'input>>> {
    let mut args = vec![];
    if self.next_token_is(&Token::RParen) {
      self.move_to_next_tok();
      return Some(args);
    }

    self.move_to_next_tok();
    args.push(self.parse_expr(ast::Precedence::Lowest)?);

    while self.next_token_is(&Token::Comma) {
      for _ in 0..=1 {
        self.move_to_next_tok();
      }
      args.push(self.parse_expr(ast::Precedence::Lowest)?);
    }

    if !self.expect_next_is(Token::RParen) {
      return None;
    }

    Some(args)
  }
}

impl<'input> Parser<'input> {
  fn parse_function_literal(&mut self) -> Option<ast::Expr<'input>> {
    if !self.expect_next_is(Token::LParen) {
      return None;
    }

    let params = if let Some(params) = self.parse_function_params() {
      params
    } else {
      vec![]
    };

    if !self.expect_next_is(Token::LBrace) {
      return None;
    }

    let body = if let Some(body) = self.parse_block_stmt() {
      body
    } else {
      vec![]
    };

    Some(ast::Expr::Function { params, body })
  }

  fn parse_function_params(&mut self) -> Option<Vec<ast::Ident<'input>>> {
    let mut identifiers = vec![];
    if self.next_token_is(&Token::RParen) {
      self.move_to_next_tok();
      return Some(identifiers);
    }

    if self.current_token_is(&Token::LParen) {
      self.move_to_next_tok();
    }

    if let Some(ident) = self.parse_ident() {
      identifiers.push(ident);
    }

    while self.next_token_is(&Token::Comma) {
      for _ in 0..=1 {
        self.move_to_next_tok();
      }
      if let Some(ident) = self.parse_ident() {
        identifiers.push(ident);
      }
    }

    if !self.expect_next_is(Token::RParen) {
      return None;
    }

    Some(identifiers)
  }
}

// precedence
impl Parser<'_> {
  fn token_to_precedence(&self, tok: &Token) -> ast::Precedence {
    match tok {
      Token::Equal | Token::NotEqual => ast::Precedence::Equals,
      Token::LessThan | Token::GreaterThan => ast::Precedence::LessGreater,
      Token::Plus | Token::Minus => ast::Precedence::Sum,
      Token::Slash | Token::Asterisk => ast::Precedence::Product,
      Token::LParen => ast::Precedence::Call,

      _ => ast::Precedence::Lowest,
    }
  }

  fn current_token_precedence(&self) -> ast::Precedence {
    self.token_to_precedence(&self.current_token)
  }

  fn next_token_precedence(&self) -> ast::Precedence {
    self.token_to_precedence(&self.next_token)
  }
}
