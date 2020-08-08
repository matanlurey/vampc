/// Handles source code being scanned into lexigraphical symbols (tokens).
///
/// Each token includes offset and raw (string) contents for parsing/debugging.
pub fn scan(input: String) -> Vec<Span> {
  let mut spans = Vec::<Span>::new();
  let mut chars = input.chars().peekable();
  let mut offset: usize = 0;
  while let Some(next) = chars.next() {
    let mut contents: String = next.to_string();
    let mut kind = Token::Unknown;
    match next {
      // Possible comment.
      '/' => {
        if let Some('/') = chars.peek() {
          contents = String::from("//");
          kind = Token::Comment;
          chars.next();
          loop {
            match chars.next() {
              Some('\n') | None => break,
              Some(other) => contents.push(other),
            }
          }
        }
      }
      // Whitespace.
      // Do nothing (skip this token as its not significant).
      ' ' | '\n' | '\t' => {
        offset += 1;
        continue;
      }
      // Unknown.
      // Do nothing (will add an unknown token to output).
      _ => {}
    }
    spans.push(Span {
      offset,
      contents: contents.to_owned(),
      kind,
    });
    offset += contents.len();
  }
  spans
}

/// A span of source text.
#[derive(Debug, PartialEq)]
pub struct Span {
  /// Starting offset of the text.
  offset: usize,
  /// Contents of the text.
  contents: String,
  /// Type of token interpreted.
  kind: Token,
}

/// A recognized kind of source span during scanning.
#[derive(Debug, PartialEq)]
pub enum Token {
  /// A single-line comment block (`// ...`).
  Comment,
  /// An invalid or unrecognized block of text.
  Unknown,
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_tokens(input: &str, output: &[Span]) {
    let result = scan(input.to_owned());
    assert_eq!(result, output);
  }

  #[test]
  fn scan_empty() {
    assert_tokens("", &[]);
  }

  #[test]
  fn scan_error() {
    assert_tokens(
      "!",
      &[Span {
        offset: 0,
        contents: "!".to_owned(),
        kind: Token::Unknown,
      }],
    );
  }

  #[test]
  fn scan_white_space_only() {
    assert_tokens(" \n \n\t", &[]);
  }

  #[test]
  fn scan_single_line_comment_terminated_by_eof() {
    assert_tokens(
      "// Hello World",
      &[Span {
        contents: "// Hello World".to_owned(),
        offset: 0,
        kind: Token::Comment,
      }],
    );
  }

  #[test]
  fn scan_single_line_comment_terminated_by_newline() {
    assert_tokens(
      "// Hello World\n",
      &[Span {
        contents: "// Hello World".to_owned(),
        offset: 0,
        kind: Token::Comment,
      }],
    );
  }

  #[test]
  fn scan_multiple_comments() {
    assert_tokens(
      "// 1\n// 2\n// 3",
      &[
        Span {
          contents: "// 1".to_owned(),
          offset: 0,
          kind: Token::Comment,
        },
        Span {
          contents: "// 2".to_owned(),
          offset: 4,
          kind: Token::Comment,
        },
        Span {
          contents: "// 3".to_owned(),
          offset: 8,
          kind: Token::Comment,
        },
      ],
    )
  }

  #[test]
  fn scan_error_not_comment() {
    assert_tokens(
      "/",
      &[Span {
        offset: 0,
        contents: "/".to_owned(),
        kind: Token::Unknown,
      }],
    );
  }
}
