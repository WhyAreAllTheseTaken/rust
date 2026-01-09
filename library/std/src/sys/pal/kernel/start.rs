use crate::ffi::{c_char, c_int};
use crate::ptr;

extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
#[used]
pub extern "C" fn _start() {
    unsafe {
        main(0, ptr::null());

        asm!(
            "mov rax,0",
            "int 0x81",
            options(noreturn)
        )
    }
}
