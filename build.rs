extern crate libbindgen;

use std::env;
use std::path::Path;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let _ = libbindgen::builder()
    .header("example.h")
    .use_core()
    .generate().unwrap()
    .write_to_file(Path::new(&out_dir).join("example.rs"));
}