#[cfg(test)]
mod test;

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
      Expr::Prefix(prefix, expr) => self.eval_prefix_expr(prefix, *expr),
      Expr::Infix(left, infix, right) => self.eval_infix_expr(*left, infix, *right),
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
  fn eval_prefix_expr(&self, prefix: ast::Prefix, expr: ast::Expr) -> Option<Object> {
    self.eval_expr(expr).map(|result| {
      match prefix {
        ast::Prefix::Bang => {
          match result {
            Object::Bool(false) => Object::Bool(true),
            Object::Int(val) => {
              if val == 0 {
                Object::Bool(true) // !0 should be treated as true
              } else {
                Object::Bool(false)
              }
            }
            _ => Object::Bool(false),
          }
        }

        ast::Prefix::Minus => {
          let result = match result {
            Object::Int(val) => Object::Int(-val),
            _ => Self::error(format!("Illegal syntax: -{}", result)),
          };
          result
        }

        ast::Prefix::Plus => match result {
          Object::Int(_) => result,
          _ => Self::error(format!("Illegal syntax: +{}", result)),
        },
      }
    })
  }

  fn eval_infix_expr(
    &self,
    left_expr: ast::Expr,
    infix: ast::Infix,
    right_expr: ast::Expr,
  ) -> Option<Object> {
    let left = self.eval_expr(left_expr)?;
    let right = self.eval_expr(right_expr)?;

    use ast::Infix;
    let result = match infix {
      Infix::Equal => Object::Bool(left == right),
      Infix::NotEqual => Object::Bool(left != right),
      Infix::LessThan => Object::Bool(left < right),
      Infix::LessThanEqual => Object::Bool(left <= right),
      Infix::GreaterThan => Object::Bool(left > right),
      Infix::GreaterThanEqual => Object::Bool(left >= right),
      Infix::Plus => left + right,
      Infix::Minus => left - right,
      Infix::Multiply => left * right,
      Infix::Divide => left / right,
    };
    Some(result)
  }
}
