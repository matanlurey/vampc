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
  Number(f64),
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
    while let Some(char) = chars.next() {
      let token: Option<Token> = match char {
        '0'..='9' => {
          let mut number = 0.0;
          let mut current = char;
          loop {
            number *= 10.0;
            number += ((current as u8) - b'0') as f64;
            // TODO: Support a single decimal point (for floats).
            // TODO: Support numerical seperators (i.e. `_`).
            // TODO: Support different radix encodings (binary, hex).
            match chars.peek() {
              Some(&next) if '0' <= next && next <= '9' => {
                current = next;
                chars.next();
              }
              _ => break,
            }
          }
          Some(Token::Number(number))
        }
        ' ' => None,
        _ => Some(Token::Unknown(char)),
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
  fn test_scan_0() {
    let mut scanner = Scanner::new("0".to_string());
    scanner.scan();
    assert_eq!(1, scanner.output.len());
    assert_eq!(Some(&Token::Number(0.0)), scanner.output.first());
  }

  #[test]
  fn test_scan_100() {
    let mut scanner = Scanner::new("100".to_string());
    scanner.scan();
    assert_eq!(1, scanner.output.len());
    assert_eq!(Some(&Token::Number(100.0)), scanner.output.first());
  }

  #[test]
  fn test_scan_num_unknown_num() {
    let mut scanner = Scanner::new("1 22 333".to_string());
    scanner.scan();
    assert_eq!(3, scanner.output.len());
    assert_eq!(Some(&Token::Number(1.0)), scanner.output.get(0));
    assert_eq!(Some(&Token::Number(22.0)), scanner.output.get(1));
    assert_eq!(Some(&Token::Number(333.0)), scanner.output.get(2));
  }
}
