#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_identifier_expression() {
      let input = "foobar;";
      let l = lexer::Lexer::new(input.to_string());
      let mut p = parser::Parser::new(l);
      p.parse_program();

      check_parser_errors(p);
  }

  #[test]
  fn test_integer_literal_expression() {
    let input = "5;";
    let l = lexer::Lexer::new(input.to_string());
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();

    check_parser_errors(p);
    
    if program.statements.len() != 1 {
        panic!("program has not enough statements. got={:?}", program.statements.len());
    }
 
    let unwrap_statement = &program.statements[0].clone().to_any();
    let stmt = match unwrap_statement.downcast_ref::<ast::ExpressionStatement>() {
      Some(x) => {
        x.clone()
        println!("{:?}", x.clone().token_literal());
      }
      None => {
        panic!("program.statements[0] is not ast::ExpressionStatement. got {:?}", program.statements[0].type_name())
      }
    };


    let exp = match stmt.clone().expression {
      Some(x) => {
        x.clone()
      }
      None => {
        panic!("Expression is not ast::IntegerLiteral. got {:?}", stmt.expression.type_name())
      }
    }
    
    let literal = match exp.clone().unwrap().to_any().downcast_ref::<ast::IntegerLiteral>() {
      Some(x) => {
        x.clone()
      }
      None => {
        panic!("Expression is not ast::IntegerLiteral. got {:?}", stmt.expression.type_name())
      }
    };

    if literal.value != Some(5) {
        panic!("literal.value not {:?}. got={:?}", 5, literal.value); 
    }
    use crate::ast::Expression;
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