#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use colored::Colorize;
use linefeed::{Interface, ReadResult};
use std::io;

// Now a REPL for lexer
// TODO: add parser and AST to make interpreter
fn main() -> io::Result<()> {
  let reader = Interface::new("namu")?;

  reader.set_prompt(">> ")?;

  while let ReadResult::Input(input) = reader.read_line()? {
    println!("got input {:?}", input.clone());
    print_token(input);
  }

  println!("Goodbye.");

  Ok(())
}

pub fn print_token(input: String) {
  let mut lexer = lexer::Lexer::new(input.clone());
  for _i in 0..input.len() {
    if lexer.read_position > input.len() {
      break;
    } else {
      let next_token = lexer.next_token();
      println!("{:?} {:?}", lexer, next_token);
    }
  }
}

