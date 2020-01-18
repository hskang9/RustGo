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
  let reader = Interface::new("monkey")?;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_identifier_expression() {
      let input = "foobar;";
      let l = lexer::Lexer::new(input.to_string());
      let p = parser::Parser::new(l);
      
    check_parser_errors(p);
  }

  #[test]
  fn test_integer_literal_expression() {
    use crate::ast::Statement;
    use crate::ast::Expression;
    let input = "5;";
    let l = lexer::Lexer::new(input.to_string());
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();

    check_parser_errors(p);
    
    if program.statements.len() != 1 {
        panic!("program has not enough statements. got={:?}", program.statements.len());
    }
 
    let stmt: ast::ExpressionStatement;
    if let Ok(ok) = program.statements[0].to_any().downcast::<ast::ExpressionStatement>() {
      stmt = *ok;
    } else {
      panic!("program.statements[0] is not ast::ExpressionStatement. got {:?}", program.statements[0].token_literal());
    }

    let literal: ast::IntegerLiteral;
    if let Ok(ok) = stmt.expression.unwrap().to_any().downcast::<ast::IntegerLiteral>() {
      literal = *ok;
    } else {
        panic!("Expression is not ast::IntegerLiteral. got {:?}", "None");
    }
   

    if literal.value != Some(5) {
        panic!("literal.value not {:?}. got={:?}", 5, literal.value); 
    }
   
    if literal.token_literal() != "5" {
        panic!("literal.token_literal() not {:?}. got={:?}", "5", literal.token_literal()); 
    }
  
  }

  fn check_parser_errors(p: parser::Parser) {
    let errors = p.errors();
    if errors.len() == 0 {
      return;
    }

    println!("parser has {:?} errors", errors.len());
    for msg in errors {
      println!("{}", format!("parser error: {:?}", msg).red());
    }
    panic!("errors");
  }
}