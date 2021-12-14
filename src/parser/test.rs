use crate::ast::{Ident, Statement as Stmt};
use crate::lexer;
use crate::parser;

#[test]
fn let_statements() {
  let input = "
  let x = 5;
  let y = 10;
  let foobar = 838383;
  ";

  let mut parser = parser::new(lexer::new(input));
  let program = parser.parse();

  assert_eq!(
    program,
    vec![
      Stmt::Let(Ident(String::from("x"))),
      Stmt::Let(Ident(String::from("y"))),
      Stmt::Let(Ident(String::from("foobar")))
    ]
  );
}
