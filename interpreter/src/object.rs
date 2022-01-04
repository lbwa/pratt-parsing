use std::fmt;

#[derive(PartialEq, Clone, Debug)]
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
