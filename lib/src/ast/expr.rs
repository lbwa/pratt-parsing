use super::BlockStatement;
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

/// A literal is an expression consisting of a single token, rather than a
/// sequence of tokens, that immediately and directly denotes the value it
/// evaluates to, rather than referring to it by name or some other evaluation
/// rule.
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
  Int(i64),
  Bool(bool),
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
pub enum Infix {
  Plus,
  Minus,
  Multiply,
  Divide,
  GreaterThan,
  LessThan,
  Equal,
  NotEqual,
  GreaterThanEqual,
  LessThanEqual,
}

impl fmt::Display for Infix {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let literal = match *self {
      Infix::Plus => "+",
      Infix::Minus => "-",
      Infix::Multiply => "×",
      Infix::Divide => "÷",
      Infix::GreaterThan => ">",
      Infix::LessThan => "<",
      Infix::Equal => "=",
      Infix::NotEqual => "≠",
      Infix::GreaterThanEqual => "≥",
      Infix::LessThanEqual => "≤",
    };
    write!(formatter, "{}", literal)
  }
}

/// An operator "after" its operand
#[derive(PartialEq, Debug, Clone)]
pub enum Postfix {}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
  Lowest,
  /// ==
  Equals,
  /// `>` or `<`
  LessGreater,
  /// `+` or `-`
  Sum,
  /// `*` or `/`
  Product,
  /// -a or !a
  Prefix,
  /// foo()
  Call,
  /// slice[index]
  Index,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr<'a> {
  Ident(super::Ident<'a>),
  /// Literal expression directly describes a number, character, string or
  /// boolean value.
  Literal(super::Literal),
  /// `<prefix operator><expression>`
  Prefix(Prefix, Box<Expr<'a>>),
  /// `<expression><infix operator><expression>`
  Infix(Box<Expr<'a>>, Infix, Box<Expr<'a>>),
  /// `if (<condition>) <consequence> else <alternative>`
  If {
    condition: Box<Expr<'a>>,
    consequence: BlockStatement<'a>,
    alternative: Option<BlockStatement<'a>>,
  },
  /// `fn <parameters> <block statements>`
  Function {
    params: Vec<super::Ident<'a>>,
    body: BlockStatement<'a>,
  },
  /// `<expression>(<comma separated expressions>)`
  Call {
    function: Box<Expr<'a>>,
    // `Vec<T>` is already on the heap, `Box<Vec<T>>` makes an extra allocation
    // https://rust-lang.github.io/rust-clippy/master/index.html#box_collection
    arguments: Vec<Expr<'a>>,
  },
}
