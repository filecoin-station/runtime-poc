#[link(wasm_import_module = "host")]
extern "C" {
  #[link_name = "log"]
  fn host_log();

  #[link_name = "sleep"]
  fn host_sleep(duration_in_ms: u64);
}

fn log() {
  unsafe { host_log() }
}

fn sleep(duration_in_ms: u64) {
  unsafe { host_sleep(duration_in_ms) }
}

#[no_mangle]
pub extern "C" fn wget() {
  log();
  sleep(1000);
  log();
}
