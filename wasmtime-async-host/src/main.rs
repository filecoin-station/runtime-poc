use std::{env, time::Duration};

use wasmtime::{Config, Engine, Linker, Module, Store};

struct MyState {
  // Here we can define our custom state
  // message: String,
}

fn zinnia_log(_caller: wasmtime::Caller<'_, MyState>, msg: &str) {
  println!("[MODULE] {msg}");
}

async fn zinnia_sleep(
  _caller: wasmtime::Caller<'_, MyState>,
  duration_in_ms: u64,
) {
  println!("[ZINNIA] going to sleep for {duration_in_ms}ms");
  tokio::time::sleep(Duration::from_millis(duration_in_ms)).await;
  println!("[ZINNIA] waking up");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let exe_path = env::current_exe()?;
  println!("exe path: {}", exe_path.display());
  let lib_wasm_file = exe_path
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("target/wasm32-unknown-unknown/debug/rs_lib.wasm");
  println!("wasm path: {}", lib_wasm_file.display());

  let mut config = Config::new();
  // We need this engine's `Store`s to be async
  config.async_support(true);

  // First the wasm module needs to be compiled. This is done with a global
  // "compilation environment" within an `Engine`. Note that engines can be
  // further configured through `Config` if desired instead of using the
  // default like this is here.
  println!("Setting up WASM engine...");
  let engine = Engine::new(&config)?;
  let mut linker = Linker::new(&engine);

  let mut store = Store::new(
    &engine,
    MyState {
      // here we can initialize our custom state
    },
  );

  println!("Compiling module...");
  let module = Module::from_file(&engine, lib_wasm_file)?;

  linker.func_wrap1_async("host", "sleep", |caller, duration_in_ms: u64| {
    Box::new(async move {
      zinnia_sleep(caller, duration_in_ms).await;
      () // empty return
    })
  })?;

  linker.func_wrap(
    "host",
    "log",
    |caller: wasmtime::Caller<'_, MyState>| {
      // TODO: how to allow the module to give use the message (a pointer to raw bytes + length)
      zinnia_log(caller, "Hello from the WASM module");
      () // empty return
    },
  )?;

  // Once we've got that all set up we can then move to the instantiation
  // phase, pairing together a compiled module as well as a set of imports.
  // Note that this is where the wasm `start` function, if any, would run.
  println!("Instantiating module...");
  let instance = linker.instantiate_async(&mut store, &module).await?;

  // Next we poke around a bit to extract the `wget` function from the module.
  println!("Extracting export...");
  let wget = instance
    .get_typed_func::<(), (), &mut Store<MyState>>(&mut store, "wget")?;

  // And last but not least we can call it!
  println!("Calling export...");
  let res = futures::future::join(
    //
    // Call the exported function. It will call back `zinnia_sleep`, which yields back to us
    wget.call_async(&mut store, ()),
    //
    // At the same time, wait 200ms and then print a log message
    (|| async {
      tokio::time::sleep(Duration::from_millis(200)).await;
      println!("[DEMO] 200ms elapsed");
    })(),
  )
  .await;
  // A hacky way how to handle errors in calling the exported function
  res.0?;

  println!("Done.");

  Ok(())
}
