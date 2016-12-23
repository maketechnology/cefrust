//include!(concat!(env!("OUT_DIR"), "/cef.rs"));
mod cef;

fn main() {
    let args = cef::_cef_main_args_t {
        argc : 2,
        argv : ""
    };
    println!("Hello CEF: {}", args.argc);
}