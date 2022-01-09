macro_rules! eval {
  ($input: tt) => {{
    use pratt_parsing::lexer::Lexer;
    use pratt_parsing::parser::Parser;
    let mut parser = Parser::new(Lexer::new($input));
    let parser = parser.parse();
    let evaluator = super::Evaluator::new();
    evaluator.eval(parser.get_stmts())
  }};
}

use crate::object::Object;

#[test]
fn eval_integer_expr() {
  let cases = vec![
    ("5", 5),
    ("10", 10),
    ("-5", -5),
    ("-10", -10),
    ("1 + 2 + 3 + 4", 10),
    ("1 + 2 - 3 + 4", 4),
    ("1 + (2 * 3) / 4", 2),
  ];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Ok(Object::Int(expected)))
  }
}

#[test]
fn eval_bool_expr() {
  let cases = vec![
    ("true", true),
    ("false", false),
    ("1 < 2", true),
    ("1 > 2", false),
    ("1 > 1", false),
    ("1 < 1", false),
    ("1 == 1", true),
    ("1 != 2", true),
    ("1 != 1", false),
    ("true == true", true),
    ("false == false", true),
    ("(1 < 2) == true", true),
    ("(1 < 2) == false", false),
    ("(1 > 2) == true", false),
    ("(1 > 2) == false", true),
  ];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Ok(Object::Bool(expected)))
  }
}

#[test]
fn eval_bang_operator() {
  let cases = vec![
    ("!true", false),
    ("!false", true),
    ("!!true", true),
    ("!!!true", false),
    ("!!false", false),
    ("!!!false", true),
    ("!0", true),
    ("!!0", false),
    ("!!-1", true),
  ];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Ok(Object::Bool(expected)))
  }
}

#[test]
fn eval_prefix_minus_operator() {
  let cases = vec![("-1", -1), ("-10", -10)];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Ok(Object::Int(expected)))
  }
}

#[test]
fn eval_prefix_plus_operator() {
  let cases = vec![("+0", 0), ("+1", 1)];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Ok(Object::Int(expected)))
  }
}

#[test]
fn eval_if_else_expr() {
  let cases = vec![
    ("if (true) { 1 }", Ok(Object::Int(1))),
    ("if (false) { 1 }", Ok(Object::None)),
    ("if (1) { 2 }", Ok(Object::Int(2))),
    ("if (1 < 2) { 3 }", Ok(Object::Int(3))),
    ("if (1 > 2) { 3 }", Ok(Object::None)),
    ("if (1 > 2) { 3 } else { 4 }", Ok(Object::Int(4))),
    ("if (1 < 2) { 3 } else { 4 }", Ok(Object::Int(3))),
    ("if (1 * 2 + 3 / 4 - 5) { 6 }", Ok(Object::Int(6))),
    ("if (1 * (2 + 3) / 4 - 5) { 6;\n 7 }", Ok(Object::Int(7))),
  ];

  for (input, expected) in cases {
    assert_eq!(eval!(input), expected)
  }
}

#[test]
fn eval_return_stmt() {
  let cases = vec![
    ("return 1;", 1),
    ("return 1; 2;", 1),
    ("return 6 * 7; 8", 42),
    ("99; return 7 * 8; 9;", 56),
    (
      "if (10 > 1) {
      if (10 > 1) {
        return 10;
      }
      return 1;
    }",
      10,
    ),
  ];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Ok(Object::Int(expected)))
  }
}

#[test]
fn catch_internal_error() {
  let cases = vec![
    ("1 + true;", "Couldn't perform operation: 1 + true"),
    ("1 + true; 2", "Couldn't perform operation: 1 + true"),
    ("+true", "Illegal syntax: +true"),
    ("true * false", "Couldn't perform operation: true * false"),
    (
      "
    if (1 < 2) {
      true / false;
    }",
      "Couldn't perform operation: true / false",
    ),
    (
      "
    if (1 < 2) {
      if (2 < 3) {
        return true / false;
      }
      return 1;
    }",
      "Couldn't perform operation: true / false",
    ),
  ];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Err(expected.to_string()))
  }
}
