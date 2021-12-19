use crate::ast::{Expr, Ident, Infix, Literal, Prefix, Statement as Stmt};
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

#[test]
fn prefix_expr() {
  let cases = vec![
    (
      "!5;",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Bang,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "+15;",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Plus,
        Box::new(Expr::Literal(Literal::Int(15))),
      ))],
    ),
    (
      "-15;",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Minus,
        Box::new(Expr::Literal(Literal::Int(15))),
      ))],
    ),
  ];

  for (input, expected) in cases {
    let mut parser = parser::new(lexer::new(input));
    let program = parser.parse();
    check_parse_error(&mut parser);

    assert_eq!(program, expected)
  }
}

#[test]
fn infix_expr() {
  let cases = vec![
    (
      "5 + 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::Plus,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "5 - 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::Minus,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "3 + 4 * 5 == 3 * 1 + 4 * 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(3))),
          Infix::Plus,
          Box::new(Expr::Infix(
            Box::new(Expr::Literal(Literal::Int(4))),
            Infix::Multiply,
            Box::new(Expr::Literal(Literal::Int(5))),
          )),
        )),
        Infix::Equal,
        Box::new(Expr::Infix(
          Box::new(Expr::Infix(
            Box::new(Expr::Literal(Literal::Int(3))),
            Infix::Multiply,
            Box::new(Expr::Literal(Literal::Int(1))),
          )),
          Infix::Plus,
          Box::new(Expr::Infix(
            Box::new(Expr::Literal(Literal::Int(4))),
            Infix::Multiply,
            Box::new(Expr::Literal(Literal::Int(5))),
          )),
        )),
      ))],
    ),
    (
      "3 + 4; -5 * 5",
      vec![
        Stmt::Expr(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(3))),
          Infix::Plus,
          Box::new(Expr::Literal(Literal::Int(4))),
        )),
        Stmt::Expr(Expr::Infix(
          Box::new(Expr::Prefix(
            Prefix::Minus,
            Box::new(Expr::Literal(Literal::Int(5))),
          )),
          Infix::Multiply,
          Box::new(Expr::Literal(Literal::Int(5))),
        )),
      ],
    ),
    (
      "5 > 4 == 3 < 4",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(5))),
          Infix::GreaterThan,
          Box::new(Expr::Literal(Literal::Int(4))),
        )),
        Infix::Equal,
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(3))),
          Infix::LessThan,
          Box::new(Expr::Literal(Literal::Int(4))),
        )),
      ))],
    ),
    (
      "5 < 4 != 3 > 4",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(5))),
          Infix::LessThan,
          Box::new(Expr::Literal(Literal::Int(4))),
        )),
        Infix::NotEqual,
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(3))),
          Infix::GreaterThan,
          Box::new(Expr::Literal(Literal::Int(4))),
        )),
      ))],
    ),
    (
      "-a * b",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Prefix(
          Prefix::Minus,
          Box::new(Expr::Ident(Ident("a".to_owned()))),
        )),
        Infix::Multiply,
        Box::new(Expr::Ident(Ident("b".to_owned()))),
      ))],
    ),
    (
      "!-a",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Bang,
        Box::new(Expr::Prefix(
          Prefix::Minus,
          Box::new(Expr::Ident(Ident("a".to_owned()))),
        )),
      ))],
    ),
    (
      "a + b - c",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Ident(Ident("a".to_owned()))),
          Infix::Plus,
          Box::new(Expr::Ident(Ident("b".to_owned()))),
        )),
        Infix::Minus,
        Box::new(Expr::Ident(Ident("c".to_owned()))),
      ))],
    ),
    (
      "a + b * c + d / e - f",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Infix(
            Box::new(Expr::Ident(Ident("a".to_owned()))),
            Infix::Plus,
            Box::new(Expr::Infix(
              Box::new(Expr::Ident(Ident("b".to_owned()))),
              Infix::Multiply,
              Box::new(Expr::Ident(Ident("c".to_owned()))),
            )),
          )),
          Infix::Plus,
          Box::new(Expr::Infix(
            Box::new(Expr::Ident(Ident("d".to_owned()))),
            Infix::Divide,
            Box::new(Expr::Ident(Ident("e".to_owned()))),
          )),
        )),
        Infix::Minus,
        Box::new(Expr::Ident(Ident("f".to_owned()))),
      ))],
    ),
  ];

  for (input, expected) in cases {
    let mut parser = parser::new(lexer::new(input));
    let program = parser.parse();
    check_parse_error(&mut parser);

    assert_eq!(program, expected);
  }
}
