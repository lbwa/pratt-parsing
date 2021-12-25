use crate::ast::{Expr, Ident, Infix, Literal, Prefix, Statement as Stmt};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn check_parse_error(parser: &Parser) {
  let errors = parser.get_errors();

  if errors.is_empty() {
    return;
  }

  println!("\nparser has {} errors.", errors.len());
  for error in errors {
    println!("parser error: {}", error);
  }
  panic!("Check parser errors.")
}

#[test]
fn let_statements() {
  let input = "
  let x = 5;
  let y = 10;
  let foobar = 838383;
  ";

  let mut parser = Parser::new(Lexer::new(input));
  let parser = parser.parse();
  check_parse_error(parser);

  assert_eq!(
    parser.stmts,
    vec![
      Stmt::Let(Ident("x")),
      Stmt::Let(Ident("y")),
      Stmt::Let(Ident("foobar"))
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

  let mut parser = Parser::new(Lexer::new(input));
  let parser = parser.parse();
  check_parse_error(parser);

  assert_eq!(parser.stmts, vec![Stmt::Return, Stmt::Return, Stmt::Return])
}

#[test]
fn ident_expr() {
  let input = "foobar;";

  let mut parser = Parser::new(Lexer::new(input));
  let parser = parser.parse();
  check_parse_error(parser);

  assert_eq!(parser.stmts, vec![Stmt::Expr(Expr::Ident(Ident("foobar")))]);
}

#[test]
fn integer_literal_expr() {
  let input = "5;";

  let mut parser = Parser::new(Lexer::new(input));
  let parser = parser.parse();
  check_parse_error(parser);

  assert_eq!(
    parser.stmts,
    vec![Stmt::Expr(Expr::Literal(Literal::Int(5)))]
  );
}

#[test]
fn bool_literal_expr() {
  let cases = vec![
    (
      "true;false",
      vec![
        Stmt::Expr(Expr::Literal(Literal::Bool(true))),
        Stmt::Expr(Expr::Literal(Literal::Bool(false))),
      ],
    ),
    (
      "3 > 5 == false",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(3))),
          Infix::GreaterThan,
          Box::new(Expr::Literal(Literal::Int(5))),
        )),
        Infix::Equal,
        Box::new(Expr::Literal(Literal::Bool(false))),
      ))],
    ),
  ];

  for (input, expected) in cases {
    let mut parser = Parser::new(Lexer::new(input));
    let parser = parser.parse();
    check_parse_error(parser);

    assert_eq!(parser.stmts, expected);
  }
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
    let mut parser = Parser::new(Lexer::new(input));
    let parser = parser.parse();
    check_parse_error(parser);

    assert_eq!(parser.stmts, expected)
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
      "5 * 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::Multiply,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "5 / 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::Divide,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "5 > 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::GreaterThan,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "5 < 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::LessThan,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "5 == 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::Equal,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
    (
      "5 != 5",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(5))),
        Infix::NotEqual,
        Box::new(Expr::Literal(Literal::Int(5))),
      ))],
    ),
  ];

  for (input, expected) in cases {
    let mut parser = Parser::new(Lexer::new(input));
    let parser = parser.parse();
    check_parse_error(parser);

    assert_eq!(parser.stmts, expected);
  }
}

#[test]
fn operator_precedence_parsing() {
  let cases = vec![
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
          Box::new(Expr::Ident(Ident("a"))),
        )),
        Infix::Multiply,
        Box::new(Expr::Ident(Ident("b"))),
      ))],
    ),
    (
      "!-a",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Bang,
        Box::new(Expr::Prefix(
          Prefix::Minus,
          Box::new(Expr::Ident(Ident("a"))),
        )),
      ))],
    ),
    (
      "a + b - c",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Ident(Ident("a"))),
          Infix::Plus,
          Box::new(Expr::Ident(Ident("b"))),
        )),
        Infix::Minus,
        Box::new(Expr::Ident(Ident("c"))),
      ))],
    ),
    (
      "a + b * c + d / e - f",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Infix(
            Box::new(Expr::Ident(Ident("a"))),
            Infix::Plus,
            Box::new(Expr::Infix(
              Box::new(Expr::Ident(Ident("b"))),
              Infix::Multiply,
              Box::new(Expr::Ident(Ident("c"))),
            )),
          )),
          Infix::Plus,
          Box::new(Expr::Infix(
            Box::new(Expr::Ident(Ident("d"))),
            Infix::Divide,
            Box::new(Expr::Ident(Ident("e"))),
          )),
        )),
        Infix::Minus,
        Box::new(Expr::Ident(Ident("f"))),
      ))],
    ),
    (
      "1 + (2 + 3) + 4",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(1))),
          Infix::Plus,
          Box::new(Expr::Infix(
            Box::new(Expr::Literal(Literal::Int(2))),
            Infix::Plus,
            Box::new(Expr::Literal(Literal::Int(3))),
          )),
        )),
        Infix::Plus,
        Box::new(Expr::Literal(Literal::Int(4))),
      ))],
    ),
    (
      "(5 + 5) * 2",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(5))),
          Infix::Plus,
          Box::new(Expr::Literal(Literal::Int(5))),
        )),
        Infix::Multiply,
        Box::new(Expr::Literal(Literal::Int(2))),
      ))],
    ),
    (
      "2 / (5 + 5)",
      vec![Stmt::Expr(Expr::Infix(
        Box::new(Expr::Literal(Literal::Int(2))),
        Infix::Divide,
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(5))),
          Infix::Plus,
          Box::new(Expr::Literal(Literal::Int(5))),
        )),
      ))],
    ),
    (
      "-(5 + 5)",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Minus,
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Int(5))),
          Infix::Plus,
          Box::new(Expr::Literal(Literal::Int(5))),
        )),
      ))],
    ),
    (
      "!(true == true)",
      vec![Stmt::Expr(Expr::Prefix(
        Prefix::Bang,
        Box::new(Expr::Infix(
          Box::new(Expr::Literal(Literal::Bool(true))),
          Infix::Equal,
          Box::new(Expr::Literal(Literal::Bool(true))),
        )),
      ))],
    ),
  ];

  for (input, expected) in cases {
    let mut parser = Parser::new(Lexer::new(input));
    let parser = parser.parse();
    check_parse_error(parser);

    assert_eq!(parser.stmts, expected);
  }
}

#[test]
fn if_expr() {
  let cases = vec![
    (
      "if (x < y) { x }",
      vec![Stmt::Expr(Expr::If {
        condition: Box::new(Expr::Infix(
          Box::new(Expr::Ident(Ident("x"))),
          Infix::LessThan,
          Box::new(Expr::Ident(Ident("y"))),
        )),
        consequence: vec![Stmt::Expr(Expr::Ident(Ident("x")))],
        alternative: None,
      })],
    ),
    (
      "if (x < y) { x } else { y }",
      vec![Stmt::Expr(Expr::If {
        condition: Box::new(Expr::Infix(
          Box::new(Expr::Ident(Ident("x"))),
          Infix::LessThan,
          Box::new(Expr::Ident(Ident("y"))),
        )),
        consequence: vec![Stmt::Expr(Expr::Ident(Ident("x")))],
        alternative: Some(vec![Stmt::Expr(Expr::Ident(Ident("y")))]),
      })],
    ),
  ];

  for (input, expected) in cases {
    let mut parser = Parser::new(Lexer::new(input));
    let parser = parser.parse();
    check_parse_error(parser);

    assert_eq!(parser.stmts, expected)
  }
}
