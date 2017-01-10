//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
mod cef;

use std::ffi;
//use std::os::ext::ffi::OsStrExt;
use std::os::unix::ffi::OsStrExt;
use std::os::raw;
use std::option::Option;
use std::env;
use std::path::Path;

unsafe extern "C" fn context_initialized_fn(_: *mut cef::_cef_browser_process_handler_t) {
    println!("In context_initialized_fn");

    let mut empty_str = cef::cef_string_t {
        str: std::ptr::null_mut(), 
        length: 0, 
        dtor: Option::None
    };
    //cef::cef_string_utf16_set("", 0, empty_str, true);
    cef::cef_string_utf8_to_utf16("".as_ptr() as *mut std::os::raw::c_char, 0, &mut empty_str);    
    
    // Create GTK window. You can pass a NULL handle 
    // to CEF and then it will create a window of its own.
//			    initialize_gtk();
//			    GtkWidget* hwnd = create_gtk_window("cefcapi example", 1024, 768);
    //let window_info = std::ptr::null();
    let window_info = cef::_cef_window_info_t {
        x: 50,
        y: 50,
        width: 800,
        height: 600,
        parent_window: 0,
        windowless_rendering_enabled: 0,
        transparent_painting_enabled: 0,
        window: 0,
    };
//			    windowInfo.parent_widget = hwnd;

    // Browser settings.
    // It is mandatory to set the "size" member.
    let browser_settings = cef::_cef_browser_settings_t {
        size: std::mem::size_of::<cef::_cef_browser_settings_t>(),
        windowless_frame_rate: 0,
        standard_font_family: empty_str,
        fixed_font_family: empty_str,
        serif_font_family: empty_str,
        sans_serif_font_family: empty_str,
        cursive_font_family: empty_str,
        fantasy_font_family: empty_str,
        default_font_size: 0,
        default_fixed_font_size: 0,
        minimum_font_size: 0,
        minimum_logical_font_size: 0,
        default_encoding: empty_str,
        remote_fonts: cef::STATE_DEFAULT,
        javascript: cef::STATE_DEFAULT,
        javascript_open_windows: cef::STATE_DEFAULT,
        javascript_close_windows: cef::STATE_DEFAULT,
        javascript_access_clipboard: cef::STATE_DEFAULT,
        javascript_dom_paste: cef::STATE_DEFAULT,
        caret_browsing: cef::STATE_DEFAULT,
        plugins: cef::STATE_DEFAULT,
        universal_access_from_file_urls: cef::STATE_DEFAULT,
        file_access_from_file_urls: cef::STATE_DEFAULT,
        web_security: cef::STATE_DEFAULT,
        image_loading: cef::STATE_DEFAULT,
        image_shrink_standalone_to_fit: cef::STATE_DEFAULT,
        text_area_resize: cef::STATE_DEFAULT,
        tab_to_links: cef::STATE_DEFAULT,
        local_storage: cef::STATE_DEFAULT,
        databases: cef::STATE_DEFAULT,
        application_cache: cef::STATE_DEFAULT,
        webgl: cef::STATE_DEFAULT,
        background_color: 0,
        accept_language_list: empty_str
    };

    // Client handler and its callbacks.
    // cef_client_t structure must be filled. It must implement
    // reference counting. You cannot pass a structure 
    // initialized with zeroes.
    let mut client = cef::_cef_client_t {
        base: cef::cef_base_t {
            size: std::mem::size_of::<cef::_cef_client_t>(),
            add_ref: Option::None,
            release: Option::None,
            has_one_ref: Option::None
        },
        get_context_menu_handler: Option::None,
        get_dialog_handler: Option::None,
        get_display_handler: Option::None,
        get_download_handler: Option::None,
        get_drag_handler: Option::None,
        get_find_handler: Option::None,
        get_focus_handler: Option::None,
        get_geolocation_handler: Option::None,
        get_jsdialog_handler: Option::None,
        get_keyboard_handler: Option::None,
        get_life_span_handler: Option::None,
        get_load_handler: Option::None,
        get_render_handler: Option::None,
        get_request_handler: Option::None,
        on_process_message_received: Option::None
    };

    let mut url_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let url = "http://www.google.com";
    cef::cef_string_utf8_to_utf16(url.as_ptr() as *mut std::os::raw::c_char, url.len(), &mut url_cef);

    // Create browser.
    println!("Calling cef_browser_host_create_browser");
    if cef::cef_browser_host_create_browser(&window_info, &mut client, &url_cef, &browser_settings, std::ptr::null_mut()) != 1 {
        println!("Failed calling browserHostCreateBrowser");
    }
}

static mut bph: cef::_cef_browser_process_handler_t = cef::_cef_browser_process_handler_t {
        base: cef::cef_base_t {
            size: 72usize,
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

unsafe extern "C" fn bph_fn(_: *mut cef::cef_app_t) -> *mut cef::_cef_browser_process_handler_t {
//let bph_fn = |_: *mut cef::cef_app_t| -> *mut cef::_cef_browser_process_handler_t {
    println!("In get_browser_process_handler");

    &mut bph
}

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
        //std::process::exit(exit_code);
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

    //let out_dir = env::var("OUT_DIR").unwrap();
    let cwd_path = env::current_exe().unwrap();
    let cwd = cwd_path.parent().unwrap();

    let mut locales_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let locales_path = cwd.join("locales");
    let locales = locales_path.to_str().unwrap();
    unsafe {cef::cef_string_utf8_to_utf16(locales.as_ptr() as *mut std::os::raw::c_char, locales.len(), &mut locales_cef);}

    let mut resources_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::None};
    let resources_path = cwd.join("Resources");
    let resources = resources_path.to_str().unwrap();
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
        log_severity: cef::LOGSEVERITY_VERBOSE,
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