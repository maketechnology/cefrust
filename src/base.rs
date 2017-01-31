use cef;

use std::option::Option;

const DEBUG_REFERENCE_COUNTING: bool = false;

pub type CefBase = cef::cef_base_t;


///
// Structure defining the reference count implementation functions. All
// framework structures must include the cef_base_t structure first.
///
impl CefBase {
    pub fn new(st_size: usize) -> CefBase {
        println!("initialize_CefBase");
        println!("cef_base_t.size = {}", st_size);

        let base = CefBase {
            size: st_size,
            add_ref: Option::Some(add_ref),
            release: Option::Some(release),
            has_one_ref: Option::Some(has_one_ref)
//            add_ref: Option::None,
//            release: Option::None,
//            has_one_ref: Option::None
        };
        //println!("add_re {:?}", base.add_ref);
        base
    }
}

        ///
        // Increment the reference count.
        ///
        pub unsafe extern "C" fn add_ref(_: *mut cef::cef_base_t) {
            debug_callback("cef_base_t.add_ref");
            if DEBUG_REFERENCE_COUNTING {
                println!("+");
            }
        }

        ///
        // Decrement the reference count.  Delete this object when no references
        // remain.
        ///
        pub unsafe extern "C" fn release(_: *mut cef::cef_base_t)
                                                -> ::std::os::raw::c_int {
            debug_callback("cef_base_t.release");
            if DEBUG_REFERENCE_COUNTING {
                println!("-");
            }
            1
        }

        ///
        // Returns the current number of references.
        ///
        pub unsafe extern "C" fn has_one_ref(_: *mut cef::cef_base_t)
                                                -> ::std::os::raw::c_int {
            debug_callback("cef_base_t.get_refct");
            if DEBUG_REFERENCE_COUNTING {
                println!("=");
            }
            1
        }


//pub 

fn debug_callback(l: &str) {
	//println!("{}", l)
}
