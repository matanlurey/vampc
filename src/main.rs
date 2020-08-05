use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    2 => {
      let file = &args[1];
      load(file);
    }
    _ => {
      println!("Usage: vampc <script>");
      std::process::exit(1);
    }
  }
}

fn run(source: String) {
  let mut scanner = Scanner::new(source);
  scanner.scan();
  for token in scanner.output {
    println!("{:?}", token);
  }
}

fn load(input: &str) {
  let result = fs::read_to_string(input);
  let contents = result.expect("Could not read file");
  run(contents);
}

struct Scanner {
  input: String,
  output: Vec<Token>,
}

impl Scanner {
  fn new(input: String) -> Scanner {
    Scanner {
      input,
      output: Vec::new(),
    }
  }

  fn scan(&mut self) {
    let _chars = self.input.chars().peekable();
  }
}

#[derive(Debug)]
enum Token {}
