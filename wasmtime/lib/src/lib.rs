use std::fs::File;
use std::io::Write;

#[no_mangle]
pub extern "C" fn wget() {
  /*
  let mut res = attohttpc::get("https://www.filstation.app").send().unwrap();
  println!("Response status: {}", res.status());

  const BUFFER_SIZE: usize = 65535;
  let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
  while let Ok(n) = res.read(&mut buffer) {
    if n == 0 {
      break;
    }
    println!("Read {} bytes", n);
  }
  */

  println!("creating output file");
  let mut file = File::create("wasmtime-station.html").unwrap();

  // Pretend we have a readable stream of the HTTP response body
  // and we want to write it to the file chunk by chunk
  file.write_all(b"<html><head><title>").unwrap();
  file.write_all(b"Filecoin Station").unwrap();
  file.write_all(b"</title></head></html>").unwrap();
}
