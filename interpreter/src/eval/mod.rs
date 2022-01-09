#[cfg(test)]
mod test;

use super::object::Object;
use pratt_parsing::ast::{self, Expr, Statement};

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {}
  }

  pub fn eval(&self, stmts: Vec<ast::Statement<'_>>) -> Result<Object, String> {
    match self.eval_block_stmt(stmts) {
      Some(result) => match result {
        Object::ReturnValue(val) => Ok(*val), // unwrap value for better DX
        Object::Error(error) => Err(error),
        val => Ok(val),
      },
      None => Ok(Object::None),
    }
  }

  fn eval_stmt(&self, stmt: ast::Statement<'_>) -> Option<Object> {
    match stmt {
      ast::Statement::Expr(expr) => self.eval_expr(expr),
      ast::Statement::Return(expr) => {
        let val = self.eval_expr(expr)?;
        if Self::is_error(&val) {
          Some(val)
        } else {
          Some(Object::ReturnValue(Box::new(val)))
        }
      }
      _ => None,
    }
  }

  fn eval_block_stmt(&self, block_stmts: Vec<ast::Statement>) -> Option<Object> {
    let mut result: Option<Object> = None;
    for stmt in block_stmts {
      match self.eval_stmt(stmt)? {
        Object::ReturnValue(val) => return Some(Object::ReturnValue(val)),
        Object::Error(error) => return Some(Self::error(error)),
        obj => result = Some(obj),
      }
    }
    result
  }

  fn eval_expr(&self, expr: ast::Expr) -> Option<Object> {
    match expr {
      Expr::Literal(literal) => self.eval_literal(literal),
      Expr::Prefix(prefix, expr) => self.eval_prefix_expr(prefix, *expr),
      Expr::Infix(left, infix, right) => self.eval_infix_expr(*left, infix, *right),
      Expr::If {
        condition,
        consequence,
        alternative,
      } => self.eval_if_expr(*condition, consequence, alternative),
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

// errors
impl Evaluator {
  pub fn error(message: String) -> Object {
    Object::Error(message)
  }

  pub fn is_error(object: &Object) -> bool {
    matches!(object, Object::Error(_))
  }
}

// eval_x_expr
impl Evaluator {
  fn eval_prefix_expr(&self, prefix: ast::Prefix, expr: ast::Expr) -> Option<Object> {
    self.eval_expr(expr).map(|result| match prefix {
      ast::Prefix::Bang => match result {
        Object::Bool(false) => Object::Bool(true),
        Object::Int(val) => Object::Bool(val == 0),
        _ => Object::Bool(false),
      },

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

    if Self::is_error(&left) {
      return Some(left);
    }
    if Self::is_error(&right) {
      return Some(right);
    }

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

  fn eval_if_expr(
    &self,
    condition: ast::Expr,
    consequence: Vec<Statement>,
    alternative: Option<Vec<Statement>>,
  ) -> Option<Object> {
    match self.eval_expr(condition)? {
      Object::Bool(val) => self.eval_block_stmt(if val { consequence } else { alternative? }),
      Object::Int(val) => self.eval_block_stmt(if val != 0 { consequence } else { alternative? }),
      _ => None,
    }
  }
}
