fn main() {
  let platform = v8::new_default_platform(1, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  let isolate = &mut v8::Isolate::new(Default::default());

  let scope = &mut v8::HandleScope::new(isolate);

  let global_template = {
    // Register a "log(msg)" function accepting a string
    // See https://github.com/v8/v8/blob/master/samples/process.cc
    let global = v8::ObjectTemplate::new(scope);
    let key = v8::String::new(scope, "log").unwrap();
    let value = v8::FunctionTemplate::new(scope, log_cb);
    global.set(key.into(), value.into());

    global
  };

  let context = v8::Context::new_from_template(scope, global_template);
  let scope = &mut v8::ContextScope::new(scope, context);

  let code = v8::String::new(scope, "log('Hello' + ' World!')").unwrap();
  println!("javascript code: {}", code.to_rust_string_lossy(scope));

  let script = v8::Script::compile(scope, code, None).unwrap();
  let result = script.run(scope).unwrap();
  let result = result.to_string(scope).unwrap();
  println!("result: {}", result.to_rust_string_lossy(scope));
}

fn log_cb(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  _rv: v8::ReturnValue,
) {
  if args.length() != 1 {
    return throw_type_error(scope, "Invalid arguments");
  }

  let msg = args.get(0).to_rust_string_lossy(scope);
  println!("[V8] {}", msg)
}

// https://github.com/denoland/deno/blob/d318e38b76c8174d48fddfb99064401050cd8333/core/bindings.rs#L622-L626
pub fn throw_type_error(scope: &mut v8::HandleScope, message: impl AsRef<str>) {
  let message = v8::String::new(scope, message.as_ref()).unwrap();
  let exception = v8::Exception::type_error(scope, message);
  scope.throw_exception(exception);
}
