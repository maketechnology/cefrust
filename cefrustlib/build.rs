fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=gtk-x11-2.0");
        println!("cargo:rustc-link-lib=gdk-x11-2.0");
        println!("cargo:rustc-link-lib=X11");
    }
}
