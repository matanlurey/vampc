use std::env;
use std::fs;

/// Entrypoint into the compiler/interpreter/virtual machine, what have it.
///
/// # Usage
///
/// ```sh
/// vampc <script>
/// ```
fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() - 1 {
    0 => {
      println!("REPL not yet implemented");
      std::process::exit(1);
    }
    1 => {
      let file = &args[1];
      load_and_run(file);
    }
    _ => {
      println!("Usage: vampc <script>");
      std::process::exit(1);
    }
  }
}

/// Parses and runs a program from source.
///
/// Currently only a single file program is supported.
fn run_source(source: String) {
  let mut scanner = Scanner::new(source);
  scanner.scan();
  for token in scanner.output {
    println!("{:?}", token);
  }
}

/// Loads a file and passes it to `run_source`.
fn load_and_run(input: &str) {
  let result = fs::read_to_string(input);
  let contents = result.expect("Could not read file");
  run_source(contents);
}

/// Represents an architecture for parsing text into symbols (tokens).
struct Scanner {
  /// Being parsed.
  input: String,

  /// Token output.
  output: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
  /// Represents a named identifier or keyword.
  Name(String),

  /// Numeric literal.
  Numeric(String),

  /// Unknown (non-whitespace).
  Unknown(char),
}

impl Scanner {
  fn new(input: String) -> Scanner {
    Scanner {
      input,
      output: Vec::new(),
    }
  }

  fn scan(&mut self) {
    let mut chars = self.input.chars().peekable();
    while let Some(next) = chars.next() {
      let token: Option<Token> = match next {
        // Identifier or Keywords.
        'a'..='z' | 'A'..='Z' => {
          let mut name = String::from("");
          let mut current = next;
          loop {
            name.push(current);
            let peek = chars.peek();
            match peek {
              Some('a'..='z') | Some('A'..='Z') => {
                current = peek.unwrap().to_owned();
                chars.next();
              }
              _ => break,
            }
          }
          Some(Token::Name(name))
        }

        // Numerical literals.
        '0'..='9' => {
          let mut number = String::from("");
          let mut current = next;
          let mut is_float = false;
          loop {
            number.push(current);
            // TODO: Support numerical seperators (i.e. `_`).
            // TODO: Support different radix encodings (binary, hex).
            let peek = chars.peek();
            match peek {
              Some('0'..='9') => {
                current = peek.unwrap().to_owned();
                chars.next();
              }
              Some('.') => {
                if is_float {
                  break;
                } else {
                  current = peek.unwrap().to_owned();
                  chars.next();
                  is_float = true;
                }
              }
              _ => break,
            }
          }
          Some(Token::Numeric(number))
        }
        // Whitespace (Ignore).
        ' ' | '\n' => None,
        // Unsupported.
        _ => Some(Token::Unknown(next)),
      };
      if token.is_some() {
        self.output.push(token.unwrap());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scan_int_0() {
    let mut scanner = Scanner::new("0".to_string());
    scanner.scan();
    assert_eq!(1, scanner.output.len());
    assert_eq!(
      Some(&Token::Numeric(String::from("0"))),
      scanner.output.first()
    );
  }

  #[test]
  fn test_scan_int_100() {
    let mut scanner = Scanner::new("100".to_string());
    scanner.scan();
    assert_eq!(1, scanner.output.len());
    assert_eq!(
      Some(&Token::Numeric(String::from("100"))),
      scanner.output.first()
    );
  }

  #[test]
  fn test_scan_multiple_ints() {
    let mut scanner = Scanner::new("10 25 303".to_string());
    scanner.scan();
    assert_eq!(3, scanner.output.len());
    assert_eq!(
      Some(&Token::Numeric(String::from("10"))),
      scanner.output.get(0)
    );
    assert_eq!(
      Some(&Token::Numeric(String::from("25"))),
      scanner.output.get(1)
    );
    assert_eq!(
      Some(&Token::Numeric(String::from("303"))),
      scanner.output.get(2)
    );
  }

  #[test]
  fn test_scan_float() {
    let mut scanner = Scanner::new("3.14".to_string());
    scanner.scan();
    assert_eq!(1, scanner.output.len());
    assert_eq!(
      Some(&Token::Numeric(String::from("3.14"))),
      scanner.output.first()
    );
  }

  #[test]
  fn test_scan_multiple_floats() {
    let mut scanner = Scanner::new("1.23 2.50 3.03".to_string());
    scanner.scan();
    assert_eq!(3, scanner.output.len());
    assert_eq!(
      Some(&Token::Numeric(String::from("1.23"))),
      scanner.output.get(0)
    );
    assert_eq!(
      Some(&Token::Numeric(String::from("2.50"))),
      scanner.output.get(1)
    );
    assert_eq!(
      Some(&Token::Numeric(String::from("3.03"))),
      scanner.output.get(2)
    );
  }

  #[test]
  fn test_scan_invalid_float() {
    let mut scanner = Scanner::new("1.2.3".to_string());
    scanner.scan();
    assert_eq!(3, scanner.output.len());
    assert_eq!(
      Some(&Token::Numeric(String::from("1.2"))),
      scanner.output.get(0)
    );
    assert_eq!(Some(&Token::Unknown('.')), scanner.output.get(1));
    assert_eq!(
      Some(&Token::Numeric(String::from("3"))),
      scanner.output.get(2)
    );
  }

  #[test]
  fn test_scan_name() {
    let mut scanner = Scanner::new("foo".to_string());
    scanner.scan();
    assert_eq!(1, scanner.output.len());
    assert_eq!(
      Some(&Token::Name(String::from("foo"))),
      scanner.output.first()
    );
  }

  #[test]
  fn test_scan_multiple_names() {
    let mut scanner = Scanner::new("foo bar baz".to_string());
    scanner.scan();
    assert_eq!(3, scanner.output.len());
    assert_eq!(
      Some(&Token::Name(String::from("foo"))),
      scanner.output.get(0)
    );
    assert_eq!(
      Some(&Token::Name(String::from("bar"))),
      scanner.output.get(1)
    );
    assert_eq!(
      Some(&Token::Name(String::from("baz"))),
      scanner.output.get(2)
    );
  }
}
