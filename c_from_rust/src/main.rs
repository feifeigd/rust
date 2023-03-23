use std::ffi::CString;
use std::os::raw::{c_char, c_uint};

extern "C"{
    fn mystrlen(std: *const c_char)->c_uint;
}
fn main() {
    println!("Hello, world!");
    let c_string = CString::new("C From Rust").expect("failed");
    let count = unsafe {
        mystrlen(c_string.as_ptr())
    };
    println!("c_string's length is {}", count);
}
