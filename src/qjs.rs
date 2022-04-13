fn main() {
  let context = quick_js::Context::new().unwrap();

  let value = context.eval_as::<String>("'Hello World!'").unwrap();
  print!("{}", &value);
}
