use http;
use std::fs::File;
use std::io::Write;

use wasi_experimental_http;

#[no_mangle]
pub extern "C" fn wget() {
  // This is a semi-standard Rust API
  let req = http::request::Builder::new()
    .uri("https://www.filstation.app")
    .body(None)
    .unwrap();

  // This is custom API provided by wasi_experimental_http
  let mut res = wasi_experimental_http::request(req).unwrap();

  println!("Response status: {}", res.status_code);

  // Create the target file - this is standard Rust API for sync I/O
  println!("creating output file");
  let mut file = File::create("wasmtime-station.html").unwrap();

  // Consume the response body stream, writing each individual chunk to the file
  const BUFFER_SIZE: usize = 65535;
  let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
  // Read the chunk - this is custom API provided by wasi_experimental_http
  while let Ok(n) = res.body_read(&mut buffer) {
    if n == 0 {
      break;
    }
    println!("Read {} bytes", n);
    // Write the chunk - this is standard Rust API for sync I/O
    file.write_all(&buffer).unwrap();
  }
}
