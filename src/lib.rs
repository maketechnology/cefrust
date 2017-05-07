#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

pub mod cef;
pub mod base;

//use std::ffi;
//use std::os::raw;
//use std::os::unix::ffi::OsStrExt;

pub fn subp_path(cwd: &std::path::Path) -> String {
    let subp_path = if cfg!(target_os = "windows") { 
        cwd.join("cefrust_subp.exe")
    } else if cfg!(target_os = "macos") {
        //cwd.join("../Frameworks/cefrust_subp.app/Contents/MacOS/cefrust_subp")
        cwd.join("cefrust_subp.app/Contents/MacOS/cefrust_subp")
    } else { 
        cwd.join("cefrust_subp") 
    };
    let subp = subp_path.to_str().unwrap();
    println!("subp: {:?}", subp);
    String::from(subp)
}

#[cfg(unix)]
pub fn prepare_args() -> cef::_cef_main_args_t {
    use std::ffi;
    use std::os::raw;
    let mut args: Vec<*mut raw::c_char> = std::env::args().map(|arg| {
        println!("arg: {:?}", arg);
        let carg_rslt = ffi::CString::new(arg);
        let carg = carg_rslt.expect("cant create arg");
        //let mut mp = carg.as_ptr();
        //std::mem::forget(carg);
        let mp = carg.into_raw();
        mp
    }).collect();

    let args_size = args.len() as i32;
    let args_ptr = args.as_mut_ptr();
    std::mem::forget(args);

    let main_args = cef::_cef_main_args_t {
        argc : args_size,
        argv : args_ptr //as *mut *mut raw::c_char
    };
    println!("Hello CEF, ARGS: {:?}", main_args.argc);

    main_args
}

#[cfg(windows)]
pub fn prepare_args() -> cef::_cef_main_args_t {
    let h_instance: winapi::HMODULE = unsafe { kernel32::GetModuleHandleA(0 as winapi::winnt::LPCSTR) };
    let main_args = cef::_cef_main_args_t {
        instance: unsafe { std::mem::transmute(h_instance) }
        //instance: unsafe { std::mem::transmute(0 as i64) }
    };
    println!("Hello CEF, hinstance: {:?}", main_args.instance);
    main_args
}

pub fn cef_string(value: &str) -> cef::cef_string_t {
    let mut str_cef = cef::cef_string_t {str: std::ptr::null_mut(), length: 0, dtor: Option::Some(dtr)};
    //unsafe { cef::cef_string_utf16_set(value.as_ptr() as *mut cef::char16, value.len(), &mut str_cef, 1) };
    unsafe {cef::cef_string_utf8_to_utf16(value.as_ptr() as *mut std::os::raw::c_char, value.len(), &mut str_cef);}
    str_cef
}


pub fn cef_string_empty() -> cef::cef_string_t {
    let mut empty_str = cef::cef_string_t {
        str: std::ptr::null_mut(), 
        length: 0, 
        dtor: Option::Some(dtr)
    };
    
    let emp = "";
    //unsafe { cef::cef_string_utf16_set(emp.as_ptr() as *mut cef::char16, 0, &mut empty_str, 1) };
    unsafe { cef::cef_string_utf8_to_utf16(emp.as_ptr() as *mut std::os::raw::c_char, 0, &mut empty_str);}

    empty_str
}

unsafe extern "C" fn dtr(_: *mut cef::char16) {
    println!("DESTROY CEF_STRING");
}