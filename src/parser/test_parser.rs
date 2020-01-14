#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_let_statements() {
    let input = "let x = 5;
        let y = 10;
        let foobar = 838383;
        return ;
        ";

    let l = lexer::Lexer::new(input.to_string());
    let mut p = parser::Parser::new(l);

    let program = p.parse_program();
    check_parser_errors(p);
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