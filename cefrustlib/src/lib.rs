extern crate cefrust;

mod app;
mod client;
mod gtk2;

use cefrust::cef;
//use std::env;

#[no_mangle]
pub extern fn init(hwnd: u64) {
    println!("DLL init");

    let main_args = cefrust::prepare_args();

    //let subp_path = cwd.join("subprocess");
    //let subp = subp_path.to_str().unwrap();
    //println!("subp: {:?}", subp);

    let locales_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/locales");
    let resources_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/Resources");
    let subp_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/subprocess");
    let logfile_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/lib.log");

    let settings = cef::_cef_settings_t {
        size: std::mem::size_of::<cef::_cef_settings_t>(),
        single_process: 1,
        no_sandbox: 1,
        browser_subprocess_path: subp_cef,
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
        log_file: logfile_cef,
        log_severity: cef::LOGSEVERITY_INFO,
        javascript_flags: cefrust::cef_string_empty(),
        resources_dir_path: resources_cef,
        locales_dir_path: locales_cef,
        pack_loading_disabled: 0,
        remote_debugging_port: 0,
        uncaught_exception_stack_size: 0,
        context_safety_implementation: 0,
        ignore_certificate_errors: 0,
        enable_net_security_expiration: 0,
        background_color: 0,
        accept_language_list: cefrust::cef_string_empty()
    };

    // Initialize CEF in the main process.
    //let mut app = app::new(hwnd);
    let app = Box::new(app::new(hwnd));
    let app = Box::into_raw(app);
    
    println!("Calling cef_initialize");
    unsafe { cef::cef_initialize(&main_args, &settings, app, std::ptr::null_mut()) };

    let browser = app::create_browser();

    //println!("Calling cef_run_message_loop");
    // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
    //unsafe { cef::cef_run_message_loop() };

    //unsafe { cef::cef_shutdown() };
}

#[no_mangle]
pub extern fn do_message_loop_work() {
    //println!("Calling cef_run_message_loop");
    // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
    //unsafe { cef::cef_run_message_loop() };

    //println!("Calling cef_do_message_loop_work");
    unsafe { cef::cef_do_message_loop_work() };
}

#[no_mangle]
pub extern fn shutdown() {
    println!("Calling cef_shutdown");
    // Shut down CEF.
    unsafe { cef::cef_shutdown() };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
