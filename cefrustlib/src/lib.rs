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
    print!("XUI error received");
    0
}

#[no_mangle]
pub extern fn init(hwnd: u64) -> *const app::App {
    println!("DLL init");

    println!("hwnd1: {}", hwnd);
    println!("sizeof: {}", std::mem::size_of::<app::App>());

    let main_args = cefrust::prepare_args();

    unsafe { xlib::XSetErrorHandler(Option::Some(xerror_handler_impl)) };
    unsafe { xlib::XSetIOErrorHandler(Option::Some(xioerror_handler_impl)) };

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
    let app = app::App::new(hwnd);
    println!("hwnd2: {:?}", app.canvas_hwnd);
    let app_box = Box::new(app);
    let app_raw = Box::into_raw(app_box);
    println!("Calling cef_initialize");
    unsafe { cef::cef_initialize(&main_args, &settings, &mut (*app_raw).cef_app, std::ptr::null_mut()) };
    //unsafe { cef::cef_initialize(&main_args, &settings, &mut app.cef_app, std::ptr::null_mut()) };
 
    unsafe{ (*app_raw).create_browser() };
    //unsafe{ app.create_browser() };

    //println!("Calling cef_run_message_loop");
    // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
    //unsafe { cef::cef_run_message_loop() };

    //unsafe { cef::cef_shutdown() };
    app_raw
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
pub extern fn resized(app: *const app::App, width: i32, height: i32) {
    //println!("Calling resized {}:{}", width, height);
    
    //println!("hwnd: {}", unsafe { (*app).canvas_hwnd });
    //println!("app: {:?}", unsafe { (*app).cef_app });
    let browser: *mut cef::cef_browser_t = unsafe { (*app).browser.expect("NO BROWSER IN APP") };
    //println!("Calling resized1");
    let get_host_fn = unsafe { (*browser).get_host.expect("No host") };
    //println!("Calling resized2");
    let browser_host = unsafe { get_host_fn(browser) };
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
