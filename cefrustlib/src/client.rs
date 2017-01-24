extern crate cefrust;

use cefrust::cef;

use cefrust::base;

use std::mem;
use std::ptr;

pub fn new() -> cef::_cef_client_t {
    let client = cef::_cef_client_t {
        base: base::CefBase::new(mem::size_of::<cef::_cef_client_t>()),
        get_context_menu_handler: Option::Some(get_context_menu_handler),
        get_dialog_handler: Option::Some(get_dialog_handler),
        get_display_handler: Option::Some(get_display_handler),
        get_download_handler: Option::Some(get_download_handler),
        get_drag_handler: Option::Some(get_drag_handler),
        get_find_handler: Option::Some(get_find_handler),
        get_focus_handler: Option::Some(get_focus_handler),
        get_geolocation_handler: Option::Some(get_geolocation_handler),
        get_jsdialog_handler: Option::Some(get_jsdialog_handler),
        get_keyboard_handler: Option::Some(get_keyboard_handler),
        get_life_span_handler: Option::Some(get_life_span_handler),
        get_load_handler: Option::Some(get_load_handler),
        get_render_handler: Option::Some(get_render_handler),
        get_request_handler: Option::Some(get_request_handler),
        on_process_message_received: Option::Some(on_process_message_received)
    };
    client
}

unsafe extern "C" fn get_context_menu_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_context_menu_handler_t {
    debug("get_context_menu_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_dialog_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_dialog_handler_t {
    debug("get_dialog_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_display_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_display_handler_t {
    //debug("get_display_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_download_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_download_handler_t {
    debug("get_download_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_drag_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_drag_handler_t {
    debug("get_drag_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_find_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_find_handler_t {
    debug("get_find_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_focus_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_focus_handler_t {
    //debug("get_focus_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_geolocation_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_geolocation_handler_t {
    debug("get_geolocation_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_jsdialog_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_jsdialog_handler_t {
    debug("get_jsdialog_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_keyboard_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_keyboard_handler_t {
    debug("get_keyboard_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_life_span_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_life_span_handler_t {
    debug("get_life_span_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_load_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_load_handler_t {
    //debug("get_load_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_render_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_render_handler_t {
    //debug("get_render_handler");
    ptr::null_mut()
}

unsafe extern "C" fn get_request_handler(_: *mut cef::_cef_client_t) -> *mut cef::_cef_request_handler_t {
    //debug("get_request_handler");
    ptr::null_mut()
}

#[allow(unused)]
unsafe extern "C" fn on_process_message_received(_: *mut cef::_cef_client_t, browser: *mut cef::_cef_browser_t, 
    source_process: cef::cef_process_id_t, message: *mut cef::_cef_process_message_t) -> ::std::os::raw::c_int {
    debug("on_process_message_received");
    0         
}

fn debug(m: &str) {
    println!("{}", m);
}
