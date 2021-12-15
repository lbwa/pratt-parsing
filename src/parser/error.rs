use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
  UnexpectedToken,
}

// implement fmt.Display trait for ParseErrorKind type
impl fmt::Display for ParseErrorKind {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParseErrorKind::UnexpectedToken => write!(formatter, "Unexpected Token"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ParseError {
  kind: ParseErrorKind,
  msg: String,
}

impl ParseError {
  pub fn new(kind: ParseErrorKind, msg: String) -> Self {
    ParseError { kind, msg }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}: {}", self.kind, self.msg)
  }
}

pub type ParseErrors = Vec<ParseError>;
