extern crate cefrust;
#[cfg(target_os = "linux")]
extern crate x11;
extern crate libc;

use cefrust::cef;

//use std::ffi;
//use std::os::unix::ffi::OsStrExt;
use std::os::raw;
use std::env;

#[cfg(target_os = "linux")]
unsafe extern fn xerror_handler_impl(_: *mut x11::xlib::Display, event: *mut x11::xlib::XErrorEvent) -> libc::c_int {
    print!("X error received: ");
    println!("type {}, serial {}, error_code {}, request_code {}, minor_code {}", 
        (*event).type_, (*event).serial, (*event).error_code, (*event).request_code, (*event).minor_code);
    0
}
#[cfg(target_os = "linux")]
unsafe extern fn xioerror_handler_impl(_: *mut x11::xlib::Display) -> libc::c_int {
    println!("XUI error received");
    0
}

fn subp() {
    println!("IN SUBP");
    if cfg!(target_os = "linux") {
        unsafe { x11::xlib::XSetErrorHandler(Option::Some(xerror_handler_impl)) };
        unsafe { x11::xlib::XSetIOErrorHandler(Option::Some(xioerror_handler_impl)) };

        let key = "LD_LIBRARY_PATH";
        //env::set_var(key, "cefrust_path");
        assert_eq!(env::var(key), Ok("/home/guille/.swtcef/3.2785.1486.g8c4ba9f/linux-x86_64".to_string()));
    }

    let main_args = cefrust::prepare_args();
    //std::mem::forget(main_args);
    println!("Calling cef_execute_process");
    //println!("Hello CEF, ARGS: {:?}", main_args.argc);
    let exit_code: raw::c_int = unsafe { cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut()) };
    println!("existing subp with {}", exit_code);
    std::process::exit(exit_code);
}

fn main() {
    subp();
}
