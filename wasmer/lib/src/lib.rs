use std::fs::File;
use std::io::Write;

#[no_mangle]
pub extern "C" fn wget() -> () {
  println!("Hello, world from WASM!");

  // Create the target file - this is standard Rust API for sync I/O
  println!("Creating output file: wasmer-station.html");
  let mut file = File::create("wasmer-station.html").unwrap();

  file.write_all(b"<html><head><title>").unwrap();
  file.write_all(b"Filecoin Station").unwrap();
  file.write_all(b"</title></head><html>").unwrap();
}
