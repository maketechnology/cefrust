extern crate libc;

pub type XID = ::std::os::raw::c_ulong;

extern "C" {
    pub fn gtk_widget_get_window(widget: *mut libc::c_void) -> *mut libc::c_void;
    pub fn gdk_x11_drawable_get_xid(drawable: *mut libc::c_void) -> XID;
    pub fn gtk_vbox_new(homogeneous: libc::c_int, spacing: libc::c_int) -> u64;
    pub fn gtk_fixed_put(container: u64, widget: u64, x: libc::c_int, y: libc::c_int);
}