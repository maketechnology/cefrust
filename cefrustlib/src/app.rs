extern crate cefrust;
extern crate libc;

use cefrust::cef;

use cefrust::base;
use client;
#[cfg(target_os = "linux")]
use gtk2;

use std;

pub fn new() -> cef::cef_app_t {
    let cef_app = cef::cef_app_t {
        base: base::CefBase::new(std::mem::size_of::<cef::cef_app_t>()),
        on_before_command_line_processing: Option::None,
        //on_before_command_line_processing: Option::Some(on_before_command_line_processing),
        on_register_custom_schemes: Option::None,
        //on_register_custom_schemes: Option::Some(on_register_custom_schemes),
        get_resource_bundle_handler: Option::None,
        //get_resource_bundle_handler: Option::Some(get_resource_bundle_handler),
        get_browser_process_handler: Option::None,
        //get_browser_process_handler: Option::Some(get_browser_process_handler),
        get_render_process_handler: Option::None,
        //get_render_process_handler: Option::Some(get_render_process_handler)
    };
    cef_app
}

pub unsafe extern "C" fn window_focus_in(widget: *mut libc::c_void, event: *mut libc::c_void, data: *mut libc::c_void) -> libc::c_int {
    println!(">>>>>>>> IN window_focus_in");
  /*if (event->in && self->browser_window_.get()) {
    self->browser_window_->SetFocus(true);
    // Return true for a windowed browser so that focus is not passed to GTK.
    return self->with_osr_ ? FALSE : TRUE;
  }

  return FALSE;*/
  1
}

pub fn create_browser(canvas_hwnd: std::os::raw::c_ulong, url: &str, jclient: &mut cef::_cef_client_t) -> *const cef::cef_browser_t {
        // Create GTK window. You can pass a NULL handle 
    // to CEF and then it will create a window of its own.
    println!("create_browser in {}", canvas_hwnd);


    //let vbox = unsafe { gtk2::gtk_vbox_new(0, 0) };
    //unsafe { gtk2::gtk_fixed_put(canvas_hwnd, vbox, 0, 0) };
    //println!("vbox {}", vbox);

    //let event = std::ffi::CString::new("focus-in-event").unwrap();
    //unsafe { gtk2::g_signal_connect_data(vbox as *mut libc::c_void, event.as_ptr(), 
    //    Option::Some(window_focus_in), std::ptr::null_mut(), Option::None, gtk2::G_CONNECT_SWAPPED) };

    let window_info = cef_window_info(canvas_hwnd);
    //println!("parent {:?}", window_info.parent_window);
    //self.vbox_hwnd = vbox;

    // Browser settings.
    // It is mandatory to set the "size" member.
    let browser_settings = cef::_cef_browser_settings_t {
        size: std::mem::size_of::<cef::_cef_browser_settings_t>(),
        windowless_frame_rate: 0,
        standard_font_family: cefrust::cef_string_empty(),
        fixed_font_family: cefrust::cef_string_empty(),
        serif_font_family: cefrust::cef_string_empty(),
        sans_serif_font_family: cefrust::cef_string_empty(),
        cursive_font_family: cefrust::cef_string_empty(),
        fantasy_font_family: cefrust::cef_string_empty(),
        default_font_size: 0,
        default_fixed_font_size: 0,
        minimum_font_size: 0,
        minimum_logical_font_size: 0,
        default_encoding: cefrust::cef_string_empty(),
        remote_fonts: cef::cef_state_t::STATE_DEFAULT,
        javascript: cef::cef_state_t::STATE_DEFAULT,
        javascript_open_windows: cef::cef_state_t::STATE_DEFAULT,
        javascript_close_windows: cef::cef_state_t::STATE_DEFAULT,
        javascript_access_clipboard: cef::cef_state_t::STATE_DEFAULT,
        javascript_dom_paste: cef::cef_state_t::STATE_DEFAULT,
        plugins: cef::cef_state_t::STATE_DEFAULT,
        universal_access_from_file_urls: cef::cef_state_t::STATE_DEFAULT,
        file_access_from_file_urls: cef::cef_state_t::STATE_DEFAULT,
        web_security: cef::cef_state_t::STATE_DEFAULT,
        image_loading: cef::cef_state_t::STATE_DEFAULT,
        image_shrink_standalone_to_fit: cef::cef_state_t::STATE_DEFAULT,
        text_area_resize: cef::cef_state_t::STATE_DEFAULT,
        tab_to_links: cef::cef_state_t::STATE_DEFAULT,
        local_storage: cef::cef_state_t::STATE_DEFAULT,
        databases: cef::cef_state_t::STATE_DEFAULT,
        application_cache: cef::cef_state_t::STATE_DEFAULT,
        webgl: cef::cef_state_t::STATE_DEFAULT,
        background_color: 0,
        accept_language_list: cefrust::cef_string_empty()
    };

    // Client handler and its callbacks.
    // cef_client_t structure must be filled. It must implement
    // reference counting. You cannot pass a structure 
    // initialized with zeroes.
    //let mut client = client::new();
    //client.get_focus_handler = jclient.get_focus_handler;
    //client.get_life_span_handler = jclient.get_life_span_handler;

    //let client = Box::new(client);
    //let client = Box::into_raw(client);

    let url_cef = cefrust::cef_string(url);

    // Create browser.
    println!("Calling cef_browser_host_create_browser");
    //if unsafe { cef::cef_browser_host_create_browser(&window_info, client, &url_cef, &browser_settings, std::ptr::null_mut()) } != 1 {
        //println!("Failed calling browserHostCreateBrowser");
    //}
    let browser: *mut cef::cef_browser_t = unsafe { cef::cef_browser_host_create_browser_sync(&window_info, jclient, &url_cef, &browser_settings, std::ptr::null_mut()) };
    assert_eq!(unsafe{(*browser).base.size}, std::mem::size_of::<cef::_cef_browser_t>());
    browser
}


#[cfg(target_os = "linux")]
fn cef_window_info(hwnd: std::os::raw::c_ulong) -> cef::_cef_window_info_t {
    // Create GTK window. You can pass a NULL handle 
    // to CEF and then it will create a window of its own.
    //initialize_gtk();
    //let hwnd = create_gtk_window(String::from("cefcapi example"), 1024, 768);
    let window_info = cef::_cef_window_info_t {
        x: 0,
        y: 0,
        width: 1024,
        height: 768,
        //parent_window: unsafe {gtk2::gdk_x11_drawable_get_xid(gtk2::gtk_widget_get_window(hwnd)) },
        parent_window: unsafe {gtk2::gdk_x11_drawable_get_xid(gtk2::gtk_widget_get_window(hwnd as *mut libc::c_void))},
        //parent_window: hwnd,
        //parent_window: 0,
        windowless_rendering_enabled: 0,
        transparent_painting_enabled: 0,
        window: 0
    };
    println!("parent {}", window_info.parent_window);
    window_info
}

#[cfg(target_os = "macos")]
fn cef_window_info(hwnd: std::os::raw::c_ulong) -> cef::_cef_window_info_t {
    let window_info = cef::_cef_window_info_t {
        x: 0,
        y: 0,
        width: 1024,
        height: 768,
        //parent_window: unsafe {gtk2::gdk_x11_drawable_get_xid(gtk2::gtk_widget_get_window(hwnd)) },
        parent_view: hwnd as *mut std::os::raw::c_void,
        //parent_window: 0,
        windowless_rendering_enabled: 0,
        transparent_painting_enabled: 0,
        view: 0 as *mut std::os::raw::c_void,
        hidden: 0,
        window_name: cef::cef_string_t { str: std::ptr::null_mut(),  length: 0,  dtor: Option::None }
    };
    println!("parent {:?}", window_info.parent_view);
    window_info
}

#[cfg(windows)]
fn cef_window_info(hwnd: std::os::raw::c_ulong) -> cef::_cef_window_info_t {
    extern crate winapi;

    let window_info = cef::_cef_window_info_t {
        x: 0,
        y: 0,
        width: 1024,
        height: 768,
        parent_window: hwnd as cef::win::HWND,
        //parent_window: std::ptr::null_mut() as cef::win::HWND,
        windowless_rendering_enabled: 0,
        transparent_painting_enabled: 0,
        window: 0 as cef::win::HWND,
        ex_style: 0,
        window_name: cef::cef_string_t { str: std::ptr::null_mut(),  length: 0,  dtor: Option::None },
        style: winapi::winuser::WS_CHILDWINDOW | winapi::winuser::WS_CLIPCHILDREN
            | winapi::winuser::WS_CLIPSIBLINGS | winapi::winuser::WS_VISIBLE | winapi::winuser::WS_TABSTOP,
        //style: winapi::winuser::WS_POPUP | winapi::winuser::WS_OVERLAPPEDWINDOW | winapi::winuser::WS_CHILDWINDOW | winapi::winuser::WS_CLIPCHILDREN
        //    | winapi::winuser::WS_CLIPSIBLINGS | winapi::winuser::WS_VISIBLE,
        menu: 0 as cef::win::HMENU
    };
    println!("parent {:?}", window_info.parent_window);
    window_info
}

unsafe extern "C" fn on_context_initialized(_: *mut cef::_cef_browser_process_handler_t) {
    debug("In context_initialized_fn");

    //create_browser();
}

#[allow(unused)]
unsafe extern "C" fn on_before_child_process_launch(_: *mut cef::_cef_browser_process_handler_t,
                                                                                   command_line:
                                                                                       *mut cef::_cef_command_line_t) {
    debug("on_before_child_process_launch")
}

#[allow(unused)]
unsafe extern "C" fn on_render_process_thread_created(_: *mut cef::_cef_browser_process_handler_t,
                                                                                     extra_info:
                                                                                         *mut cef::_cef_list_value_t) {
    debug("on_render_process_thread_created")
}

unsafe extern "C" fn get_print_handler(_: *mut cef::_cef_browser_process_handler_t)
                                                     ->
                                                         *mut cef::_cef_print_handler_t {
    debug("get_print_handler");
    std::ptr::null_mut()
}

#[allow(unused)]
unsafe extern "C" fn  on_schedule_message_pump_work(_: *mut cef::_cef_browser_process_handler_t,
                                                                                  delay_ms: cef::int64) {
    debug("on_schedule_message_pump_work");
}


#[allow(unused)]
unsafe extern "C" fn on_before_command_line_processing(self_:
                                                                                          *mut cef::_cef_app_t,
                                                                                      process_type:
                                                                                          *const cef::cef_string_t,
                                                                                      command_line:
                                                                                          *mut cef::_cef_command_line_t) {
    debug("on_before_command_line_processing");
}

#[allow(unused)]
unsafe extern "C" fn on_register_custom_schemes(self_:
                                                                                   *mut cef::_cef_app_t,
                                                                               registrar:
                                                                                   *mut cef::_cef_scheme_registrar_t) {
    debug("on_register_custom_schemes");
}

unsafe extern "C" fn get_resource_bundle_handler(_: *mut cef::_cef_app_t)
                                                               ->
                                                                   *mut cef::_cef_resource_bundle_handler_t {
    //debug("get_resource_bundle_handler");
    std::ptr::null_mut()
}

unsafe extern "C" fn get_browser_process_handler(_: *mut cef::cef_app_t) -> *mut cef::_cef_browser_process_handler_t {
    debug("In get_browser_process_handler");

    let bph: cef::_cef_browser_process_handler_t = cef::_cef_browser_process_handler_t {
        base: base::CefBase::new(std::mem::size_of::<cef::_cef_browser_process_handler_t>()),
        on_context_initialized: Option::Some(on_context_initialized),
        on_before_child_process_launch: Option::Some(on_before_child_process_launch),
        on_render_process_thread_created: Option::Some(on_render_process_thread_created),
        get_print_handler: Option::Some(get_print_handler),
        on_schedule_message_pump_work: Option::Some(on_schedule_message_pump_work)
    };

    let browser_process_handler_box = Box::new(bph);
    let browser_process_handler = Box::into_raw(browser_process_handler_box);
    browser_process_handler
}

unsafe extern "C" fn get_render_process_handler(_:
                                                                                   *mut cef::_cef_app_t)
                                                              ->
                                                                  *mut cef::_cef_render_process_handler_t {
    //debug("get_render_process_handler");
    std::ptr::null_mut()
}

/*fn create_gtk_window(title: std::string::String, width: i32, height: i32) -> *mut gtk2::GtkWidget {
    println!("create_gtk_window");
    
    // Create window.
    let window = unsafe { gtk2::gtk_window_new(gtk2::GTK_WINDOW_TOPLEVEL) };

    // Destroy signal.
    //g_signal_connect(G_OBJECT(window), "destroy",
    //        G_CALLBACK(window_destroy_signal), NULL);
    
    // Default size.
    unsafe { gtk2::gtk_window_set_default_size(std::mem::transmute(window), width, height) };
    
    // Center.
    unsafe { gtk2::gtk_window_set_position(std::mem::transmute(window), gtk2::GTK_WIN_POS_CENTER) };
    
    // Title.
    let c_title = ffi::CString::new(title).unwrap();
    unsafe { gtk2::gtk_window_set_title(std::mem::transmute(window), c_title.as_ptr()) };
    
    // TODO: focus
    // g_signal_connect(window, "focus", G_CALLBACK(&HandleFocus), NULL);

    // CEF requires a container. Embedding browser in a top
    // level window fails.
    let vbox = unsafe { gtk2::gtk_vbox_new(0, 0) };
    unsafe { gtk2::gtk_container_add(std::mem::transmute(window), vbox) };
    
    // Show.
    unsafe { gtk2::gtk_widget_show_all(window) };

    vbox
}*/

fn debug(m: &str) {
    println!("{}", m);
}
