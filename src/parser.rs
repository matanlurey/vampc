mod scanner;

pub enum AST {
  Function,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(1, 2);
  }
}
