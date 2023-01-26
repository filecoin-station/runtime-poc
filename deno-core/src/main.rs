use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use deno_core::anyhow::anyhow;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::url::Url;
use deno_core::ByteString;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;
use deno_core::ModuleSpecifier;
use deno_core::ModuleType;
use deno_core::OpState;
use deno_core::RuntimeOptions;
use futures::FutureExt;

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
  println!("== Running the JavaScript demo ==");
  run_module(&Url::parse("zinnia://demo-module.js").unwrap())
    .await
    .unwrap();

  println!("== Running the Rust/WASM demo ==");

  run_module(&Url::parse("zinnia://demo-module-rs.js").unwrap())
    .await
    .unwrap();

  println!("== DONE ==");
}

async fn run_module(
  specifier: &ModuleSpecifier,
) -> Result<(), deno_core::anyhow::Error> {
  // Build a deno_core::Extension providing custom ops
  let ext = Extension::builder("zinnia")
    .ops(vec![
      // An op for summing an array of numbers
      // The op-layer automatically deserializes inputs
      // and serializes the returned Result & value
      op_log::decl(),
      op_sleep::decl(),
      op_base64_atob::decl(),
    ])
    .build();

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![ext],
    will_snapshot: false,
    inspector: false,
    module_loader: Some(Rc::new(ZinniaModuleLoader)),
    ..Default::default()
  });

  // Running a script that's not an ES module
  //
  // runtime
  //   .execute_script("demo-module.js", include_str!("../mod-js/demo-module.js"))
  //   .unwrap();
  // runtime.run_event_loop(false).await.unwrap();

  // Load TextEncoder and TextDecoder APIs
  runtime
    .execute_script(
      "zinnia://text-encoding.js",
      include_str!("./text-encoding.js"),
    )
    .unwrap();

  // Create global `atob` function
  runtime
    .execute_script(
      "zinnia://atob.js",
      r#"
  globalThis.atob = function(str) { return Deno.core.ops.op_base64_atob(str); };
  "#,
    )
    .unwrap();

  // Enable Async Ops
  runtime
    .execute_script(
      "zinnia://enable-async-ops.js",
      "Deno.core.initializeAsyncOps()",
    )
    .unwrap();

  let main_module_id = runtime.load_main_module(specifier, None).await?;

  // println!("evaluating the demo module");
  let res = runtime.mod_evaluate(main_module_id);

  // println!("running the event loop");
  runtime.run_event_loop(false).await?;
  // println!("awaiting module evaluation result");
  res.await??;

  Ok(())
}

// This is another shim for WebAPIs provided by Deno
// See https://github.com/denoland/deno/blob/21065797f6dce285e55705007f54abe2bafb611c/ext/web/lib.rs#L138-L155

#[op]
fn op_base64_atob(mut s: ByteString) -> Result<ByteString, AnyError> {
  let decoded_len = forgiving_base64_decode_inplace(&mut s)?;
  s.truncate(decoded_len);
  Ok(s)
}

/// See <https://infra.spec.whatwg.org/#forgiving-base64>
#[inline]
fn forgiving_base64_decode_inplace(
  input: &mut [u8],
) -> Result<usize, AnyError> {
  let error: _ = || anyhow!("Failed to decode base64");
  let decoded =
    base64_simd::forgiving_decode_inplace(input).map_err(|_| error())?;
  Ok(decoded.len())
}

/// Our custom module loader.
pub struct ZinniaModuleLoader;

impl ModuleLoader for ZinniaModuleLoader {
  fn resolve(
    &self,
    specifier: &str,
    _referrer: &str,
    _kind: deno_core::ResolutionKind,
  ) -> Result<deno_core::ModuleSpecifier, deno_core::anyhow::Error> {
    match specifier {
      "zinnia://demo-module.js" => Ok(Url::parse(specifier).unwrap()),
      "zinnia://demo-module-rs.js" => Ok(Url::parse(specifier).unwrap()),
      "zinnia://demo-module-rs.loader.js" => Ok(Url::parse(specifier).unwrap()),
      _ => Err(anyhow!(
        "Zinnia does not support module resolution: {}",
        specifier
      )),
    }
  }

  fn load(
    &self,
    module_specifier: &deno_core::ModuleSpecifier,
    _maybe_referrer: Option<deno_core::ModuleSpecifier>,
    is_dyn_import: bool,
  ) -> std::pin::Pin<Box<deno_core::ModuleSourceFuture>> {
    let specifier = String::from(module_specifier.as_str());
    async move {
      if is_dyn_import {
        return Err(anyhow!(
          "Zinnia does not support dynamic imports. (URL: {})",
          specifier
        ));
      }

      let code = {
        match specifier.as_str() {
          "zinnia://demo-module.js" => include_str!("../mod-js/demo-module.js"),
          "zinnia://demo-module-rs.js" => {
            r#"
import {instantiate} from 'zinnia://demo-module-rs.loader.js';

// Zinnia SDK
globalThis.Zinnia = {
  log(msg) {
    console.trace('log')
    Deno.core.ops.op_log(msg)
  },

  async sleep(durationInMs) {
    console.trace('sleep')
    return Deno.core.ops.op_sleep(durationInMs);
  },
};

// Run the WASM module
const {run} = await instantiate();
await run();
"#
          }
          "zinnia://demo-module-rs.loader.js" => {
            include_str!("../target/deno/mod_rs.generated.js")
          }
          _ => Err(anyhow!("Unknown module: {}", specifier))?,
        }
      };

      let module = ModuleSource {
        code: Box::from(code.as_bytes()),
        module_type: ModuleType::JavaScript,
        module_url_specified: specifier.clone(),
        module_url_found: specifier,
      };
      Ok(module)
    }
    .boxed_local()
  }
}
