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

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
  /// Identifier type in an expression
  Ident(super::Ident),
}
