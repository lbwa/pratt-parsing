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
    assert_eq!(eval!(input), Some(Object::Int(expected)))
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
    assert_eq!(eval!(input), Some(Object::Bool(expected)))
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
    assert_eq!(eval!(input), Some(Object::Bool(expected)))
  }
}

#[test]
fn eval_prefix_minus_operator() {
  let cases = vec![("-1", -1), ("-10", -10)];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Some(Object::Int(expected)))
  }
}

#[test]
fn eval_prefix_plus_operator() {
  let cases = vec![("+0", 0), ("+1", 1)];

  for (input, expected) in cases {
    assert_eq!(eval!(input), Some(Object::Int(expected)))
  }
}