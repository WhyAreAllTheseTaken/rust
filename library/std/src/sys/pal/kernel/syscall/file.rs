use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct FileDescriptor(usize);

#[stable(feature = "kernel_syscall", since = "1.94.0")]
impl core::convert::From<FileDescriptor> for usize {
    fn from(value: FileDescriptor) -> Self {
        value.0
    }
}

impl core::convert::From<usize> for FileDescriptor {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// Flags for opening files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct OpenFlags(usize);

#[stable(feature = "kernel_syscall", since = "1.94.0")]
impl From<OpenFlags> for usize {
    fn from(value: OpenFlags) -> Self {
        value.0
    }
}

impl From<usize> for OpenFlags {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl BitAndAssign for OpenFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitAnd for OpenFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXorAssign for OpenFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl BitOrAssign for OpenFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for OpenFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitOr for OpenFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for OpenFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl OpenFlags {
    pub const EMPTY: OpenFlags = OpenFlags(0);
    pub const READ: OpenFlags = OpenFlags(1 << 0);
    pub const WRITE: OpenFlags = OpenFlags(1 << 1);
    pub const CREATE: OpenFlags = OpenFlags(1 << 2);
    pub const APPEND: OpenFlags = OpenFlags(1 << 3);
    pub const REQUIRE_CREATE: OpenFlags = OpenFlags(1 << 4);
    pub const TRUNCATE: OpenFlags = OpenFlags(1 << 5);
    pub const SEMAPHORE: OpenFlags = OpenFlags(1 << 6);
    pub const LINK: OpenFlags = OpenFlags(1 << 7);
}

impl Default for OpenFlags {
    fn default() -> Self {
        Self::READ.into()
    }
}

impl OpenFlags {
    pub fn contains(self, flags: Self) -> bool {
        (self.0 & flags.0) != 0
    }
}

/// The mode to use for seeking through a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(usize)]
pub(crate) enum SeekMode {
    #[default]
    Start,
    End,
    Current
}

