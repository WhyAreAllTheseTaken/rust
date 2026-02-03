use core::{error::Error, fmt::{self, Display, Formatter}, mem::transmute};

/// Error numbers returned by system calls.
/// cbindgen:prefix-with-name
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(isize)]
#[non_exhaustive]
pub(crate) enum SystemError {
    None = 0,
    NotPermitted = -1,
    FileNotFound = -2,
    ProcessNotFound = -3,
    Interrupted = -4,
    IO = -5,
    DeviceNotFound = -6,
    TooManyArguments = -7,
    InvalidExecutableFormat = -8,
    UnknownFileDescriptor = -9,
    InsufficientMemory = -12,
    PermissionDenied = -13,
    BadAddress = -14,
    Busy = -16,
    FileAlreadyExists = -17,
    NotADirectory = -20,
    IsADirectory = -21,
    InvalidArgument = -22,
    FileTooLarge = -27,
    NoSpace = -28,
    InvalidSeek = -29,
    NotWritable = -30,
    PipeClosed = -32,
    Deadlock = -45,
    InvalidRequestNumber = -54,
    FileDeadlock = -56,
    Timeout = -62,
    NotEmpty = -90,
    PathTooLong = -91,
    NotSupported = -95,
    DataTooLong = -122,
    Cancelled = -140,
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::None => write!(f, "none"),
            SystemError::NotPermitted => write!(f, "notpermitted"),
            SystemError::FileNotFound => write!(f, "filenotfound"),
            SystemError::ProcessNotFound => write!(f, "processnotfound"),
            SystemError::Interrupted => write!(f, "interrupted"),
            SystemError::IO => write!(f, "io"),
            SystemError::DeviceNotFound => write!(f, "devicenotfound"),
            SystemError::TooManyArguments => write!(f, "toomanyarguments"),
            SystemError::InvalidExecutableFormat => write!(f, "invalidexecutableformat"),
            SystemError::UnknownFileDescriptor => write!(f, "unknownfiledescriptor"),
            SystemError::InsufficientMemory => write!(f, "insufficientmemory"),
            SystemError::PermissionDenied => write!(f, "permissiondenied"),
            SystemError::BadAddress => write!(f, "badaddress"),
            SystemError::Busy => write!(f, "busy"),
            SystemError::FileAlreadyExists => write!(f, "filealreadyexists"),
            SystemError::NotADirectory => write!(f, "notadirectory"),
            SystemError::IsADirectory => write!(f, "isadirectory"),
            SystemError::InvalidArgument => write!(f, "invalidargument"),
            SystemError::FileTooLarge => write!(f, "filetoolarge"),
            SystemError::NoSpace => write!(f, "nospace"),
            SystemError::InvalidSeek => write!(f, "invalidseek"),
            SystemError::NotWritable => write!(f, "notwritable"),
            SystemError::PipeClosed => write!(f, "pipeclosed"),
            SystemError::Deadlock => write!(f, "deadlock"),
            SystemError::InvalidRequestNumber => write!(f, "invalidrequestnumber"),
            SystemError::FileDeadlock => write!(f, "filedeadlock"),
            SystemError::Timeout => write!(f, "timeout"),
            SystemError::NotEmpty => write!(f, "notempty"),
            SystemError::PathTooLong => write!(f, "pathtoolong"),
            SystemError::NotSupported => write!(f, "notsupported"),
            SystemError::DataTooLong => write!(f, "datatoolong"),
            SystemError::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl Error for SystemError {
}

impl From<isize> for SystemError {
    fn from(value: isize) -> Self {
        unsafe { transmute::<isize, SystemError>(value) }
    }
}

/// A mirror of rust's `Result` that can be used specifically with system errors with the platform
/// ABI.
///
/// To use this the type `T` must be transmutable to an isize and must not be valid for any
/// negative value.
#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) union UnionResult<T: Copy> {
    value: T,
    error: isize
}

impl <T: Copy> UnionResult<T> {
    pub extern "C" fn is_err(&self) -> bool {
        unsafe {self.error < 0}
    }

    pub extern "C" fn is_ok(&self) -> bool {
        !self.is_err()
    }

    pub(crate) fn from_reg(value: usize) -> Self {
        Self {
            error: value as isize
        }
    }
}

impl <T: Copy> From<usize> for UnionResult<T> {
    fn from(value: usize) -> Self {
        Self {
            error: value as isize
        }
    }
}

#[stable(feature = "kernel_syscall", since = "1.94.0")]
impl <T: Copy> From<UnionResult<T>> for Result<T, SystemError> {
    fn from(value: UnionResult<T>) -> Self {
        if unsafe {value.error < 0} {
            Result::Err(SystemError::from(unsafe {value.error}))
        } else {
            Result::Ok(unsafe {value.value})
        }
    }
}

impl <T: Copy> From<Result<T, SystemError>> for UnionResult<T> {
    fn from(value: Result<T, SystemError>) -> Self {
        match value {
            Result::Ok(value) => UnionResult { value },
            Result::Err(error) => UnionResult { error: error as isize },
        }
    }
}


