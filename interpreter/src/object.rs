use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum Object {
  Int(i64),
  Bool(bool),

  Error(String),
}

impl fmt::Display for Object {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Object::Int(ref val) => write!(formatter, "{}", val),
      Object::Bool(val) => write!(formatter, "{}", val),
      Object::Error(ref val) => write!(formatter, "{}", val),
    }
  }
}

impl Add for Object {
  type Output = Object;
  fn add(self, rhs: Object) -> Object {
    if let Object::Int(left_val) = self {
      if let Object::Int(right_val) = rhs {
        return Object::Int(left_val + right_val);
      }
    }
    // TODO: handle the addition for string type
    Object::Error(format!("Couldn't perform operation: {} + {}", self, rhs,))
  }
}

impl Sub for Object {
  type Output = Object;
  fn sub(self, rhs: Object) -> Object {
    if let Object::Int(left_val) = self {
      if let Object::Int(right_val) = rhs {
        return Object::Int(left_val - right_val);
      }
    }
    Object::Error(format!("Couldn't perform operation: {} - {}", self, rhs,))
  }
}

impl Mul for Object {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    if let Object::Int(left_val) = self {
      if let Object::Int(right_val) = rhs {
        return Object::Int(left_val * right_val);
      }
    }
    Object::Error(format!("Couldn't perform operation: {} * {}", self, rhs,))
  }
}

impl Div for Object {
  type Output = Self;
  fn div(self, rhs: Self) -> Self {
    if let Object::Int(left_val) = self {
      if let Object::Int(right_val) = rhs {
        return Object::Int(left_val / right_val);
      }
    }
    Object::Error(format!("Couldn't perform operation: {} / {}", self, rhs,))
  }
}
