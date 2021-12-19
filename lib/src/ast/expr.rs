/// An operator "in front of" its operand
#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {}

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
}
