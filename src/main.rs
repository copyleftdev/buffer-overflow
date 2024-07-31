extern crate libc;

use std::ffi::CString;
use std::ptr;

unsafe fn exploit() {
    let payload = CString::new(vec!['A' as u8; 128]).unwrap();
    let vulnerable_function: extern "C" fn(*const i8) = std::mem::transmute(0xdeadbeef as *const ());

    vulnerable_function(payload.as_ptr());
}

fn main() {
    unsafe {
        exploit();
    }
}
