extern crate cefrust;
extern crate libc;
#[cfg(target_os = "linux")]
extern crate x11;
#[cfg(unix)]
extern crate nix;

mod app;
mod client;
#[cfg(target_os = "linux")]
mod gtk2;

use cefrust::cef;

use std::env;
#[cfg(unix)]
use std::collections::HashMap;

#[cfg(target_os = "linux")]
unsafe extern fn xerror_handler_impl(_: *mut x11::xlib::Display, event: *mut x11::xlib::XErrorEvent) -> libc::c_int {
    print!("X error received: ");
    println!("type {}, serial {}, error_code {}, request_code {}, minor_code {}", 
        (*event).type_, (*event).serial, (*event).error_code, (*event).request_code, (*event).minor_code);
    0
}
#[cfg(target_os = "linux")]
unsafe extern fn xioerror_handler_impl(_: *mut x11::xlib::Display) -> libc::c_int {
    println!("XUI error received");
    0
}

#[no_mangle]
pub extern fn init(japp: *const cef::cef_app_t, cefrust_path: *const libc::c_char) -> *const cef::cef_app_t {
    println!("DLL init");

    let cefrust_path = str_from_c(cefrust_path);

    let key = "LD_LIBRARY_PATH";
    env::set_var(key, cefrust_path);

    //println!("sizeof: {}", std::mem::size_of::<app::App>());

    let main_args = cefrust::prepare_args();

    let cefrust_dir = std::path::Path::new(&cefrust_path);

    env::set_current_dir(cefrust_dir).expect("Failed to set current dir");
    println!("{:?}", env::current_dir().unwrap().to_str());

    let subp = cefrust::subp_path(cefrust_dir);
    let subp_cef = cefrust::cef_string(&subp);
    
    //let cefrust_dir = cefrust_dir.to_str().unwrap();
    let locales_cef = cefrust::cef_string(cefrust_dir.join("locales").to_str().unwrap());
    //let resources_cef = cefrust::cef_string(cefrust_dir.join("Resources").to_str().unwrap());
    //let resources_cef = cefrust::cef_string_empty();
    let resources_cef = cefrust::cef_string(cefrust_dir.to_str().unwrap());
    let logfile_cef = cefrust::cef_string(cefrust_dir.join("lib.log").to_str().unwrap());

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
    do_initialize(main_args, settings, app_raw);
    
    app_raw
}

#[cfg(target_os = "linux")]
fn do_initialize(main_args: cef::_cef_main_args_t, settings: cef::_cef_settings_t, app_raw: *mut cefrust::cef::_cef_app_t) {
    unsafe { x11::xlib::XSetErrorHandler(Option::Some(xerror_handler_impl)) };
    unsafe { x11::xlib::XSetIOErrorHandler(Option::Some(xioerror_handler_impl)) };

    let mut signal_handlers: HashMap<libc::c_int, nix::sys::signal::SigAction> = HashMap::new();
    backup_signal_handlers(&mut signal_handlers);
    
    unsafe { cef::cef_initialize(&main_args, &settings, &mut (*app_raw), std::ptr::null_mut()) };

    restore_signal_handlers(signal_handlers);
}

#[cfg(target_os = "macos")]
fn do_initialize(main_args: cef::_cef_main_args_t, settings: cef::_cef_settings_t, app_raw: *mut cefrust::cef::_cef_app_t) {
    let mut signal_handlers: HashMap<libc::c_int, nix::sys::signal::SigAction> = HashMap::new();
    backup_signal_handlers(&mut signal_handlers);
    
    unsafe { cef::cef_initialize(&main_args, &settings, &mut (*app_raw), std::ptr::null_mut()) };

    restore_signal_handlers(signal_handlers);
}

#[cfg(target_os = "windows")]
fn do_initialize(main_args: cef::_cef_main_args_t, settings: cef::_cef_settings_t, app_raw: *mut cefrust::cef::_cef_app_t) {
    unsafe { cef::cef_initialize(&main_args, &settings, &mut (*app_raw), std::ptr::null_mut()) };
}

#[cfg(unix)]
fn backup_signal_handlers(signal_handlers: &mut HashMap<libc::c_int, nix::sys::signal::SigAction>) {
    use nix::sys::signal;
    let signals_to_restore = [signal::SIGHUP, signal::SIGINT, signal::SIGQUIT, signal::SIGILL, 
        signal::SIGABRT, signal::SIGFPE, signal::SIGSEGV, signal::SIGALRM, signal::SIGTERM, 
        signal::SIGCHLD, signal::SIGBUS, signal::SIGTRAP, signal::SIGPIPE];
    
    for signal in &signals_to_restore {
        //let sigact: *mut libc::sigaction = unsafe { std::mem::zeroed() };
        //let sigact: libc::sigaction = libc::sigaction {};
        //unsafe { libc::sigaction(*signal, std::ptr::null(), sigact) };
        //println!("backup signal {:?}:{:?}", *signal, sigact);
        //signal_handlers.insert(*signal, sigact);
        let sig_action = signal::SigAction::new(signal::SigHandler::SigDfl,
                                          signal::SaFlags::empty(),
                                          signal::SigSet::empty());
        let oldsigact = unsafe { signal::sigaction(*signal, &sig_action) };
        println!("backup signal {:?}:{:?}", signal, "oldsigact.ok()");
        signal_handlers.insert(*signal as libc::c_int, oldsigact.unwrap());
    }
}

#[cfg(unix)]
fn restore_signal_handlers(signal_handlers: HashMap<libc::c_int, nix::sys::signal::SigAction>) {
    use nix::sys::signal;
    for (signal, sigact) in signal_handlers {
        println!("restore signal {:?}:{:?}", signal, "sigact");
        //unsafe { libc::sigaction(signal, sigact, std::ptr::null_mut()) };
        unsafe { signal::sigaction(std::mem::transmute(signal), &sigact).unwrap() };
    }
}

fn str_from_c(cstr: *const libc::c_char) -> &'static str {
    let slice = unsafe { std::ffi::CStr::from_ptr(cstr) };
    let url = std::str::from_utf8(slice.to_bytes()).unwrap();
    url
}

#[no_mangle]
pub extern fn create_browser(hwnd: std::os::raw::c_ulong, url: *const libc::c_char, client: &mut cef::_cef_client_t) -> *const cef::cef_browser_t {
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
    do_resize(win_handle, width, height);
}

#[cfg(target_os = "linux")]
fn do_resize(win_handle: std::os::raw::c_ulong, width: i32, height: i32) {
    use x11::xlib;

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

#[cfg(target_family = "windows")]
fn do_resize(win_handle: std::os::raw::c_ulong, width: i32, height: i32) {
    extern crate winapi;
    extern crate user32;

    let x = 0;
    let y = 0;
    unsafe { user32::SetWindowPos(win_handle as *mut winapi::HWND__, std::ptr::null_mut(), x, y,
                 width, height,
                 winapi::winuser::SWP_NOZORDER) };
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
    if !set && parent as std::os::raw::c_ulong != 0 {
        do_set_focus(parent, focus);
    }
}

#[cfg(target_os = "linux")]
fn do_set_focus(parent: *mut libc::c_void, focus: i32) {
    let root = unsafe { gtk2::gtk_widget_get_toplevel(parent) };
    println!("<<<<<<<< set_focus {} {:?} {:?}", focus, parent, root);
    // workaround to actually remove focus from cef inputs
    unsafe { gtk2::gtk_window_present(root) };
}

#[cfg(target_family = "windows")]
fn do_set_focus(parent: *mut libc::c_void, focus: i32) {
    // TODO
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
