#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_identifier_expression() {
      let input = "foobar;";
      let l = lexer::Lexer::new(input.to_string());
      let p = parser::Parser::new(l);
      p.parse_program();

      check_parser_errors(p);
  }

  #[test]
  fn test_integer_literal_expression() {
    let input = "5;";
    let l = lexer::Lexer::new(input.to_string());
    let p = parser::Parser::new(l);
    let program = p.parse_program();

    check_parser_errors(p);

    if program.statements.len() != 1 {
        panic!("program has ot enough statements. got={:?}", len(program.statements));
    }
    
    println!("{:?}", program.statements);
    let stmt = match program.statements[1].type_name() {
        some(x) if type_of(x) == "ast::ExpressionStatement":  => stmt,
        _ => panic!("program.statements[0] is not ast::ExpressionStatement. got {:?}", program.statements[0]);
    };

    


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