use crate::ast::{Ident, Statement as Stmt};
use crate::lexer;
use crate::parser;

fn check_parse_error(parser: &mut parser::Parser) {
  let errors = parser.get_errors();

  if errors.len() < 1 {
    return;
  }

  println!("\nparser has {} errors.", errors.len());
  for error in errors {
    println!("parser error: {}", error);
  }
  panic!("Check parser error failed.")
}

#[test]
fn let_statements() {
  let input = "
  let x = 5;
  let y = 10;
  let foobar = 838383;
  ";

  let mut parser = parser::new(lexer::new(input));
  let program = parser.parse();
  check_parse_error(&mut parser);

  assert_eq!(
    program,
    vec![
      Stmt::Let(Ident(String::from("x"))),
      Stmt::Let(Ident(String::from("y"))),
      Stmt::Let(Ident(String::from("foobar")))
    ]
  );
}
