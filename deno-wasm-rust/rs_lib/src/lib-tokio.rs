/*
We cannot use Tokio for async filesystem operations:

error: Only features sync,macros,io-util,rt,time are supported on wasm.
   --> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.24.1/src/lib.rs:462:1
    |
462 | compile_error!("Only features sync,macros,io-util,rt,time are supported on wasm.");
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `tokio` due to previous error

According to WASI docs, WASI + wasm-bindgen is experimental.
https://bytecodealliance.github.io/cargo-wasi/wasm-bindgen.html

Usage of wasm-bindgen and WebAssembly Interface Types is highly experimental, it's recommended that
you expect breakage and/or surprises if you're using this.
*/

use js_sys::Uint8Array;
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()

use futures_util::StreamExt; // for IntoStream.next()

use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

use wasm_streams::ReadableStream;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);
}

#[wasm_bindgen]
pub async fn wget() -> Result<JsValue, JsValue> {
  use web_sys::console;

  let window = web_sys::window().unwrap();
  let resp_value =
    // Convert the `Promise` returned by Fetch API into a rust `Future`.
    JsFuture::from(window.fetch_with_str("https://www.filstation.app")).await?;

  // Assert that `resp_value` is a `Response` object.
  // The line below does not compile, so I am using a simpler check - response is an object
  // assert!(resp_value.is_instance_of::<Response>());
  assert!(resp_value.is_object());
  let resp: Response = resp_value.dyn_into::<Response>().unwrap();

  console::log_2(&"Status:".into(), &JsValue::from(resp.status()));

  // Get the response's body as a JS ReadableStream
  let raw_body = resp.body().unwrap_throw();
  let body = ReadableStream::from_raw(raw_body.dyn_into().unwrap_throw());

  // Convert the JS ReadableStream to a Rust stream
  let mut stream = body.into_stream();

  // Create the target file
  // TODO: create this file in the root of deno-wasm-rust
  // Deno code: import.meta.resolve('./station.html'))

  let mut file = File::create("rusty-station.html").await.unwrap_throw();

  // Consume the stream, writing each individual chunk to the file
  while let Some(Ok(chunk)) = stream.next().await {
    let u8arr = chunk.dyn_into::<Uint8Array>().unwrap_throw();
    // FIXME: can we avoid copying the chunk data into a new u8 vector? Maybe using a BYOB reader?
    file.write_all(&u8arr.to_vec()).await.unwrap_throw();
  }

  Ok(JsValue::UNDEFINED)
}
