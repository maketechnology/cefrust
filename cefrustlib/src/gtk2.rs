#![allow(non_camel_case_types)]

extern crate libc;

pub type XID = libc::c_ulong;

pub type gint = libc::c_int;
pub type gulong = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gpointer = *mut libc::c_void;

type GClosure = *mut libc::c_void;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct _GdkRectangle {
    pub x: gint,
    pub y: gint,
    pub width: gint,
    pub height: gint,
}
impl Clone for _GdkRectangle {
    fn clone(&self) -> Self { *self }
}
pub type GdkRectangle = _GdkRectangle;

pub type GtkAllocation = GdkRectangle;

extern "C" {
    pub fn gtk_widget_get_window(widget: *mut libc::c_void) -> *mut libc::c_void;
    pub fn gdk_x11_drawable_get_xid(drawable: *mut libc::c_void) -> XID;
    pub fn gtk_vbox_new(homogeneous: libc::c_int, spacing: libc::c_int) -> u64;
    pub fn gtk_fixed_put(container: u64, widget: u64, x: libc::c_int, y: libc::c_int);
    //pub fn gtk_widget_set_size_request(widget: *mut libc::c_void, width: libc::c_int, height: libc::c_int);
    //pub fn gtk_widget_get_allocation(widget: *mut libc::c_void, allocation: *mut GtkAllocation);
    //pub fn gdk_window_resize(widget: *mut libc::c_void, width: libc::c_int, height: libc::c_int);
    //pub fn gdk_window_lookup(x11_win: XID) -> *mut libc::c_void;
    pub fn gtk_window_present(widget: *mut libc::c_void);
    pub fn gtk_widget_get_toplevel(widget: *mut libc::c_void) -> *mut libc::c_void;
}
