use std::env;
use std::fs;

#[allow(dead_code)]
mod parser;
mod scanner;

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
  let mut scanner = scanner::Scanner::new(source);
  scanner.scan();
  let mut parser = parser::Parser::new(scanner.output);
  parser.parse();
  for ast in parser.output {
    println!("{:?}", ast);
  }
}

/// Loads a file and passes it to `run_source`.
fn load_and_run(input: &str) {
  let result = fs::read_to_string(input);
  let contents = result.expect("Could not read file");
  run_source(contents);
}
