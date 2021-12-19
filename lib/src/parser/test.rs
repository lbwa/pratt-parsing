use crate::ast::{Expr, Ident, Literal, Statement as Stmt};
use crate::lexer;
use crate::parser;

fn check_parse_error(parser: &mut parser::Parser) {
  let errors = parser.get_errors();

  if errors.is_empty() {
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

#[test]
fn return_statements() {
  let input = "
  return 5;
  return 10;
  return 993322;
  ";

  let mut parser = parser::new(lexer::new(input));
  let program = parser.parse();
  check_parse_error(&mut parser);

  assert_eq!(program, vec![Stmt::Return, Stmt::Return, Stmt::Return])
}

#[test]
fn ident_expr() {
  let input = "foobar;";

  let mut parser = parser::new(lexer::new(input));
  let program = parser.parse();
  check_parse_error(&mut parser);

  assert_eq!(
    program,
    vec![Stmt::Expr(Expr::Ident(Ident("foobar".to_string())))]
  );
}

#[test]
fn integer_literal_expr() {
  let input = "5;";

  let mut parser = parser::new(lexer::new(input));
  let program = parser.parse();
  check_parse_error(&mut parser);

  assert_eq!(program, vec![Stmt::Expr(Expr::Literal(Literal::Int(5)))]);
}
