//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
mod cef;

use std::ffi;
use std::os::raw;

fn main() {
    let argv:Vec<ffi::CString> = std::env::args_os().map(|arg| { ffi::CString::new(arg.into_string().unwrap()).unwrap() } ).collect();
    let args:Vec<*const ::std::os::raw::c_char> = argv.into_iter().map(|arg| { arg.as_ptr() } ).collect();
    //fuse_main_real(args.len() as c_int, args.as_ptr() as *const *const c_char, .... );
 
    // Structure for passing command-line arguments.
    // The definition of this structure is platform-specific.
    let main_args = cef::_cef_main_args_t {
        argc : args.len() as std::os::raw::c_int,
        argv : args.as_ptr() as *mut *mut std::os::raw::c_char
    };
    println!("Hello CEF: {}", main_args.argc);

    // Execute the sub-process logic, if any. This will either return immediately for the browser
    // process or block until the sub-process should exit
    let exit_code:raw::c_int = unsafe{
        let exit_code = cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut());
        exit_code
    };
    println!("exit_code: {}", exit_code);

    if exit_code >= 0 {
        // The sub-process terminated, exit now.
        std::process::exit(exit_code);
    }

    let settings = cef::_cef_settings_t {
        size: 344usize,
        single_process: 0,
        no_sandbox: 1,
        browser_subprocess_path: std::ptr::null(),
        multi_threaded_message_loop: 0,
        external_message_pump: 0,
        windowless_rendering_enabled: 0,
        command_line_args_disabled: 0,
        cache_path: std::ptr::null_mut(),
        user_data_path: std::ptr::null_mut(),
        persist_session_cookies: 1,
        persist_user_preferences: 1,
        user_agent: std::ptr::null_mut(),
        product_version: std::ptr::null_mut(),
        locale: std::ptr::null_mut(),
        log_file: std::ptr::null_mut(),
        log_severity: cef::_bindgen_ty_3::LOGSEVERITY_VERBOSE,
        javascript_flags: std::ptr::null_mut(),
        resources_dir_path: std::ptr::null_mut(),
        locales_dir_path: std::ptr::null_mut(),
        pack_loading_disabled: 0,
        remote_debugging_port: 0,
        uncaught_exception_stack_size: 0,
        context_safety_implementation: 0,
        ignore_certificate_errors: 0,
        enable_net_security_expiration: 0,
        background_color: 0,
        accept_language_list: std::ptr::null_mut()
    };

    // Initialize CEF in the main process.
    let app = std::ptr::null_mut();

    cef::cef_initialize(main_args, settings, app, std::ptr::null_mut());

    // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
    cef::cef_run_message_loop();

    // Shut down CEF.
    cef::cef_shutdown();
}