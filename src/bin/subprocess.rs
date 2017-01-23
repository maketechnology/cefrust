extern crate cefrust;

use cefrust::cef;

use std::ffi;
use std::os::unix::ffi::OsStrExt;
use std::os::raw;
//use std::env;

fn prepare_args() -> cef::_cef_main_args_t {
    let argv:Vec<ffi::CString> = std::env::args_os().map(|arg| {
        //println!("argv: {:?}", arg);     
        //ffi::CString::new(arg.into_string().unwrap()).unwrap() 
        let osstr:&ffi::OsStr = arg.as_os_str();
        ffi::CString::new(osstr.as_bytes()).unwrap() 
    } ).collect();
    let args:Vec<_> = argv.iter().map(|arg| { 
        println!("args: {:?}", arg);
        arg.as_ptr() 
    } ).collect();
    //fuse_main_real(args.len() as c_int, args.as_ptr() as *const *const c_char, .... );
 
    // Structure for passing command-line arguments.
    // The definition of this structure is platform-specific.
    let args_ptr = args.as_ptr();

    let main_args = cef::_cef_main_args_t {
        argc : args.len() as raw::c_int,
        argv : args_ptr as *mut *mut raw::c_char
    };
    println!("Hello CEF, ARGS: {}", main_args.argc);

    main_args
}

fn subp() {
    println!("IN SUBP");
    let main_args = prepare_args();
    println!("Calling cef_execute_process");
    let exit_code: raw::c_int = unsafe { cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut()) };
    std::process::exit(exit_code);
}

fn main() {
    subp();
}
