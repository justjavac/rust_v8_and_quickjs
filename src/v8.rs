use v8;

fn main() {
  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  let isolate = &mut v8::Isolate::new(Default::default());

  let scope = &mut v8::HandleScope::new(isolate);
  let context = v8::Context::new(scope);
  let scope = &mut v8::ContextScope::new(scope, context);

  let code = v8::String::new(scope, "'Hello World!'").unwrap();
  let script = v8::Script::compile(scope, code, None).unwrap();
  let result = script.run(scope).unwrap();
  let result = result.to_string(scope).unwrap();
  println!("{}", result.to_rust_string_lossy(scope));
}
