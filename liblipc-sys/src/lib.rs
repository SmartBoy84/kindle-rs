use core::ffi::c_str;
use std::{
    ffi::{CStr, CString},
    str::FromStr,
    thread::{self, JoinHandle},
};

use signal_hook::{consts::SIGINT, iterator::Signals};

use crate::liblipc_bindings::LIPC;

mod liblipc_bindings;

pub struct rLIPC {
    name: Option<String>,
    lipc: *mut LIPC, // LIPC is void -> *mut LIPC == *void in c
}

#[derive(Debug)]
pub enum LipcError {
    FailedToOpen,
}

impl rLIPC {
    /// spawns a thread to handle ctrl-c - must join this
    pub fn with_name(name: &str) -> Result<(Self, JoinHandle<()>), LipcError> {
        let name_c = CString::from_str(name).unwrap();
        let lipc = unsafe { liblipc_bindings::LipcOpen(name_c.as_ptr()) };
        if lipc == std::ptr::null_mut() {
            return Err(LipcError::FailedToOpen);
        }

        let term_handle = thread::spawn(|| {
            Signals::new([SIGINT])
                .expect("failed to register signal handler")
                .forever()
                .next();
        });
        let rlipc = Self {
            name: Some(name.to_string()),
            lipc,
        };

        Ok((rlipc, term_handle))
    }
}

impl Drop for rLIPC {
    fn drop(&mut self) {
        println!("Closing lipc");
        unsafe {
            liblipc_bindings::LipcClose(self.lipc);
        }
    }
}

// pub fn test() {
//     unsafe {
// liblipc_bindings::LipcOpen(service)

//         let x = CStr::from_ptr(liblipc_bindings::LipcGetErrorString(1))
//             .to_str()
//             .expect("failed to run funtion");
//         println!("{x}");
//     }
// }
