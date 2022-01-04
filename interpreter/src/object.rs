use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
  Int(i64),
  Bool(bool),
}

impl fmt::Display for Object {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let val = match *self {
      Object::Int(val) => val.to_string(),
      Object::Bool(val) => val.to_string(),
    };
    write!(formatter, "{}", val)
  }
}
