use std::ffi::c_char;
use std::slice;

pub unsafe fn print_buffer(buffer: *const c_char) {
    println!("[message bytes]");
    for byte in slice::from_raw_parts(buffer as *const u8, 1024) {
        print!("{:02x} ", byte);
    }
    println!();

}