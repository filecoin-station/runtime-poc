use std::env;

use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

struct MyState {
  wasi: WasiCtx,
  // Here we can define our custom state
  // message: String,
}

fn main() {
  let exe_path = env::current_exe().unwrap();
  println!("exe path: {}", exe_path.display());
  let lib_wasm_file = exe_path
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("lib/target/wasm32-wasi/debug/rs_lib.wasm");

  // First the wasm module needs to be compiled. This is done with a global
  // "compilation environment" within an `Engine`. Note that engines can be
  // further configured through `Config` if desired instead of using the
  // default like this is here.
  println!("Setting up WASI engine...");
  let engine = Engine::default();

  // Define the WASI functions globally on the `Config`.
  let mut linker = Linker::new(&engine);
  wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| {
    &mut state.wasi
  })
  .unwrap();

  // Create a WASI context and put it in a Store; all instances in the store
  // share this context. `WasiCtxBuilder` provides a number of ways to
  // configure what the target program will have access to.
  let wasi = WasiCtxBuilder::new()
    .inherit_stdio()
    // .inherit_args()?
    .build();
  let mut store = Store::new(
    &engine,
    MyState {
      wasi,
      // here we can initialize our custom state
    },
  );

  println!("Compiling module...");
  let module = Module::from_file(&engine, lib_wasm_file).unwrap();

  linker.module(&mut store, "", &module).unwrap();

  /*
  // Our wasm module we'll be instantiating requires one imported function.
  // the function takes no parameters and returns no results. We create a host
  // implementation of that function here, and the `caller` parameter here is
  // used to get access to our original `MyState` value.
  println!("Creating callback...");
  let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
    println!("Calling back...");
    println!("> {}", caller.data().name);
    caller.data_mut().count += 1;
  });
  */

  // Once we've got that all set up we can then move to the instantiation
  // phase, pairing together a compiled module as well as a set of imports.
  // Note that this is where the wasm `start` function, if any, would run.
  println!("Instantiating module...");
  // let imports = [hello_func.into()];
  // let instance =
  //   Instance::new(&mut store, &module, /* &imports */ &[]).unwrap();
  let instance = linker.instantiate(&mut store, &module).unwrap();

  // Next we poke around a bit to extract the `wget` function from the module.
  println!("Extracting export...");
  let wget = instance
    .get_typed_func::<(), ()>(&mut store, "wget")
    .unwrap();

  // And last but not least we can call it!
  println!("Calling export...");
  wget.call(&mut store, ()).unwrap();

  println!("Done.");
}
