//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
mod cef;

use std::ffi;
//use std::os::ext::ffi::OsStrExt;
use std::os::unix::ffi::OsStrExt;
use std::os::raw;
use std::option::Option;

fn main() {
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
    unsafe { println!("Hello CEF, ARGS: {:?}", *args[0]) };

    let main_args = cef::_cef_main_args_t {
        argc : args.len() as std::os::raw::c_int,
        argv : args_ptr as *mut *mut std::os::raw::c_char
    };
    println!("Hello CEF, ARGS: {}", main_args.argc);

    //println!("arg0: {}", argv[0].into_string().unwrap());
    //println!("arg0: {}", args[0]);

    //std::process::exit(0);

    // Execute the sub-process logic, if any. This will either return immediately for the browser
    // process or block until the sub-process should exit
    println!("Calling cef_execute_process");
    let exit_code:raw::c_int = unsafe{
        let exit_code = cef::cef_execute_process(&main_args, std::ptr::null_mut(), std::ptr::null_mut());
        exit_code
    };
    println!("exit_code: {}", exit_code);

    if exit_code >= 0 {
        // The sub-process terminated, exit now.
        std::process::exit(exit_code);
    }

    let mut empty_str = cef::cef_string_t {
        str: std::ptr::null_mut(), 
        length: 0, 
        dtor: Option::None
    };
    //cef::cef_string_utf16_set("", 0, empty_str, true);
    unsafe {
        cef::cef_string_utf8_to_utf16("".as_ptr() as *mut std::os::raw::c_char, 0, &mut empty_str);
    }

    let mut locales_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let locales = "/home/gzunino/workspaces/rust/cefrust/target/debug/locales";
    unsafe {cef::cef_string_utf8_to_utf16(locales.as_ptr() as *mut std::os::raw::c_char, locales.len(), &mut locales_cef);}

    let mut resources_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let resources = "/home/gzunino/workspaces/rust/cefrust/target/debug/Resources";
    unsafe {cef::cef_string_utf8_to_utf16(resources.as_ptr() as *mut std::os::raw::c_char, resources.len(), &mut resources_cef);}

    let settings = cef::_cef_settings_t {
        size: 344usize,
        single_process: 0,
        no_sandbox: 1,
        browser_subprocess_path: empty_str,
        multi_threaded_message_loop: 0,
        external_message_pump: 0,
        windowless_rendering_enabled: 0,
        command_line_args_disabled: 0,
        cache_path: empty_str,
        user_data_path: empty_str,
        persist_session_cookies: 1,
        persist_user_preferences: 1,
        user_agent: empty_str,
        product_version: empty_str,
        locale: locales_cef,
        log_file: empty_str,
        log_severity: cef::_bindgen_ty_3::LOGSEVERITY_VERBOSE,
        javascript_flags: empty_str,
        resources_dir_path: resources_cef,
        locales_dir_path: empty_str,
        pack_loading_disabled: 0,
        remote_debugging_port: 0,
        uncaught_exception_stack_size: 0,
        context_safety_implementation: 0,
        ignore_certificate_errors: 0,
        enable_net_security_expiration: 0,
        background_color: 0,
        accept_language_list: empty_str
    };

    let app_base = cef::cef_base_t {
        size: std::mem::size_of::<cef::cef_app_t>(),
        add_ref: Option::None,
        release: Option::None,
        has_one_ref: Option::None
    };

    unsafe extern "C" fn context_initialized_fn(self_: *mut _cef_browser_process_handler_t) {
        
    };

    unsafe extern "C" fn bph_fn(self_: *mut cef::cef_app_t) -> *mut cef::_cef_browser_process_handler_t {
        println!("In get_browser_process_handler");

        let mut bph = cef::_cef_browser_process_handler_t {
            base: cef::cef_base_t {
                size: std::mem::size_of::<cef::_cef_browser_process_handler_t>(),
                add_ref: Option::None,
                release: Option::None,
                has_one_ref: Option::None
            },
            on_context_initialized: Option::Some(context_initialized_fn),
            on_before_child_process_launch: Option::None,
            on_render_process_thread_created: Option::None,
            get_print_handler: Option::None,
            on_schedule_message_pump_work: Option::None
        };
        &mut bph
    };

    //let bph = bph_fn;

    // Initialize CEF in the main process.
    let mut app = cef::cef_app_t {
        base: app_base,
        on_before_command_line_processing: Option::None,
        on_register_custom_schemes: Option::None,
        get_resource_bundle_handler: Option::None,
        get_browser_process_handler: Option::Some(bph_fn),
        get_render_process_handler: Option::None
    };

    unsafe {
        println!("Calling cef_initialize");
        cef::cef_initialize(&main_args, &settings, &mut app, std::ptr::null_mut());
        
        println!("Calling cef_run_message_loop");
        // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
        cef::cef_run_message_loop();

        println!("Calling cef_shutdown");
        // Shut down CEF.
        cef::cef_shutdown();
    }
}