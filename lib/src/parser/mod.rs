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
pub struct Parser<'a> {
  /// An instance of the lexer, on which we repeatedly call next_token() to get
  /// the next token in the input.
  lexer: Lexer<'a>,
  // The last of tokens we have read.
  current_token: Token<'a>,
  /// next token, to decide whether we are at the end of the line or if we are
  /// at just the start of an arithmetic expression.
  next_token: Token<'a>,
  errors: ParseErrors,
}

trait ParseToken {
  /// It' s used to move pointer to next token, and usually work with `self.parse_*` methods.
  fn move_to_next_tok(&mut self);
  fn error_next_token(&mut self, tok: Token);

  fn expect_next_is(&mut self, tok: Token) -> bool;
  fn current_token_is(&self, tok: Token) -> bool;
  fn next_token_is(&self, tok: &Token) -> bool;

  fn parse_ident(&self) -> Option<ast::Ident>;
}

trait ParseStmt {
  fn parse_stmt(&mut self) -> Option<ast::Statement>;

  fn parse_let_stmt(&mut self) -> Option<ast::Statement>;
  fn parse_return_stmt(&mut self) -> Option<ast::Statement>;
  fn parse_expr_stmt(&mut self) -> Option<ast::Statement>;
}

trait ParseExpr {
  fn parse_expr(&mut self, precedence: ast::Precedence) -> Option<ast::Expr>;
  fn error_no_prefix_parser(&mut self);

  fn parse_ident_expr(&self) -> Option<ast::Expr>;
  fn parse_int_expr(&self) -> Option<ast::Expr>;
  fn parse_bool_expr(&self) -> Option<ast::Expr>;
  fn parse_prefix_expr(&mut self) -> Option<ast::Expr>;
  fn parse_infix_expr(&mut self, left_expr: ast::Expr) -> Option<ast::Expr>;
}

trait Precedence {
  fn token_to_precedence(&self, tok: &Token) -> ast::Precedence;
  fn current_token_precedence(&self) -> ast::Precedence;
  fn next_token_precedence(&self) -> ast::Precedence;
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

impl Parser<'_> {
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

  fn get_errors(&self) -> ParseErrors {
    self.errors.clone()
  }
}

impl ParseToken for Parser<'_> {
  fn move_to_next_tok(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.move_to_next_tok();
  }

  fn expect_next_is(&mut self, tok: Token) -> bool {
    if self.next_token_is(&tok) {
      self.move_to_next_tok();
      true
    } else {
      self.error_next_token(tok);
      false
    }
  }

  fn current_token_is(&self, tok: Token) -> bool {
    self.current_token == tok
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

  fn parse_ident(&self) -> Option<ast::Ident> {
    match &self.current_token {
      Token::Ident(ident) => Some(ast::Ident((*ident).to_owned())),
      _ => None,
    }
  }
}

impl ParseStmt for Parser<'_> {
  fn parse_stmt(&mut self) -> Option<ast::Statement> {
    match self.current_token {
      Token::Let => self.parse_let_stmt(),
      Token::Return => self.parse_return_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  fn parse_let_stmt(&mut self) -> Option<ast::Statement> {
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

  fn parse_return_stmt(&mut self) -> Option<ast::Statement> {
    // TODO: we're skipping the expression until we encounter a semicolon.
    while !self.current_token_is(Token::Semicolon) {
      self.move_to_next_tok();
    }
    Some(ast::Statement::Return)
  }

  fn parse_expr_stmt(&mut self) -> Option<ast::Statement> {
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
}

impl ParseExpr for Parser<'_> {
  fn parse_expr(&mut self, precedence: ast::Precedence) -> Option<ast::Expr> {
    let mut left_expr = match self.current_token {
      Token::Ident(_) => self.parse_ident_expr(),
      Token::Int(_) => self.parse_int_expr(),
      Token::Minus | Token::Plus | Token::Bang => self.parse_prefix_expr(),
      Token::Bool(_) => self.parse_bool_expr(),
      _ => {
        // unexpected token type
        self.error_no_prefix_parser();
        None
      }
    };

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
          left_expr = self.parse_infix_expr(left_expr.unwrap());
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

  fn parse_ident_expr(&self) -> Option<ast::Expr> {
    self.parse_ident().map(ast::Expr::Ident)
  }

  fn parse_int_expr(&self) -> Option<ast::Expr> {
    match self.current_token {
      Token::Int(literal) => Some(ast::Expr::Literal(ast::Literal::Int(literal))),
      _ => None,
    }
  }

  fn parse_bool_expr(&self) -> Option<ast::Expr> {
    match self.current_token {
      Token::Bool(literal) => Some(ast::Expr::Literal(ast::Literal::Bool(literal == true))),
      _ => None,
    }
  }

  fn parse_prefix_expr(&mut self) -> Option<ast::Expr> {
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

  fn parse_infix_expr(&mut self, left_expr: ast::Expr) -> Option<ast::Expr> {
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
}

impl Precedence for Parser<'_> {
  fn token_to_precedence(&self, tok: &Token) -> ast::Precedence {
    match tok {
      Token::Equal | Token::NotEqual => ast::Precedence::Equals,
      Token::LessThan | Token::GreaterThan => ast::Precedence::LessGreater,
      Token::Plus | Token::Minus => ast::Precedence::Sum,
      Token::Slash | Token::Asterisk => ast::Precedence::Product,

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
