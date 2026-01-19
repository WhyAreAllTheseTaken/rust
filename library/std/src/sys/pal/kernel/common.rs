use kernel_call::SystemError;

use crate::io::{self as std_io, ErrorKind};
use super::os::exit;

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn is_interrupted(error: SystemError) -> bool {
    error == SystemError::Interrupted
}

pub fn decode_error_kind(error: SystemError) -> ErrorKind {
    match error {
        SystemError::None => ErrorKind::Uncategorized,
        SystemError::NotPermitted => ErrorKind::PermissionDenied,
        SystemError::FileNotFound => ErrorKind::NotFound,
        SystemError::ProcessNotFound => ErrorKind::NotFound,
        SystemError::Interrupted => ErrorKind::Interrupted,
        SystemError::IO => ErrorKind::Other,
        SystemError::DeviceNotFound => ErrorKind::NotFound,
        SystemError::TooManyArguments => ErrorKind::ArgumentListTooLong,
        SystemError::InvalidExecutableFormat => ErrorKind::InvalidData,
        SystemError::UnknownFileDescriptor => ErrorKind::InvalidInput,
        SystemError::InsufficientMemory => ErrorKind::OutOfMemory,
        SystemError::PermissionDenied => ErrorKind::PermissionDenied,
        SystemError::BadAddress => ErrorKind::AddrNotAvailable,
        SystemError::Busy => ErrorKind::ResourceBusy,
        SystemError::FileAlreadyExists => ErrorKind::AlreadyExists,
        SystemError::NotADirectory => ErrorKind::NotADirectory,
        SystemError::IsADirectory => ErrorKind::IsADirectory,
        SystemError::InvalidArgument => ErrorKind::InvalidInput,
        SystemError::FileTooLarge => ErrorKind::FileTooLarge,
        SystemError::NoSpace => ErrorKind::StorageFull,
        SystemError::InvalidSeek => ErrorKind::InvalidInput,
        SystemError::NotWritable => ErrorKind::ReadOnlyFilesystem,
        SystemError::PipeClosed => ErrorKind::BrokenPipe,
        SystemError::Deadlock => ErrorKind::Deadlock,
        SystemError::InvalidRequestNumber => ErrorKind::InvalidInput,
        SystemError::FileDeadlock => ErrorKind::Deadlock,
        SystemError::Timeout => ErrorKind::TimedOut,
        SystemError::NotEmpty => ErrorKind::DirectoryNotEmpty,
        SystemError::PathTooLong => ErrorKind::InvalidFilename,
        SystemError::NotSupported => ErrorKind::Unsupported,
        SystemError::DataTooLong => ErrorKind::FileTooLarge,
        SystemError::Cancelled => ErrorKind::ConnectionAborted,
        _ => ErrorKind::Uncategorized
    }
}

pub fn abort_internal() -> ! {
    exit(-1)
}

