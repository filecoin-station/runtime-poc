use futures_util::StreamExt;

use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

use wasm_streams::{ReadableStream, WritableStream};

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);

  #[wasm_bindgen(js_namespace = Deno, js_name = create)]
  pub fn create_file(s: &str) -> js_sys::Promise;
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
  let fs_file = JsFuture::from(create_file(&"rusty-station.html")).await?;
  assert!(fs_file.is_object());
  // Call `file.writable` via Reflection.
  // In the real runtime, we would describe Deno API via wasm-bindgen interfaces,
  // cast fs_file from JsObject to Deno.FsFile, and then use Rust to get the property
  let raw_writable = Reflect::get(&fs_file, &"writable".into()).unwrap_throw();
  let mut writable =
    WritableStream::from_raw(raw_writable.dyn_into().unwrap_throw());
  // Call writable.get_writer()
  let mut writer = writable.get_writer();
  // Alternatively, we could call e.g. `writable.into_sync` to obtain futures::sink::Sink

  // Consume the response body stream, writing each individual chunk to the file
  while let Some(Ok(chunk)) = stream.next().await {
    console::log_2(
      &"writing %s bytes".into(),
      &Reflect::get(&chunk, &"byteLength".into()).unwrap_throw(),
    );
    writer.write(chunk).await?;
  }

  // Manually close the file.
  // Note that our PoC is not error-safe - if any of the code above throws,
  // then we never close the file
  //
  // In the real runtime, we would describe Deno API via wasm-bindgen interfaces,
  // and find a way how to call file.close() automatically when the handle leaves the scope
  console::log_1(&"closing the file".into());
  Reflect::get(&fs_file, &"close".into())
    .unwrap_throw()
    .dyn_into::<Function>()
    .unwrap_throw()
    .call0(&fs_file)
    .unwrap_throw();

  Ok(JsValue::UNDEFINED)
}
