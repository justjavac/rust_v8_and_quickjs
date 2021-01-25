use quick_js::Context;

fn main() {
  let context = Context::new().unwrap();

  let value = context.eval_as::<String>("'Hello World!'").unwrap();
  print!("{}", &value);
}
