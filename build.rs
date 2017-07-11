extern crate bindgen;

use std::env;
use std::path;
//use std::string::String;

const CEF_TARGET: &'static str = "Release";

fn main() {
  // 2883 branch is ~50MB bigger than 2785 on each platform (and linux requires strip of the .so to reduce 500MB size)
  // due change on build tools.
  // Stay with 2785 until size is reduced on newer versions
  //let cef_path = format!("{}/Downloads/cef_binary_3.2883.1539.gd7f087e_linux64/", env::var("HOME").unwrap());
  let cwd = std::env::current_dir().unwrap();
  let mut cef_path = cwd.clone();
  
  if cfg!(target_os = "macos") {
    cef_path.push("cef_osx");
  } 
  else if cfg!(target_os = "linux") {
    cef_path.push("cef_linux");
  } 
  else if cfg!(target_os = "windows") {
    cef_path.push("cef_windows");
  }

  if cfg!(target_os = "linux") {
    println!("cargo:rustc-link-lib=gtk-x11-2.0");
    println!("cargo:rustc-link-lib=gdk-x11-2.0");
    println!("cargo:rustc-link-lib=X11");
  }

  // Tell cargo to tell rustc to link the system shared library.
  let mut cef_bin = cef_path.clone();
  cef_bin.push(CEF_TARGET);
  let lib = if cfg!(target_os = "windows") {
    println!("cargo:rustc-link-search={}", cef_bin.display()); 
    "libcef" 
  } else if cfg!(target_os = "macos") {
    println!("cargo:rustc-link-search=framework={}", cef_bin.display());
    "framework=Chromium Embedded Framework"
  } else { 
    println!("cargo:rustc-link-search={}", cef_bin.display());
    "cef" 
  };
  println!("cargo:rustc-link-lib={}", lib);

  if cfg!(feature = "gen") {
    gen_cef(cef_path.display());
    gen_os(cef_path.display());
  }
  //let mut cef_path_linux = cwd.clone();
  //cef_path_linux.push("cef_linux");
  //create_links_linux(cef_path_linux.clone());
  //let mut cef_path_win = cwd.clone();
  //cef_path_win.push("cef_windows");
  //create_links_win(cef_path_win.clone());
  create_links(cef_path.clone());
  
  //gen_gtk();
}

#[cfg(target_os = "linux")]
fn create_links(mut cef_path: path::PathBuf) {
  //link(cef_path.clone(), "Resources");
  cef_path.push("Resources");
  link_dir(cef_path.clone(), "locales");
  link(cef_path.clone(), "icudtl.dat");
  link(cef_path.clone(), "cef.pak");
  link(cef_path.clone(), "cef_100_percent.pak");
  link(cef_path.clone(), "cef_200_percent.pak");
  link(cef_path.clone(), "cef_extensions.pak");
  link(cef_path.clone(), "devtools_resources.pak");

  cef_path.pop();
  cef_path.push(CEF_TARGET);
  link(cef_path.clone(), "libcef.so");
  link(cef_path.clone(), "natives_blob.bin");
  link(cef_path.clone(), "snapshot_blob.bin");
}

#[cfg(windows)]
fn create_links(mut cef_path: path::PathBuf) {
  //link(cef_path.clone(), "Resources");
  cef_path.push("Resources");
  link_dir(cef_path.clone(), "locales");
  link(cef_path.clone(), "icudtl.dat");
  link(cef_path.clone(), "cef.pak");
  link(cef_path.clone(), "cef_100_percent.pak");
  link(cef_path.clone(), "cef_200_percent.pak");
  link(cef_path.clone(), "cef_extensions.pak");
  link(cef_path.clone(), "devtools_resources.pak");

  cef_path.pop();
  cef_path.push(CEF_TARGET);  
  link(cef_path.clone(), "chrome_elf.dll");
  link(cef_path.clone(), "d3dcompiler_43.dll");
  link(cef_path.clone(), "d3dcompiler_47.dll");
  link(cef_path.clone(), "libcef.dll");
  link(cef_path.clone(), "libEGL.dll");
  link(cef_path.clone(), "libGLESv2.dll");
  link(cef_path.clone(), "natives_blob.bin");
  link(cef_path.clone(), "snapshot_blob.bin");
}

#[cfg(target_os = "macos")]
fn create_links(mut cef_path: path::PathBuf) {
  extern crate fs_extra;

  cef_path.push(CEF_TARGET);  
  
  let profile = env::var("PROFILE").unwrap();
  let mut out_path = path::PathBuf::from("target");
  out_path.push(profile);

  let mut app_path = out_path.clone();
  app_path.push("cefrust.app/Contents");  
  let mut contents_path = app_path.clone();
  std::fs::create_dir_all(&app_path).ok();

  app_path.push("Frameworks");
  std::fs::create_dir(&app_path).ok();

  let mut opts = fs_extra::dir::CopyOptions::new();
  opts.skip_exist = true;
  fs_extra::dir::copy(cef_path.join("Chromium Embedded Framework.framework"), &app_path, &opts).unwrap();
  //link_gen(cef_path.clone(), app_path.clone(), "Chromium Embedded Framework.framework");

  app_path.push("cefrust_subp.app/Contents");
  std::fs::create_dir_all(&app_path).ok();

  std::fs::copy(path::PathBuf::new().join("PkgInfo"), app_path.join("PkgInfo")).ok();
  std::fs::copy(path::PathBuf::new().join("Info_subp.plist"), app_path.join("Info.plist")).ok();

  app_path.push("MacOS");
  std::fs::create_dir(&app_path).ok();
  //link_gen(out_path.clone(), app_path.clone(), "cefrust_subp");
  
  std::fs::copy(path::PathBuf::new().join("PkgInfo"), contents_path.join("PkgInfo")).ok();
  std::fs::copy(path::PathBuf::new().join("Info.plist"), contents_path.join("Info.plist")).ok();

  contents_path.push("MacOS");
  std::fs::create_dir_all(&contents_path).ok();
  //link_gen(out_path.clone(), contents_path.clone(), "cefrust");
}

#[allow(dead_code)]
fn link_dir(cef_path: path::PathBuf, file: &str) {
  _link(cef_path, file, true);
}

#[allow(dead_code)]
fn link(cef_path: path::PathBuf, file: &str) {
  _link(cef_path, file, false);
}

#[allow(dead_code)]
fn _link(mut cef_path: path::PathBuf, file: &str, dir: bool) {
  let profile = env::var("PROFILE").unwrap();
  let mut out_path = path::PathBuf::from("target");
  out_path.push(profile);

  cef_path.push(file);
  out_path.push(file);
  //std::fs::remove_file(&out_path).ok();
  do_link(cef_path, out_path, dir);
}

#[allow(dead_code)]
fn link_gen(mut src: path::PathBuf, mut dst: path::PathBuf, file: &str) {
  src.push(file);
  dst.push(file);
  //std::fs::remove_file(&out_path).ok();
  do_link(src, dst, false);
}

#[cfg(windows)]
fn do_link(cef_path: path::PathBuf, out_path: path::PathBuf, dir: bool) {
  if dir {
    std::os::windows::fs::symlink_dir(cef_path, out_path).ok();
  } else {
    std::os::windows::fs::symlink_file(cef_path, out_path).ok();
  }
}

#[cfg(unix)]
fn do_link(cef_path: path::PathBuf, out_path: path::PathBuf, _: bool) {
    std::os::unix::fs::symlink(cef_path, out_path).ok();
}

#[cfg(target_os = "windows")]
fn gen_os(cef_path: path::Display) {
  let _ = generator(cef_path)
    .header("cef_win.h")
    .whitelisted_type("_cef_main_args_t")
    .whitelisted_type("_cef_window_info_t")
    .hide_type(".*string.*")
    .raw_line("use cef::cef_string_t;")
    .generate().expect("Failed to gencef win")
    .write_to_file(path::Path::new("src").join("cef").join("win.rs"));
}

#[cfg(target_os = "linux")]
fn gen_os(cef_path: path::Display) {
  let _ = generator(cef_path)
    .header("cef_linux.h")
    .whitelisted_type("_cef_main_args_t")
    .whitelisted_type("_cef_window_info_t")
    .generate().expect("Failed to gencef linux")
    .write_to_file(path::Path::new("src").join("cef").join("linux.rs"));
}

#[cfg(target_os = "macos")]
fn gen_os(cef_path: path::Display) {
  let _ = generator(cef_path)
    .header("cef_mac.h")
    .whitelisted_type("_cef_main_args_t")
    .whitelisted_type("_cef_window_info_t")
    .hide_type(".*string.*")
    .raw_line("use cef::cef_string_t;")
    .generate().expect("Failed to gencef mac")
    .write_to_file(path::Path::new("src").join("cef").join("mac.rs"));
}

#[allow(dead_code)]
fn gen_cef(cef_path: path::Display) {
  let _ = generator(cef_path)
    .header("cef.h")
    .whitelisted_type("cef_string_t")
    .whitelisted_type(".*cef_base_t")
    .whitelisted_type("_cef_scheme_registrar_t")
    .whitelisted_type("_cef_.*_handler_t")
    .whitelisted_function("cef_string_.*")
    .whitelisted_function("cef_execute_process")
    .whitelisted_function("cef_initialize")
    .whitelisted_function("cef_run_message_loop")
    .whitelisted_function("cef_shutdown")
    .whitelisted_function("cef_browser_host_create_browser")
    .whitelisted_function("cef_.*")
    .hide_type("_cef_main_args_t")
    .hide_type("_cef_window_info_t")
    .raw_line("#[cfg(target_os = \"linux\")] pub mod linux;")
    .raw_line("#[cfg(target_os = \"linux\")] pub use self::linux::_cef_window_info_t;")
    .raw_line("#[cfg(target_os = \"linux\")] pub use self::linux::_cef_main_args_t;")
    .raw_line("#[cfg(target_os = \"macos\")] pub mod mac;")
    .raw_line("#[cfg(target_os = \"macos\")] pub use self::mac::_cef_window_info_t;")
    .raw_line("#[cfg(target_os = \"macos\")] pub use self::mac::_cef_main_args_t;")
    .raw_line("#[cfg(windows)] pub mod win;")
    .raw_line("#[cfg(windows)] pub use self::win::_cef_window_info_t;")
    .raw_line("#[cfg(windows)] pub use self::win::_cef_main_args_t;")
    .generate().expect("Failed to gencef")
    .write_to_file(path::Path::new("src").join("cef").join("mod.rs"));
}

fn generator(cef_path: path::Display) -> bindgen::Builder {
  let config = bindgen::CodegenConfig {
            functions: true,
            types: true,
            vars: false,
            methods: true,
            constructors: false,
            destructors: false
        };
  let gen = bindgen::builder()
    .clang_arg(format!("-I{}", cef_path))
    .clang_arg(format!("-I{}", "C:\\Program Files (x86)\\Microsoft SDKs\\Windows\\v7.1A\\Include"))
    .clang_arg("-Wno-nonportable-include-path")
    .clang_arg("-Wno-invalid-token-paste")
    .link("cef")
    //.use_core()
    .with_codegen_config(config)
    .no_unstable_rust()
    .raw_line("#![allow(dead_code)]")
    .raw_line("#![allow(non_snake_case)]")
    .raw_line("#![allow(non_camel_case_types)]");
  gen
} 

#[allow(dead_code)]
fn gen_gtk() {
  //let out_dir = env::var("OUT_DIR").unwrap();
  let config = bindgen::CodegenConfig {
            functions: true,
            types: true,
            vars: true,
            methods: true,
            constructors: true,
            destructors: true
        };
  let _ = bindgen::builder()
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
    .no_unstable_rust()
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
    .write_to_file(path::Path::new("src").join("gtk2.rs"));
}