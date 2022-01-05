use monkey_interpreter::eval;
use pratt_parsing::lexer::Lexer;
use pratt_parsing::parser::Parser;
use rustyline::error::ReadlineError as ReadLineError;
use rustyline::Editor;

fn main() {
  let mut rl = Editor::<()>::new();

  loop {
    match rl.readline(">> ") {
      Ok(line) => {
        rl.add_history_entry(&line);

        let mut parser = Parser::new(Lexer::new(line.as_str()));
        let parser = parser.parse();

        for error in parser.get_errors() {
          println!("{}", error);
        }

        let evaluated = eval::Evaluator::new().eval(parser.get_stmts());
        println!("{}", evaluated.unwrap());
      }
      Err(ReadLineError::Interrupted) => {
        break;
      }
      Err(ReadLineError::Eof) => {
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
      }
    }
  }
}
