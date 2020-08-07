use std::iter;

/// Represents an architecture for parsing text into symbols (tokens).
pub struct Scanner {
  /// Being parsed.
  pub input: String,

  /// Token output.
  pub output: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
  /// Represents a single-line comment.
  Comment(String),

  /// Represents a named identifier or keyword.
  Name(String),

  /// Numeric literal.
  Numeric(String),

  /// Represents a pairing of symbols.
  Pair(PairSymbol, PairType),

  /// Represents a string literal.
  String(String),

  /// Unknown (non-whitespace).
  Unknown(char),
}

#[derive(Debug, PartialEq)]
pub enum PairSymbol {
  /// `{` or `}`.
  CurlyBracket,

  /// `(` or `)`.
  Parentheses,
}

#[derive(Debug, PartialEq)]
pub enum PairType {
  /// `{` or `(`.
  Open,

  /// `}` or `)`.
  Close,
}

impl Scanner {
  pub fn new(input: String) -> Scanner {
    Scanner {
      input,
      output: Vec::new(),
    }
  }

  pub fn scan(&mut self) {
    let mut chars = self.input.chars().peekable();
    while let Some(next) = chars.next() {
      let token: Option<Token> = match next {
        // Identifier or Keywords.
        'a'..='z' | 'A'..='Z' => Scanner::scan_name(&mut chars, next),

        // Numerical literals.
        '0'..='9' => Scanner::scan_number(&mut chars, next),

        // String literals.
        '\'' => Scanner::scan_string(&mut chars),

        // Pairings.
        '(' => Some(Token::Pair(PairSymbol::Parentheses, PairType::Open)),
        ')' => Some(Token::Pair(PairSymbol::Parentheses, PairType::Close)),
        '{' => Some(Token::Pair(PairSymbol::CurlyBracket, PairType::Open)),
        '}' => Some(Token::Pair(PairSymbol::CurlyBracket, PairType::Close)),

        // Comments.
        '/' => Scanner::scan_comment(&mut chars, next),

        // Whitespace (Ignore).
        ' ' | '\n' => None,

        // Unsupported.
        _ => Some(Token::Unknown(next)),
      };
      if let Some(token) = token {
        self.output.push(token);
      }
    }
  }

  fn scan_comment<T: Iterator<Item = char>>(
    chars: &mut iter::Peekable<T>,
    next: char,
  ) -> Option<Token> {
    match chars.peek() {
      Some('/') => {
        chars.next();
        let mut comment = String::from("");
        loop {
          let peek = chars.next();
          match peek {
            Some('\n') | None => break,
            _ => comment.push(peek.unwrap().to_owned()),
          }
        }
        Some(Token::Comment(comment))
      }
      _ => Some(Token::Unknown(next)),
    }
  }

  fn scan_name<T: Iterator<Item = char>>(
    chars: &mut iter::Peekable<T>,
    next: char,
  ) -> Option<Token> {
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

  fn scan_number<T: Iterator<Item = char>>(
    chars: &mut iter::Peekable<T>,
    next: char,
  ) -> Option<Token> {
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

  fn scan_string<T: Iterator<Item = char>>(
    chars: &mut iter::Peekable<T>,
  ) -> Option<Token> {
    let mut literal = String::from("");
    loop {
      let peek = chars.next();
      match peek {
        Some('\'') => {
          chars.next();
          break;
        }
        Some('\n') | None => break,
        _ => {
          literal.push(peek.unwrap().to_owned());
        }
      }
    }
    Some(Token::String(literal))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_tokens(input: &str, tokens: &[Token]) {
    let mut scanner = Scanner::new(input.to_string());
    scanner.scan();
    assert_eq!(tokens.len(), scanner.output.len());
    for (i, token) in tokens.iter().enumerate() {
      assert_eq!(token, scanner.output.get(i).unwrap());
    }
  }

  #[test]
  fn test_scan_int_0() {
    assert_tokens("0", &[Token::Numeric(String::from("0"))]);
  }

  #[test]
  fn test_scan_int_100() {
    assert_tokens("100", &[Token::Numeric(String::from("100"))]);
  }

  #[test]
  fn test_scan_multiple_ints() {
    assert_tokens(
      "10 25 303",
      &[
        Token::Numeric(String::from("10")),
        Token::Numeric(String::from("25")),
        Token::Numeric(String::from("303")),
      ],
    );
  }

  #[test]
  fn test_scan_float() {
    assert_tokens("3.14", &[Token::Numeric(String::from("3.14"))]);
  }

  #[test]
  fn test_scan_multiple_floats() {
    assert_tokens(
      "1.23 2.50 3.03",
      &[
        Token::Numeric(String::from("1.23")),
        Token::Numeric(String::from("2.50")),
        Token::Numeric(String::from("3.03")),
      ],
    );
  }

  #[test]
  fn test_scan_invalid_float() {
    assert_tokens(
      "1.2.3",
      &[
        Token::Numeric(String::from("1.2")),
        Token::Unknown('.'),
        Token::Numeric(String::from("3")),
      ],
    );
  }

  #[test]
  fn test_scan_name() {
    assert_tokens("foo", &[Token::Name(String::from("foo"))]);
  }

  #[test]
  fn test_scan_multiple_names() {
    assert_tokens(
      "foo bar baz",
      &[
        Token::Name(String::from("foo")),
        Token::Name(String::from("bar")),
        Token::Name(String::from("baz")),
      ],
    );
  }

  #[test]
  fn test_scan_parentheses() {
    assert_tokens(
      "foo(bar)",
      &[
        Token::Name(String::from("foo")),
        Token::Pair(PairSymbol::Parentheses, PairType::Open),
        Token::Name(String::from("bar")),
        Token::Pair(PairSymbol::Parentheses, PairType::Close),
      ],
    );
  }

  #[test]
  fn test_scan_curlies() {
    assert_tokens(
      "class A {}",
      &[
        Token::Name(String::from("class")),
        Token::Name(String::from("A")),
        Token::Pair(PairSymbol::CurlyBracket, PairType::Open),
        Token::Pair(PairSymbol::CurlyBracket, PairType::Close),
      ],
    );
  }

  #[test]
  fn test_scan_string() {
    assert_tokens("'foo'", &[Token::String(String::from("foo"))]);
  }

  #[test]
  fn test_scan_string_no_terminator() {
    assert_tokens("'foo", &[Token::String(String::from("foo"))]);
  }

  #[test]
  fn test_scan_string_line_terminator() {
    assert_tokens(
      "'foo\nbar'",
      &[
        Token::String(String::from("foo")),
        Token::Name(String::from("bar")),
        Token::String(String::from("")),
      ],
    );
  }

  #[test]
  fn test_comment() {
    assert_tokens("// Hello", &[Token::Comment(String::from(" Hello"))])
  }

  #[test]
  fn test_comment_line_terminator() {
    assert_tokens(
      "// Foo\nbar",
      &[
        Token::Comment(String::from(" Foo")),
        Token::Name(String::from("bar")),
      ],
    );
  }
}
