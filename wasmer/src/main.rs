use std::env;

use wasmer::TypedFunction;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let exe_path = env::current_exe()?;
  println!("exe path: {}", exe_path.display());
  let lib_wasm_file = exe_path
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("wasm32-wasi/debug/rs_lib.wasm");

  // Setup the engine
  let mut store = Store::default();

  // Load our WASM module
  let file_str = lib_wasm_file.display().to_string();
  let module = Module::from_file(&store, lib_wasm_file)
    .expect(&format!("load WASM module from {}", file_str));

  // Create the `WasiEnv`
  println!("Setting up WASI runtime");
  let wasi_env = WasiState::new("demo")
    // .args(&["world"])
    // .env("KEY", "Value")
    .finalize(&mut store)?;

  /*
  // Define what can be imported by WASM modules
  let import_object = imports! {
      // We use the default namespace "env".
      "env" => {
          // And call our function "say_hello".
          // "say_hello" => Function::new_typed(&mut store, say_hello_world),
      }
  };
  */
  // Create WASI import object instead
  let import_object = wasi_env.import_object(&mut store, &module)?;

  // Instantiate the module
  println!("Instantiating our WASM module");
  let instance = Instance::new(&mut store, &module, &import_object)?;

  println!("Attach WASI memory...");
  // Attach the memory export
  let memory = instance.exports.get_memory("memory")?;
  wasi_env.data_mut(&mut store).set_memory(memory.clone());

  // Find the exported function `wget`
  let wget_func: TypedFunction<(), ()> =
    instance.exports.get_typed_function(&mut store, "wget")?;

  // Finally, call the function
  println!("Executing the exported function");
  wget_func.call(&mut store)?;

  Ok(())
}
