use std::ffi::CStr;

use crate::bindings::liblipc_bindings;

mod bindings;

const LIPC_SO_PATH: &str = "/usr/lib/liblipc.so"; // K4

pub fn test() {
    unsafe {
        let lib =
            liblipc_bindings::OPENLIPC::new(LIPC_SO_PATH).expect("failed to load dynamic lib");
        let x = CStr::from_ptr(lib.LipcGetErrorString(1))
            .to_str()
            .expect("failed to run funtion");
        println!("{x}");
    }
}
