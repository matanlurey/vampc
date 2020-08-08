use scanner::Token;

/// Represents an architecture for parsing tokens into an AST (tree).
pub struct Parser {
  pub input: Vec<Token>,

  pub output: Vec<Declaration>,
}

/// Known top-level declarations.
pub enum Declaration {
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

  pub fn parse(&mut self) {}
}
