//! Global initialization and retrieval of command line arguments.
//!
//! On some platforms these are stored during runtime startup,
//! and on some they are retrieved from the system on demand.

#![allow(dead_code)] // runtime init functions not used during testing

use core::str::FromStr;

pub use super::common::Args;
use crate::ffi::{CStr, OsString};

/// One-time global initialization.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    unsafe { imp::init(argc, argv) }
}

/// Returns the command line arguments
pub fn args() -> Args {
    let (argc, argv) = imp::argc_argv();

    let mut vec = Vec::with_capacity(argc as usize);

    for i in 0..argc {
        // SAFETY: `argv` is non-null if `argc` is positive, and it is
        // guaranteed to be at least as long as `argc`, so reading from it
        // should be safe.
        let ptr = unsafe { argv.offset(i).read() };

        // Some C commandline parsers (e.g. GLib and Qt) are replacing already
        // handled arguments in `argv` with `NULL` and move them to the end.
        //
        // Since they can't directly ensure updates to `argc` as well, this
        // means that `argc` might be bigger than the actual number of
        // non-`NULL` pointers in `argv` at this point.
        //
        // To handle this we simply stop iterating at the first `NULL`
        // argument. `argv` is also guaranteed to be `NULL`-terminated so any
        // non-`NULL` arguments after the first `NULL` can safely be ignored.
        if ptr.is_null() {
            break;
        }

        // SAFETY: Just checked that the pointer is not NULL, and arguments
        // are otherwise guaranteed to be valid C strings.
        let cstr = unsafe { CStr::from_ptr(ptr) };
        vec.push(OsString::from_str(cstr.to_str().expect("Argument is not valid UTF-8. This is guaranteed by the ABI.")).expect("Rust str is not valid OsString"));
    }

    Args::new(vec)
}

mod imp {
    use crate::ffi::c_char;
    use crate::ptr;
    use crate::sync::atomic::{Atomic, AtomicIsize, AtomicPtr, Ordering};

    // The system-provided argc and argv, which we store in static memory
    // here so that we can defer the work of parsing them until its actually
    // needed.
    //
    // Note that we never mutate argv/argc, the argv array, or the argv
    // strings, which allows the code in this file to be very simple.
    static ARGC: Atomic<isize> = AtomicIsize::new(0);
    static ARGV: Atomic<*mut *const u8> = AtomicPtr::new(ptr::null_mut());

    unsafe fn really_init(argc: isize, argv: *const *const u8) {
        // These don't need to be ordered with each other or other stores,
        // because they only hold the unmodified system-provided argv/argc.
        ARGC.store(argc, Ordering::Relaxed);
        ARGV.store(argv as *mut _, Ordering::Relaxed);
    }

    #[inline(always)]
    pub unsafe fn init(argc: isize, argv: *const *const u8) {
        // on GNU/Linux if we are main then we will init argv and argc twice, it "duplicates work"
        // BUT edge-cases are real: only using .init_array can break most emulators, dlopen, etc.
        unsafe { really_init(argc, argv) };
    }

    /// glibc passes argc, argv, and envp to functions in .init_array, as a non-standard extension.
    /// This allows `std::env::args` to work even in a `cdylib`, as it does on macOS and Windows.
    #[cfg(target_env = "gnu")]
    #[used]
    #[unsafe(link_section = ".init_array.00099")]
    static ARGV_INIT_ARRAY: extern "C" fn(
        crate::os::raw::c_int,
        *const *const u8,
        *const *const u8,
    ) = {
        extern "C" fn init_wrapper(
            argc: crate::os::raw::c_int,
            argv: *const *const u8,
            _envp: *const *const u8,
        ) {
            unsafe { really_init(argc as isize, argv) };
        }
        init_wrapper
    };

    pub fn argc_argv() -> (isize, *const *const c_char) {
        // Load ARGC and ARGV, which hold the unmodified system-provided
        // argc/argv, so we can read the pointed-to memory without atomics or
        // synchronization.
        //
        // If either ARGC or ARGV is still zero or null, then either there
        // really are no arguments, or someone is asking for `args()` before
        // initialization has completed, and we return an empty list.
        let argv = ARGV.load(Ordering::Relaxed);
        let argc = if argv.is_null() { 0 } else { ARGC.load(Ordering::Relaxed) };

        // Cast from `*mut *const u8` to `*const *const c_char`
        (argc, argv.cast())
    }
}
