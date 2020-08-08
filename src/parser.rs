use scanner::Keyword;
use scanner::PairSymbol;
use scanner::PairType;
use scanner::Token;
use std::iter;

/// Represents an architecture for parsing tokens into an AST (tree).
pub struct Parser {
  pub input: Vec<Token>,

  pub output: Vec<Declaration>,
}

/// Known top-level declarations.
#[derive(Debug, PartialEq)]
pub enum Declaration {
  Comment { text: String },
  Function { name: String, body: Vec<Statement> },
}

/// Known statements.
#[derive(Debug, PartialEq)]
pub enum Statement {
  Comment {
    text: String,
  },
  Expression {
    expression: Expression,
  },
  Variable {
    name: String,
    value: Option<Expression>,
  },
}

/// Known expressions.
#[derive(Debug, PartialEq)]
pub enum Expression {
  Assignment {
    name: String,
    value: Box<Expression>,
  },
  Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: BinaryOperator,
  },
}

/// Known binary operators.
#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
  Addition,
  Equality,
  Subtraction,
}

impl Parser {
  pub fn new(input: Vec<Token>) -> Parser {
    Parser {
      input,
      output: Vec::new(),
    }
  }

  pub fn parse(&mut self) {
    let mut tokens = self.input.iter().peekable();
    while let Some(next) = tokens.next() {
      let declaration: Option<Declaration> = match next {
        Token::Comment(comment) => Some(Declaration::Comment {
          text: Parser::parse_comment_contents(comment, &mut tokens),
        }),
        Token::Keyword(keyword) => match keyword {
          Keyword::Func => {
            Some(Parser::parse_function_declaration(&mut tokens))
          }
          _ => panic!("Unexpected keyword @ top level: {:?}", next),
        },
        _ => panic!("Unexpected token @ top level: {:?}", next),
      };
      if let Some(declaration) = declaration {
        self.output.push(declaration);
      }
    }
  }

  fn parse_comment_contents<'a, T: Iterator<Item = &'a Token>>(
    initial: &'a str,
    tokens: &mut iter::Peekable<T>,
  ) -> String {
    let mut buffer = String::from(initial);
    while let Some(Token::Comment(comment)) = tokens.peek() {
      buffer.push_str("\n");
      buffer.push_str(comment);
      tokens.next();
    }
    buffer
  }

  fn parse_function_declaration<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut iter::Peekable<T>,
  ) -> Declaration {
    if let Some(Token::Identifier(name)) = tokens.peek() {
      tokens.next();
      let body = Parser::parse_statement_body(tokens);
      Declaration::Function {
        name: name.to_string(),
        body,
      }
    } else {
      panic!("Expected Identifier");
    }
  }

  fn parse_statement_body<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut iter::Peekable<T>,
  ) -> Vec<Statement> {
    let mut statements = Vec::<Statement>::new();
    match tokens.peek() {
      Some(Token::Pair(PairSymbol::CurlyBracket, PairType::Open)) => {
        tokens.next();
      }
      _ => panic!("Expected {"),
    }
    loop {
      match tokens.peek() {
        Some(Token::Pair(PairSymbol::CurlyBracket, PairType::Close)) => {
          tokens.next();
          break;
        }
        None => {
          panic!("Expected }");
        }
        _ => {
          if let Some(statement) = Parser::parse_statement(tokens) {
            statements.push(statement);
          } else {
            break;
          }
        }
      }
    }
    statements
  }

  fn parse_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut iter::Peekable<T>,
  ) -> Option<Statement> {
    if let Some(expression) = Parser::parse_expression(tokens) {
      Some(Statement::Expression { expression })
    } else {
      None
    }
  }

  fn parse_expression<'a, T: Iterator<Item = &'a Token>>(
    _tokens: &mut iter::Peekable<T>,
  ) -> Option<Expression> {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_tree(input: Vec<Token>, output: &[Declaration]) {
    let mut parser = Parser::new(input);
    parser.parse();
    assert_eq!(parser.output, output);
  }
  #[test]
  fn test_top_level_comment() {
    assert_tree(
      vec![Token::Comment(String::from("Hello World"))],
      &[Declaration::Comment {
        text: String::from("Hello World"),
      }],
    );
  }

  #[test]
  fn test_top_level_comments() {
    assert_tree(
      vec![
        Token::Comment(String::from("Hello")),
        Token::Comment(String::from("World")),
      ],
      &[Declaration::Comment {
        text: String::from("Hello\nWorld"),
      }],
    );
  }

  #[test]
  fn test_function_declaration() {
    assert_tree(
      vec![
        Token::Keyword(Keyword::Func),
        Token::Identifier(String::from("main")),
        Token::Pair(PairSymbol::CurlyBracket, PairType::Open),
        Token::Pair(PairSymbol::CurlyBracket, PairType::Close),
      ],
      &[Declaration::Function {
        name: String::from("main"),
        body: Vec::new(),
      }],
    );
  }
}
