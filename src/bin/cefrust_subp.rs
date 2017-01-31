extern crate cefrust;

use cefrust::cef;

//use std::ffi;
//use std::os::unix::ffi::OsStrExt;
use std::os::raw;
//use std::env;

fn subp() {
    println!("IN SUBP");
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
