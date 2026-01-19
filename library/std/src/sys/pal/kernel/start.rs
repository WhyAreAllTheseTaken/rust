use crate::ffi::{c_char, c_int};
use crate::ptr;
use kernel_call::syscall_exit;

unsafe extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
#[allow(unused)]
pub extern "C" fn _start() {
    unsafe {
        super::init(0, ptr::null(), 0);

        syscall_exit(main(0, ptr::null()) as usize)
    }
}
