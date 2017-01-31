extern crate cefrust;
extern crate libc;
extern crate x11;

mod app;
mod client;
mod gtk2;

use cefrust::cef;
use x11::xlib;
//use std::env;

unsafe extern fn xerror_handler_impl(_: *mut xlib::Display, event: *mut xlib::XErrorEvent) -> libc::c_int {
    print!("X error received: ");
    println!("type {}, serial {}, error_code {}, request_code {}, minor_code {}", 
        (*event).type_, (*event).serial, (*event).error_code, (*event).request_code, (*event).minor_code);
    0
}

unsafe extern fn xioerror_handler_impl(_: *mut xlib::Display) -> libc::c_int {
    println!("XUI error received");
    0
}

#[no_mangle]
pub extern fn init(japp: *const cef::cef_app_t) -> *const cef::cef_app_t {
    println!("DLL init");

    //println!("sizeof: {}", std::mem::size_of::<app::App>());

    let main_args = cefrust::prepare_args();

    unsafe { xlib::XSetErrorHandler(Option::Some(xerror_handler_impl)) };
    unsafe { xlib::XSetIOErrorHandler(Option::Some(xioerror_handler_impl)) };

    //let subp_path = cwd.join("cefrust_subp");
    //let subp = subp_path.to_str().unwrap();
    //println!("subp: {:?}", subp);

    let locales_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/locales");
    let resources_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/Resources");
    let subp_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/cefrust_subp");
    let logfile_cef = cefrust::cef_string("/home/guille/workspaces/rust/cefrust/target/debug/lib.log");

    let settings = cef::_cef_settings_t {
        size: std::mem::size_of::<cef::_cef_settings_t>(),
        single_process: 0,
        no_sandbox: 1,
        browser_subprocess_path: subp_cef,
        multi_threaded_message_loop: 0,
        external_message_pump: 1,
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
        //log_severity: cef::LOGSEVERITY_VERBOSE,
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
    let mut app = app::new();
    app.get_browser_process_handler = unsafe {(*japp).get_browser_process_handler};

    let app_box = Box::new(app);
    let app_raw = Box::into_raw(app_box);
    println!("Calling cef_initialize");
    unsafe { cef::cef_initialize(&main_args, &settings, &mut (*app_raw), std::ptr::null_mut()) };
    //unsafe { cef::cef_initialize(&main_args, &settings, &mut app.cef_app, std::ptr::null_mut()) };
    app_raw
}

fn str_from_c(cstr: *const libc::c_char) -> &'static str {
    let slice = unsafe { std::ffi::CStr::from_ptr(cstr) };
    let url = std::str::from_utf8(slice.to_bytes()).unwrap();
    url
}

#[no_mangle]
pub extern fn create_browser(hwnd: u64, url: *const libc::c_char, client: &mut cef::_cef_client_t) -> *const cef::cef_browser_t {
    println!("create_browser");

    println!("hwnd: {}", hwnd);
    println!("client: {:?}", client);
    println!("_cef_client_t sizeof: {:?}", std::mem::size_of::<cef::_cef_client_t>());
    println!("_cef_focus_handler_t sizeof: {:?}", std::mem::size_of::<cef::_cef_focus_handler_t>());
 
    //let url = "http://www.google.com";
    //let url = std::ffi::CString::new(url).unwrap().to_str().unwrap();
    let url = str_from_c(url);
    println!("url: {:?}", url);
    //let url = String::from_utf16(url as &[u16]).unwrap();
    let browser = app::create_browser(hwnd, url, client);

    browser
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
pub extern fn resized(browser: *mut cef::cef_browser_t, width: i32, height: i32) {
    //println!("Calling resized {}:{}", width, height);
    
    //println!("hwnd: {}", unsafe { (*app).canvas_hwnd });
    //println!("app: {:?}", unsafe { (*app).cef_app });
    //println!("Calling resized1");
    let browser_host = get_browser_host(browser);
    //println!("Calling resized3");
    //let resized_fn = unsafe { (*browser_host).was_resized.unwrap() };
    //println!("Calling resized4");
    //unsafe {resized_fn(browser_host) };

    //let move_fn = unsafe { (*browser_host).notify_move_or_resize_started.unwrap() };
    //println!("Calling resized5");
    //unsafe {move_fn(browser_host) };
    //println!("Calling resized3");
    let get_window_handle_fn = unsafe { (*browser_host).get_window_handle.expect("no get_window_handle") };
    //println!("Calling resized4");
    let win_handle = unsafe { get_window_handle_fn(browser_host) };
    //println!("win_handle: {:?}", win_handle);
    //println!("vbox_hwnd: {:?}", unsafe{(*app).vbox_hwnd});
    
    //let vbox = unsafe {gtk2::gtk_widget_get_window(win_handle as *mut libc::c_void) };
    //println!("vbox: {:?}", vbox);
    //unsafe { gtk2::gtk_widget_set_size_request((*app).vbox_hwnd as *mut libc::c_void, width, height) };

    //unsafe { gtk2::gtk_widget_set_size_request(vbox, width, height) };

    /*let mut allocation = gtk2::GtkAllocation {
        width: 0,
        height: 0,
        x: 0,
        y: 0
    };*/
    //unsafe { gtk2::gtk_widget_get_allocation((*app).canvas_hwnd as *mut libc::c_void, &mut allocation) };
    //println!("rs canvas: {}, {}", allocation.width, allocation.height);
    
    //unsafe { gtk2::gtk_widget_get_allocation((*app).vbox_hwnd as *mut libc::c_void, &mut allocation) };
    //println!("rs vbox: {}, {}", allocation.width, allocation.height);

    //gtk_widget_show_all(GTK_WIDGET(canvas)); 
    //let gtk_win = unsafe { gtk2::gdk_window_lookup(win_handle) };
    //println!("win: {:?}", gtk_win);
    //unsafe { gtk2::gdk_window_resize(gtk_win, width, height) };
    //unsafe { gtk2::gtk_widget_set_size_request((*app).vbox_hwnd as *mut libc::c_void, width, height) };
    let xwindow = win_handle;
    let xdisplay = unsafe { cef::cef_get_xdisplay() };
    let mut changes = xlib::XWindowChanges {
        x: 0,
        y: 0,
        width: width,
        height: height,
        border_width: 0,
        sibling: 0,
        stack_mode: 0
    };
    unsafe { xlib::XConfigureWindow(std::mem::transmute(xdisplay), xwindow,
        (xlib::CWX | xlib::CWY | xlib::CWHeight | xlib::CWWidth) as u32, &mut changes) };
}

#[no_mangle]
pub extern fn try_close_browser(browser: *mut cef::cef_browser_t) {
    let browser_host = get_browser_host(browser);
    let close_fn = unsafe { (*browser_host).close_browser.expect("null try_close_browser") };
    unsafe { close_fn(browser_host, 1) };
}


#[no_mangle]
pub extern fn load_url(browser: *mut cef::cef_browser_t, url: *const libc::c_char) {
    let url = str_from_c(url);
    let url_cef = cefrust::cef_string(url);
    println!("url: {:?}", url);
    let get_frame = unsafe { (*browser).get_main_frame.expect("null get_main_frame") };
    let main_frame = unsafe { get_frame(browser) };
    let load_url = unsafe { (*main_frame).load_url.expect("null load_url") };
    unsafe { load_url(main_frame, &url_cef) };
}

#[no_mangle]
pub extern fn set_focus(browser: *mut cef::cef_browser_t, set: bool, parent: *mut libc::c_void) {
    let browser_host = get_browser_host(browser);
    let focus_fn = unsafe { (*browser_host).set_focus.expect("null set_focus") };
    let focus = if set {
        1
    } else {
        0
    };
    println!("<<<<<<<< set_focus {}", focus);
    unsafe { focus_fn(browser_host, focus) };
    if !set && parent as u64 != 0 {
        let root = unsafe { gtk2::gtk_widget_get_toplevel(parent) };
        println!("<<<<<<<< set_focus {} {:?} {:?}", focus, parent, root);
        // workaround to actually remove focus from cef inputs
        unsafe { gtk2::gtk_window_present(root) };
    }
}

#[no_mangle]
pub extern fn shutdown() {
    println!("Calling cef_shutdown");
    // Shut down CEF.
    unsafe { cef::cef_shutdown() };
}

fn get_browser_host(browser: *mut cef::cef_browser_t) -> *mut cef::_cef_browser_host_t {
    let get_host_fn = unsafe { (*browser).get_host.expect("null get_host") };
    let browser_host = unsafe { get_host_fn(browser) };
    browser_host
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
