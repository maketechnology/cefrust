//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
mod cef;

use std::ffi;
use std::os::raw;

fn main() {
    let argv:Vec<ffi::CString> = std::env::args_os().map(|arg| { ffi::CString::new(arg.into_string().unwrap()).unwrap() } ).collect();
    let args:Vec<*const ::std::os::raw::c_char> = argv.into_iter().map(|arg| { arg.as_ptr() } ).collect();
    //fuse_main_real(args.len() as c_int, args.as_ptr() as *const *const c_char, .... );
 
    let args = cef::_cef_main_args_t {
        argc : args.len() as std::os::raw::c_int,
        argv : args.as_ptr() as *mut *mut std::os::raw::c_char
    };
    println!("Hello CEF: {}", args.argc);

    unsafe{
        let exit_code = cef::cef_execute_process(&args, std::ptr::null_mut(), std::ptr::null_mut());
        println!("exit_code: {}", exit_code)
    }
}