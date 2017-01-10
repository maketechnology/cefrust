extern crate libbindgen;

use std::env;
use std::path::Path;

fn main() {
  let cef_path = format!("{}/Downloads/cef_binary_3.2883.1539.gd7f087e_linux64/", env::var("HOME").unwrap());
  // Tell cargo to tell rustc to link the system shared library.
  println!("cargo:rustc-link-search={}", format!("{}Release/", cef_path));
  println!("cargo:rustc-link-lib=cef");
  //cargo:rustc-flags=-l foo -L src/c/foo

  //let out_dir = env::var("OUT_DIR").unwrap();
  let config = libbindgen::CodegenConfig {
            functions: true,
            types: true,
            vars: false,
            methods: false,
            constructors: false,
        };
  let _ = libbindgen::builder()
    .header("cef.h")
    .clang_arg(format!("-I{}", cef_path))
    //.clang_arg("-L/home/gzunino/Downloads/cef_binary_3.2883.1539.gd7f087e_linux64/Release")
    //.clang_arg("-lcef")
    .link("cef")
    //.use_core()
    .with_codegen_config(config)
    .no_unstable_rust()
    .whitelisted_type("_cef_main_args_t")
    //.whitelisted_type("_cef_client_t")
    //.hide_type(".*XEvent")
    .whitelisted_function("cef_string_.*")
    .whitelisted_function("cef_execute_process")
    .whitelisted_function("cef_initialize")
    .whitelisted_function("cef_run_message_loop")
    .whitelisted_function("cef_shutdown")
    .whitelisted_function("cef_browser_host_create_browser")
    .generate().unwrap()
    //.write_to_file(Path::new(&out_dir).join("cef.rs"));
    .write_to_file(Path::new("src").join("cef.rs"));
}