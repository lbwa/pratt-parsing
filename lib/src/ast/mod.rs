mod expr;
mod stmt;

pub use self::expr::*;
pub use self::stmt::*;

/// It represents a kind of AST node, unlike `crate::token::Token::Ident(String)` which represents a kind of Token type.
#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

pub type Program = Vec<Statement>;
