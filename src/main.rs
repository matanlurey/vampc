fn main() {
  println!("{}", greet());
}

fn greet() -> String {
  "Hello World!".to_string()
}

#[test]
fn test_greet() {
  assert_eq!("Hello World!", greet())
}
