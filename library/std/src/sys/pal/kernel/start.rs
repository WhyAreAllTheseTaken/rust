use core::arch::naked_asm;

use crate::ffi::{c_char, c_int};
use crate::ptr;
use crate::sys::syscall::syscall_exit;

unsafe extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
#[allow(unused)]
#[unsafe(naked)]
pub extern "C" fn _start() {
    naked_asm!(
        // Argc
        "mov rdi,[rsp]",
        "mov rsi,rsp",
        // Argv
        "add rsi,8",
        // Call _start2
        "call _start2")
}

#[unsafe(no_mangle)]
extern "C" fn _start2(argc: usize, argv: *const *const u8) {
    unsafe {
        crate::sys::args::init(argc as isize, argv);

        super::init(argc as isize, argv, 0);

        syscall_exit(main(0, ptr::null()) as isize)
    }
}

