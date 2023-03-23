
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub enum Order{
    Gt,
    Lt,
    Eq,
}

// 不要名字改编
#[no_mangle]
pub extern "C" fn compare_str(a:*const c_char, b:*const c_char)-> Order{
    let a = unsafe {
        CStr::from_ptr(a).to_bytes()
    };
    let b = unsafe {
        CStr::from_ptr(b).to_bytes()
    };
    if a > b{
        Order::Gt
    }else if a < b{
        Order::Lt
    }else{
        Order::Eq
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
