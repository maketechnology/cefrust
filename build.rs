extern crate libbindgen;

use std::env;
use std::path::Path;
use std::string::String;

fn main() {
  let cef_path = format!("{}/Downloads/cef_binary_3.2883.1539.gd7f087e_linux64/", env::var("HOME").unwrap());
  // Tell cargo to tell rustc to link the system shared library.
  //println!("cargo:rustc-link-search={}", format!("{}Release/", cef_path));
  println!("cargo:rustc-link-search={}", format!("{}Debug/", cef_path));
  println!("cargo:rustc-link-lib=cef");
  //cargo:rustc-flags=-l foo -L src/c/foo

  //gen_cef(cef_path);

  println!("cargo:rustc-link-lib=gtk-x11-2.0");
  println!("cargo:rustc-link-lib=gdk-x11-2.0");
  println!("cargo:rustc-link-lib=X11");
  
  //gen_gtk();
}

#[allow(dead_code)]
fn gen_cef(cef_path: String) {
    //let out_dir = env::var("OUT_DIR").unwrap();
  let config = libbindgen::CodegenConfig {
            functions: true,
            types: true,
            vars: false,
            methods: true,
            constructors: false,
        };
  let _ = libbindgen::builder()
    .header("cef.h")
    .clang_arg(format!("-I{}", cef_path))
    .link("cef")
    //.use_core()
    .with_codegen_config(config)
    .no_unstable_rust()
    .raw_line("#![allow(dead_code)]")
    .raw_line("#![allow(non_snake_case)]")
    .raw_line("#![allow(non_camel_case_types)]")
    .whitelisted_type("_cef_main_args_t")
    //.hide_type(".*XEvent")
    .whitelisted_function("cef_string_.*")
    .whitelisted_function("cef_execute_process")
    .whitelisted_function("cef_initialize")
    .whitelisted_function("cef_run_message_loop")
    .whitelisted_function("cef_shutdown")
    .whitelisted_function("cef_browser_host_create_browser")
    .whitelisted_function("cef_.*")
    .generate().unwrap()
    //.write_to_file(Path::new(&out_dir).join("cef.rs"));
    .write_to_file(Path::new("src").join("cef.rs"));
}

#[allow(dead_code)]
fn gen_gtk() {
  //let out_dir = env::var("OUT_DIR").unwrap();
  let config = libbindgen::CodegenConfig {
            functions: true,
            types: true,
            vars: true,
            methods: true,
            constructors: true,
        };
  let _ = libbindgen::builder()
    //.header("/usr/include/gtk-2.0/gtk/gtk.h")
    .header("gtk2.h")
    .clang_arg(format!("-I{}", "/usr/include/gtk-2.0"))
    .clang_arg(format!("-I{}", "/usr/include/glib-2.0"))
    .clang_arg(format!("-I{}", "/usr/lib/x86_64-linux-gnu/glib-2.0/include"))
    .clang_arg(format!("-I{}", "/usr/include/cairo"))
    .clang_arg(format!("-I{}", "/usr/include/pango-1.0"))
    .clang_arg(format!("-I{}", "/usr/lib/x86_64-linux-gnu/gtk-2.0/include"))
    .clang_arg(format!("-I{}", "/usr/include/gdk-pixbuf-2.0"))
    .clang_arg(format!("-I{}", "/usr/include/atk-1.0"))
    //.use_core()
    .with_codegen_config(config)
    //.no_unstable_rust()
    .raw_line("#![allow(dead_code)]")
    .raw_line("#![allow(non_snake_case)]")
    .raw_line("#![allow(non_camel_case_types)]")

    .whitelisted_function("gtk_init")
    .whitelisted_function("gtk_window_new")
    .whitelisted_function("gtk_widget_get_window")
    .whitelisted_function("gtk_window_new")
    .whitelisted_function("gdk_x11_drawable_get_xid")
    .whitelisted_function("gtk_window_set_default_size")
    .whitelisted_function("gtk_window_set_position")
    .whitelisted_function("gtk_window_set_title")
    .whitelisted_function("gtk_vbox_new")
    .whitelisted_function("gtk_container_add")
    .whitelisted_function("gtk_widget_show_all")
    .whitelisted_function("gtk_main")
    .whitelisted_function("g_signal_connect")
    .whitelisted_function("gtk_widget_get_allocation")
    .whitelisted_function("g_signal_connect_data")
    .generate().unwrap()
    .write_to_file(Path::new("src").join("gtk2.rs"));
}