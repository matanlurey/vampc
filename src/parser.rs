use scanner::Token;
use std::iter;

/// Represents an architecture for parsing tokens into an AST (tree).
pub struct Parser {
  pub input: Vec<Token>,

  pub output: Vec<Declaration>,
}

/// Known top-level declarations.
pub enum Declaration {
  Comment { text: String },
  Function { name: String, body: Vec<Statement> },
}

/// Known statements.
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
        Token::Comment(comment) => {
          let text = Parser::parse_comment_contents(&mut tokens);
          Some(Declaration::Comment { text })
        }
        _ => panic!("Unexpected"),
      };
    }
  }

  fn parse_comment_contents<T: Iterator<Item = Token>>(
    tokens: &mut iter::Peekable<T>,
  ) -> String {
    String::from("")
  }

  fn parse_declaration() -> Option<Declaration> {
    None
  }

  fn parse_statement() -> Option<Statement> {
    None
  }

  fn parse_expression() -> Option<Expression> {
    None
  }
}
