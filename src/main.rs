//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
//extern crate x11;
extern crate cefrust;

use cefrust::cef;

//mod cef;
//mod base;
mod app;
mod client;
mod gtk2;

//use std::os::raw;
//use std::option::Option;
use std::env;
//use std::path::Path;
use std::str::FromStr;

//use x11::xlib;
/*
unsafe extern fn xerror_handler_impl(_: *mut xlib::Display, event: *mut xlib::XErrorEvent) -> raw::c_int {
    print!("X error received: ");
    println!("type {}, serial {}, error_code {}, request_code {}, minor_code {}", 
        (*event).type_, (*event).serial, (*event).error_code, (*event).request_code, (*event).minor_code);
    0
}

unsafe extern fn xioerror_handler_impl(_: *mut xlib::Display) -> raw::c_int {
    print!("XUI error received");
    0
}*/

fn cef() {
    //let main_args = cefrust::prepare_args();

    //std::process::exit(0);

    //let mut app = app::new();

    // Execute the sub-process logic, if any. This will either return immediately for the browser
    // process or block until the sub-process should exit
    // println!("Calling cef_execute_process");
    // let exit_code: raw::c_int = unsafe { cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut()) };
    // println!("exit_code: {}", exit_code);
    // if exit_code >= 0 {
    //     // The sub-process terminated, exit now.
    //     std::process::exit(exit_code);
    // }

    //unsafe { xlib::XSetErrorHandler(Option::Some(xerror_handler_impl)) };
    //unsafe { xlib::XSetIOErrorHandler(Option::Some(xioerror_handler_impl)) };

    //let out_dir = env::var("OUT_DIR").unwrap();
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
    let cwd_path = env::current_exe().unwrap();
    let cwd = cwd_path.parent().unwrap();
    let subp = cefrust::subp_path(cwd);
    let subp_cef = cefrust::cef_string(&subp);

    let resources_cef = cefrust::cef_string_empty();
    let locales_cef = cefrust::cef_string_empty();

    let settings = cef::_cef_settings_t {
        size: std::mem::size_of::<cef::_cef_settings_t>(),
        single_process: 0,
        no_sandbox: 1,
        browser_subprocess_path: subp_cef,
        //browser_subprocess_path: cefrust::cef_string_empty(),
        multi_threaded_message_loop: 0,
        external_message_pump: 0,
        windowless_rendering_enabled: 0,
        command_line_args_disabled: 0,
        cache_path: cefrust::cef_string_empty(),
        user_data_path: cefrust::cef_string_empty(),
        persist_session_cookies: 0,
        persist_user_preferences: 0,
        user_agent: cefrust::cef_string_empty(),
        product_version: cefrust::cef_string_empty(),
        locale: cefrust::cef_string_empty(),
        log_file: cefrust::cef_string_empty(),
        log_severity: cef::LOGSEVERITY_DEFAULT,
        javascript_flags: cefrust::cef_string_empty(),
        resources_dir_path: resources_cef,
        locales_dir_path: locales_cef,
        pack_loading_disabled: 0,
        remote_debugging_port: 0,
        uncaught_exception_stack_size: 100,
        context_safety_implementation: 0,
        ignore_certificate_errors: 0,
        enable_net_security_expiration: 0,
        background_color: 0,
        accept_language_list: cefrust::cef_string_empty()
    };

    let hwnd = std::env::args().nth(1);
    let hwnd = hwnd.unwrap_or("0".to_string());
    //let hwnd: usize = usize::from_str(&hwnd).unwrap();
    let hwnd = std::os::raw::c_ulong::from_str(&hwnd).unwrap();
    println!("main hwnd: {}", hwnd);

    // Initialize CEF in the main process.
    let mut app = app::new(hwnd);
    
    //let ten_millis = std::time::Duration::from_millis(3000);
    //std::thread::sleep(ten_millis);
    let main_args = cefrust::prepare_args();
    //std::mem::forget(main_args);
    println!("Calling cef_initialize");
    unsafe { cef::cef_initialize(&main_args, &settings, &mut app, std::ptr::null_mut()) };
    //let ten_millis = std::time::Duration::from_millis(3000);
    //let now = std::time::Instant::now();
    //std::thread::sleep(ten_millis);
    //assert!(now.elapsed() >= ten_millis);

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

fn main() {
    cef();
    //subp();

    //initialize_gtk();
    //create_gtk_window(String::from("Hi"), 800, 600);
    
}
