#[cfg(test)]
mod test;

use std::borrow::Borrow;

use super::object::Object;
use pratt_parsing::ast::{self, Expr};

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {}
  }

  pub fn error(message: String) -> Object {
    Object::Error(message)
  }

  pub fn eval(&self, stmts: Vec<ast::Statement<'_>>) -> Option<Object> {
    let mut result: Option<Object> = None;
    for stmt in stmts {
      match self.eval_stmt(stmt) {
        Some(Object::Error(message)) => return Some(Object::Error(message)),
        object => result = object,
      }
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
      Expr::Prefix(prefix, expr) => self.eval_prefix_expr(prefix, expr),
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

// eval_x_expr
impl Evaluator {
  fn eval_prefix_expr(&self, prefix: ast::Prefix, expr: Box<ast::Expr>) -> Option<Object> {
    let expr = (expr.borrow() as &ast::Expr).clone();
    if let Some(result) = self.eval_expr(expr) {
      match prefix {
        ast::Prefix::Bang => {
          let result = match result {
            Object::Bool(false) => Object::Bool(true),
            Object::Int(val) => {
              if val == 0 {
                Object::Bool(true) // !0 should be treated as false
              } else {
                Object::Bool(false)
              }
            }
            _ => Object::Bool(false),
          };
          Some(result)
        }

        ast::Prefix::Minus => {
          let result = match result {
            Object::Int(val) => Object::Int(-val),
            _ => Self::error(format!("Illegal syntax: -{}", result)),
          };
          Some(result)
        }

        ast::Prefix::Plus => match result {
          Object::Int(_) => Some(result),
          _ => Some(Self::error(format!("Illegal syntax: +{}", result))),
        },
      }
    } else {
      None
    }
  }
}
