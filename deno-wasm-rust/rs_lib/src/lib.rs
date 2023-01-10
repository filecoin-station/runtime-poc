use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

use js_sys::Uint8Array;
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

  console::log_1(&"creating output file".into());
  let result = File::create("rusty-station.html");
  /* ^^^ that does not work in Deno :(
    creating output file
    error: Uncaught (in promise) "operation not supported on this platform"
  */
  if let Err(err) = result {
    return Err(err.to_string().into());
  }

  let mut file = result.unwrap();

  // Consume the stream, writing each individual chunk to the file
  while let Some(Ok(chunk)) = stream.next().await {
    console::log_1(&chunk);
    let u8arr = chunk.dyn_into::<Uint8Array>().unwrap_throw();
    console::log_2(
      &"writing %s bytes".into(),
      &JsValue::from(u8arr.byte_length()),
    );
    // FIXME: can we avoid copying the chunk data into a new u8 vector? Maybe using a BYOB reader?
    let data = u8arr.to_vec();
    file.write_all(&data).unwrap_throw();
  }

  Ok(JsValue::UNDEFINED)
}
