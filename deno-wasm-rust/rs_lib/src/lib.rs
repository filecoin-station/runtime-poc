use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

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
    JsFuture::from(window.fetch_with_str("https://www.filstation.app")).await?;

  // `resp_value` is a `Response` object.
  // assert!(resp_value.is_instance_of::<Response>());
  assert!(resp_value.is_object());
  let resp: Response = resp_value.dyn_into::<Response>().unwrap();

  console::log_1(&"response".into());

  console::log_2(&"Status:".into(), &JsValue::from(resp.status()));
  // log(format!("Status: {} {}", resp.))

  // Convert this other `Promise` into a rust `Future`.
  // let json = JsFuture::from(resp.json()?).await?;

  Ok(JsValue::UNDEFINED)
}
