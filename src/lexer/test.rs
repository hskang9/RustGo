
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_next_token() {
  let input = "let five = 5;
  let ten = 10;
  let add = fn(x, y) {
  x + y;
  };
  let result = add(five, ten);
  !-/*5;
  5 < 10 > 5;
  ";
  print_token(input.to_string());
  }
}
