use std::ffi::CStr;

use crate::bindings::liblipc_bindings;

mod bindings;

const LIPC_SO_PATH: &str = "/usr/lib/liblipc.so"; // K4

pub fn test() {
    unsafe {
        let x = CStr::from_ptr(liblipc_bindings::LipcGetErrorString(1))
            .to_str()
            .expect("failed to run funtion");
        println!("{x}");
    }
}
