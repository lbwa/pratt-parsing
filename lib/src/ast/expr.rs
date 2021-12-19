use std::fmt;

/// An operator "in front of" its operand. Any expression can follow a prefix
/// operator as operand.
///
/// `<prefix operator><expression>`
#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
  Bang,
  Minus,
  Plus,
}

impl fmt::Display for Prefix {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let literal = match *self {
      Prefix::Plus => "+",
      Prefix::Minus => "-",
      Prefix::Bang => "!",
    };
    write!(formatter, "{}", literal)
  }
}

/// An infix operator sits between its operands, and appear in binary
/// expressions - where the operator has two operands.
#[derive(PartialEq, Debug, Clone)]
pub enum Infix {}

/// An operator "after" its operand
#[derive(PartialEq, Debug, Clone)]
pub enum Postfix {}

#[derive(PartialEq, Debug, Clone)]
pub enum Precedence {
  Lowest,
  /// ==
  Equals,
  /// > or <
  LessGreater,
  /// +
  Sum,
  /// *
  Product,
  /// -a or !a
  Prefix,
  /// foo()
  Call,
  /// slice[index]
  Index,
}

/// A literal is an expression consisting of a single token, rather than a
/// sequence of tokens, that immediately and directly denotes the value it
/// evaluates to, rather than referring to it by name or some other evaluation
/// rule.
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
  Int(i64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
  Ident(super::Ident),
  /// Literal expression directly describes a number, character, string or
  /// boolean value.
  Literal(super::Literal),
  /// `<prefix operator><expression>`
  Prefix(Prefix, Box<Expr>),
}
