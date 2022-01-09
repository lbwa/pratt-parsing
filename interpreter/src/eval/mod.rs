#[cfg(test)]
mod test;

use super::environment::Environment;
use super::object::Object;
use pratt_parsing::ast;

#[derive(Default)]
pub struct Evaluator<'ident> {
  environment: Environment<'ident>,
}

impl<'ident> Evaluator<'ident> {
  pub fn new() -> Self {
    Evaluator {
      environment: Environment::new(),
    }
  }

  pub fn eval(&mut self, stmts: Vec<ast::Statement<'ident>>) -> Result<Object, String> {
    match self.eval_block_stmt(stmts) {
      Some(result) => match result {
        Object::ReturnValue(val) => Ok(*val), // unwrap value for better DX
        Object::Error(error) => Err(error),
        val => Ok(val),
      },
      None => Ok(Object::None),
    }
  }

  fn eval_stmt(&mut self, stmt: ast::Statement<'ident>) -> Option<Object> {
    match stmt {
      ast::Statement::Expr(expr) => self.eval_expr(expr),
      ast::Statement::Let(ident, expr) => self.eval_let_stmt(ident, expr),
      ast::Statement::Return(expr) => self.eval_return_stmt(expr),
    }
  }

  fn eval_block_stmt(&mut self, block_stmts: Vec<ast::Statement<'ident>>) -> Option<Object> {
    let mut result: Option<Object> = None;
    for stmt in block_stmts {
      println!("stmt ---> {:?}", stmt.clone());
      match self.eval_stmt(stmt)? {
        Object::ReturnValue(val) => return Some(Object::ReturnValue(val)),
        Object::Error(error) => return Some(Self::error(error)),
        obj => result = Some(obj),
      }
    }
    result
  }

  fn eval_expr(&mut self, expr: ast::Expr<'ident>) -> Option<Object> {
    use ast::Expr;
    match expr {
      Expr::Literal(literal) => self.eval_literal(literal),
      Expr::Ident(ident) => self.eval_ident(ident),
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

  fn eval_ident(&self, ident: ast::Ident<'ident>) -> Option<Object> {
    let ast::Ident(ident) = ident;
    if let Some(value) = self.environment.get(ident) {
      Some(value)
    } else {
      Some(Self::error(format!("Identifier not found: {}", ident)))
    }
  }
}

// errors
impl Evaluator<'_> {
  fn error(message: String) -> Object {
    Object::Error(message)
  }

  fn is_error(object: &Object) -> bool {
    matches!(object, Object::Error(_))
  }
}

// eval_x_stmt
impl<'ident> Evaluator<'ident> {
  fn eval_return_stmt(&mut self, expr: ast::Expr<'ident>) -> Option<Object> {
    self.eval_expr(expr).map(|value| {
      if Self::is_error(&value) {
        value
      } else {
        Object::ReturnValue(Box::new(value))
      }
    })
  }

  fn eval_let_stmt(
    &mut self,
    ident: ast::Ident<'ident>,
    expr: ast::Expr<'ident>,
  ) -> Option<Object> {
    let value = self.eval_expr(expr)?;
    if Self::is_error(&value) {
      return Some(value);
    }

    use ast::Ident;
    let Ident(ident) = ident;
    self.environment.set(ident, value.clone());
    Some(value)
  }
}

// eval_x_expr
impl<'ident> Evaluator<'ident> {
  fn eval_prefix_expr(&mut self, prefix: ast::Prefix, expr: ast::Expr<'ident>) -> Option<Object> {
    use ast::Prefix;

    self.eval_expr(expr).map(|result| match prefix {
      Prefix::Bang => match result {
        Object::Bool(false) => Object::Bool(true),
        Object::Int(val) => Object::Bool(val == 0),
        _ => Object::Bool(false),
      },

      Prefix::Minus => {
        let result = match result {
          Object::Int(val) => Object::Int(-val),
          _ => Self::error(format!("Illegal syntax: -{}", result)),
        };
        result
      }

      Prefix::Plus => match result {
        Object::Int(_) => result,
        _ => Self::error(format!("Illegal syntax: +{}", result)),
      },
    })
  }

  fn eval_infix_expr(
    &mut self,
    left_expr: ast::Expr<'ident>,
    infix: ast::Infix,
    right_expr: ast::Expr<'ident>,
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
    &mut self,
    condition: ast::Expr<'ident>,
    consequence: Vec<ast::Statement<'ident>>,
    alternative: Option<Vec<ast::Statement<'ident>>>,
  ) -> Option<Object> {
    match self.eval_expr(condition)? {
      Object::Bool(val) => self.eval_block_stmt(if val { consequence } else { alternative? }),
      Object::Int(val) => self.eval_block_stmt(if val != 0 { consequence } else { alternative? }),
      _ => None,
    }
  }
}
