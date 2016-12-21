extern crate libbindgen;

use std::env;
use std::path::Path;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let config = libbindgen::CodegenConfig {
            functions: true,
            types: true,
            vars: false,
            methods: false,
            constructors: false,
        };
  let _ = libbindgen::builder()
    .header("cef.h")
    .clang_arg("-I/home/gzunino/Downloads/cef_binary_3.2883.1539.gd7f087e_linux64/")
    .use_core()
    .with_codegen_config(config)
    //.no_unstable_rust()
    .whitelisted_type("_cef_main_args_t")
    //.hide_type("XEvent")
    .generate().unwrap()
    .write_to_file(Path::new(&out_dir).join("cef.rs"));
}