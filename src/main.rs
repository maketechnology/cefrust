//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
extern crate x11;

mod cef;
mod base;
mod app;
mod client;
mod gtk2;

use std::ffi;
//use std::os::ext::ffi::OsStrExt;
use std::os::unix::ffi::OsStrExt;
use std::os::raw;
use std::option::Option;
use std::env;
//use std::path::Path;

use x11::xlib;

unsafe extern fn xerror_handler_impl(_: *mut xlib::Display, event: *mut xlib::XErrorEvent) -> raw::c_int {
    print!("X error received: ");
    println!("type {}, serial {}, error_code {}, request_code {}, minor_code {}", 
        (*event).type_, (*event).serial, (*event).error_code, (*event).request_code, (*event).minor_code);
    0
}

unsafe extern fn xioerror_handler_impl(_: *mut xlib::Display) -> raw::c_int {
    print!("XUI error received");
    0
}

fn prepare_args() -> cef::_cef_main_args_t {
    let argv:Vec<ffi::CString> = std::env::args_os().map(|arg| {
        //println!("argv: {:?}", arg);     
        //ffi::CString::new(arg.into_string().unwrap()).unwrap() 
        let osstr:&ffi::OsStr = arg.as_os_str();
        ffi::CString::new(osstr.as_bytes()).unwrap() 
    } ).collect();
    let args:Vec<_> = argv.iter().map(|arg| { 
        //println!("args: {:?}", arg);
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
    let main_args = prepare_args();

    println!("Calling cef_execute_process");
    let exit_code: raw::c_int = unsafe { cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut()) };
    std::process::exit(exit_code);
}

fn cef() {
    let main_args = prepare_args();

    //println!("arg0: {}", argv[0].into_string().unwrap());
    //println!("arg0: {}", args[0]);

    //std::process::exit(0);

    //let mut app = app::new();

    // Execute the sub-process logic, if any. This will either return immediately for the browser
    // process or block until the sub-process should exit
    println!("Calling cef_execute_process");
    let exit_code: raw::c_int = unsafe{
        let exit_code = cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut());
        exit_code
    };
    println!("exit_code: {}", exit_code);

    if exit_code >= 0 {
        // The sub-process terminated, exit now.
        std::process::exit(exit_code);
    }

    unsafe { xlib::XSetErrorHandler(Option::Some(xerror_handler_impl)) };
    unsafe { xlib::XSetIOErrorHandler(Option::Some(xioerror_handler_impl)) };

    //let out_dir = env::var("OUT_DIR").unwrap();
    let cwd_path = env::current_exe().unwrap();
    let cwd = cwd_path.parent().unwrap();
/*
    let mut locales_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let locales_path = cwd.join("locales");
    let locales = locales_path.to_str().unwrap();
    unsafe {cef::cef_string_utf8_to_utf16(locales.as_ptr() as *mut std::os::raw::c_char, locales.len(), &mut locales_cef);}

    let mut resources_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let resources_path = cwd.join("Resources");
    let resources = resources_path.to_str().unwrap();
    unsafe {cef::cef_string_utf8_to_utf16(resources.as_ptr() as *mut std::os::raw::c_char, resources.len(), &mut resources_cef);}
*/
    let resources_cef = empty_str();
    let locales_cef = empty_str();

    let settings = cef::_cef_settings_t {
        size: std::mem::size_of::<cef::_cef_settings_t>(),
        single_process: 1,
        no_sandbox: 1,
        browser_subprocess_path: empty_str(),
        multi_threaded_message_loop: 0,
        external_message_pump: 0,
        windowless_rendering_enabled: 0,
        command_line_args_disabled: 0,
        cache_path: empty_str(),
        user_data_path: empty_str(),
        persist_session_cookies: 0,
        persist_user_preferences: 0,
        user_agent: empty_str(),
        product_version: empty_str(),
        locale: locales_cef,
        log_file: empty_str(),
        log_severity: cef::LOGSEVERITY_INFO,
        javascript_flags: empty_str(),
        resources_dir_path: resources_cef,
        locales_dir_path: empty_str(),
        pack_loading_disabled: 0,
        remote_debugging_port: 0,
        uncaught_exception_stack_size: 0,
        context_safety_implementation: 0,
        ignore_certificate_errors: 0,
        enable_net_security_expiration: 0,
        background_color: 0,
        accept_language_list: empty_str()
    };

    // Initialize CEF in the main process.
    let mut app = app::new();
    
    println!("Calling cef_initialize");
    unsafe { cef::cef_initialize(&main_args, &settings, &mut app, std::ptr::null_mut()) };

    //app::create_browser();

    println!("Calling cef_run_message_loop");
    // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
    unsafe { cef::cef_run_message_loop() };

    //loop {
    //    unsafe { cef::cef_do_message_loop_work() };
    //}

    println!("Calling cef_shutdown");
    // Shut down CEF.
    unsafe { cef::cef_shutdown() };
}

fn empty_str() -> cef::cef_string_t {
    let mut empty_str = cef::cef_string_t {
        str: std::ptr::null_mut(), 
        length: 0, 
        dtor: Option::None
    };
    
    //unsafe { cef::cef_string_utf16_set("".as_ptr(), 0, &mut empty_str, 1) };
    //unsafe { cef::cef_string_utf8_to_utf16("".as_ptr() as *mut std::os::raw::c_char, 0, &mut empty_str);}

    empty_str
}

fn main() {
    cef();
    //subp();

    //initialize_gtk();
    //create_gtk_window(String::from("Hi"), 800, 600);
    
}
