#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
  Let(Ident),
  Return,
}

pub type Program = Vec<Statement>;
