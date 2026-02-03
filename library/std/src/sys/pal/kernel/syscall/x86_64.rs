use core::arch::naked_asm;

use crate::sys::syscall::{error::{SystemError, UnionResult}, file::{FileDescriptor, OpenFlags}, map::MapFlags};

/// Exit the program.
#[unsafe(naked)]
pub extern "C" fn syscall_exit(code: isize) -> ! {
    naked_asm!(
        "mov rax,0x00",
        "int 0x81"
        )
}

#[unsafe(naked)]
pub extern "C" fn syscall_thread_exit(code: isize) -> ! {
    naked_asm!(
        "mov rax,0x01",
        "int 0x81"
        )
}

/// Yield control back to the operating system.
///
/// If `micros` is 0 a yield is instead performed.
#[unsafe(naked)]
pub extern "C" fn syscall_sleep(micros: u64) {
    naked_asm!(
        "mov rax,0x02",
        "int 0x81",
        "ret"
        )
}

/// Read bytes from the specified file to fill the specified buffer up to length.
///
/// Returns -1 on an error or the number of bytes read on a success.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_read(file: FileDescriptor, buffer: *mut u8, length: usize) -> UnionResult<usize> {
    naked_asm!(
        "mov rax,0x12",
        "int 0x81",
        "ret"
        )
}

/// Writes bytes to the specified file to fill the specified buffer up to length.
///
/// Returns -1 on an error or the number of bytes written on a success.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_write(file: FileDescriptor, buffer: *const u8, length: usize) -> UnionResult<usize> {
    naked_asm!(
        "mov rax,0x13",
        "int 0x81",
        "ret"
        )
}

/// Opens the specified file descriptor.
///
/// Returns -1 on an error or a file descriptor number on success.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_open(path: *const u8, path_length: usize, mode: OpenFlags) -> UnionResult<FileDescriptor> {
    naked_asm!(
        "mov rax,0x10",
        "int 0x81",
        "ret"
        )
}

/// Closes the specified file descriptor.
#[unsafe(naked)]
pub extern "C" fn syscall_close(file: usize) -> SystemError {
    naked_asm!(
        "mov rax,0x11",
        "int 0x81",
        "ret"
        )
}

/// Allocates sufficient memory for the given length returning a pointer to that memory.
///
/// - `base` - The base address to map from, may be null in which case an address will be assigned.
/// - `length` - The length of the mapping.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_memory_map(base: *mut u8, length: usize, flags: MapFlags, file: FileDescriptor, offset: u64) -> *mut u8 {
    naked_asm!(
        "mov rax,0x20",
        "int 0x81",
        "ret"
        )
}

/// Deallocates sufficient memory for the given length returning a pointer to that memory.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_memory_unmap(base: *mut u8, length: usize) -> SystemError {
    naked_asm!(
        "mov rax,0x21",
        "int 0x81",
        "ret"
        )
}

/// Gets the specified environment variable.
///
/// Returns a negative value on an error or the length of the variable's value.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_get_var(name: *const u8, name_length: usize, buffer: *mut u8, buffer_length: usize) -> UnionResult<usize> {
    naked_asm!(
        "mov rax,0x30",
        "int 0x81",
        "ret"
        )
}

/// Sets the specified environment variable.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_set_var(name: *const u8, name_length: usize, buffer: *const u8, buffer_length: usize) -> SystemError {
    naked_asm!(
        "mov rax,0x31",
        "int 0x81",
        "ret"
        )
}

