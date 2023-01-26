// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
//!  This example shows you how to define ops in Rust and then call them from
//!  JavaScript.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use deno_core::op;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::OpState;
use deno_core::RuntimeOptions;

#[op]
fn op_log(msg: String) -> Result<(), deno_core::error::AnyError> {
  println!("[MODULE] {msg}");
  Ok(())
}

#[op]
async fn op_sleep(
  _state: Rc<RefCell<OpState>>,
  duration_in_ms: u64,
) -> Result<(), deno_core::error::AnyError> {
  println!("[ZINNIA] going to sleep for {duration_in_ms}ms");
  tokio::time::sleep(Duration::from_millis(duration_in_ms)).await;
  println!("[ZINNIA] waking up");
  Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  // Build a deno_core::Extension providing custom ops
  let ext = Extension::builder("zinnia")
    .ops(vec![
      // An op for summing an array of numbers
      // The op-layer automatically deserializes inputs
      // and serializes the returned Result & value
      op_log::decl(),
      op_sleep::decl(),
    ])
    .build();

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![ext],
    will_snapshot: false,
    inspector: false,
    ..Default::default()
  });

  println!("== Running the JavaScript demo ==");

  runtime
    .execute_script("demo-module.js", include_str!("../mod-js/demo-module.js"))
    .unwrap();

  runtime.run_event_loop(false).await.unwrap();

  println!("== DONE ==");
}
