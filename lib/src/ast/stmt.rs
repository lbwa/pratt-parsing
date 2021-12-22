use super::expr;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement<'a> {
  Let(super::Ident<'a>),
  Return,
  /// It's a statement that consists solely of one expression. and not really
  /// a distinct statement, and only a wrapper.
  /// We need it because it's totally legal in Monkey to write the following
  /// code:
  /// ```monkey
  /// let x = 5; // normal statement
  /// x + 10; // expression statement
  /// ```
  /// As we see, Expr that is a kind variant of Statement, which means we can
  /// add it to the Statements slice of ast::Program
  ///
  /// [Similar implementation in rust: expression statement](https://doc.rust-lang.org/reference/statements.html#expression-statements)
  Expr(expr::Expr<'a>),
}
