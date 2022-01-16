use super::expr;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement<'ident> {
  Let(super::Ident<'ident>, super::Expr<'ident>),
  Return(super::Expr<'ident>),
  /// It's a statement that consists solely of one expression. and not really
  /// a distinct statement, and only a wrapper.
  /// We need it because it's totally legal in Monkey to write the following
  /// code:
  /// ```ignore
  /// let x = 5; // normal statement
  /// x + 10; // expression statement
  /// ```
  /// As we see, Expr that is a kind variant of Statement, which means we can
  /// add it to the Statements slice of ast::Program
  ///
  /// [Similar implementation in rust: expression statement](https://doc.rust-lang.org/reference/statements.html#expression-statements)
  Expr(expr::Expr<'ident>),
}

pub type BlockStatement<'ident> = Vec<Statement<'ident>>;
