#[cfg(test)]
mod test;

use super::object::Object;
use monkey_rust::ast::{self, Expr};

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {}
  }

  pub fn eval(&self, stmts: Vec<ast::Statement<'_>>) -> Option<Object> {
    let mut result: Option<Object> = None;
    for stmt in stmts {
      result = self.eval_stmt(stmt);
    }
    result
  }

  fn eval_stmt(&self, stmt: ast::Statement<'_>) -> Option<Object> {
    match stmt {
      ast::Statement::Expr(expr) => self.eval_expr(expr),
      _ => None,
    }
  }

  fn eval_expr(&self, expr: ast::Expr) -> Option<Object> {
    match expr {
      Expr::Literal(literal) => self.eval_literal(literal),
      _ => None,
    }
  }

  fn eval_literal(&self, literal: ast::Literal) -> Option<Object> {
    match literal {
      ast::Literal::Int(val) => Some(Object::Int(val)),
      ast::Literal::Bool(val) => Some(Object::Bool(val)),
    }
  }
}
