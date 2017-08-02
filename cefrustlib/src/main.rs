extern crate cefrustlib;
extern crate cefrust;

use std::env;
//use std::path::Path;

extern crate libc;

fn cef() {
    let cefrust_path = "/home/guille/.swtcef/3.3029.1611.g44e39a8/linux-x86_64";
    let cefrust_path = std::ffi::CString::new(cefrust_path).unwrap();

    let app = cefrustlib::app::new();
    let app = Box::new(app);
    let appp = Box::into_raw(app);

    println!("call init");
    cefrustlib::init(appp, cefrust_path.as_ptr());
    println!("after call init");

    let ten = std::time::Duration::from_secs(8);
    let now = std::time::Instant::now();

    std::thread::sleep(ten);

    assert!(now.elapsed() >= ten);
    println!("call shutdown");

    // Shut down CEF.
    cefrustlib::shut();
    println!("after call shutdown");

    println!("app {:?}", appp);

    std::thread::sleep(ten);

    assert!(now.elapsed() >= ten);
    println!("exiting");
}

fn main() {
    cef();
    //subp();

    //initialize_gtk();
    //create_gtk_window(String::from("Hi"), 800, 600);
    
}
