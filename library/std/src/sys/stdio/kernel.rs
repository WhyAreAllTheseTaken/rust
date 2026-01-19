use core::mem::ManuallyDrop;

use kernel_call::{FileDescriptor, SystemError, syscall_read, syscall_write};

use crate::{io::{self, BorrowedCursor, ErrorKind, IoSlice, IoSliceMut}, os::fd::{FromRawFd, OwnedFd}, sys::{FromInner, fd::FileDesc, pal::io::convert_syscall_result, unsupported}};

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Self
    }
}

impl io::Read for Stdin {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read(buf)
    }

    #[inline]
    fn read_buf(&mut self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read_buf(cursor)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read_vectored(bufs)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).is_read_vectored()
    }

    #[inline]
    fn read_exact(&mut self, mut buf: &mut [u8]) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read_exact(buf)
    }

    #[inline]
    fn read_buf_exact(&mut self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read_buf_exact(cursor)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(0) })).read_to_string(buf)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(1) })).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(1) })).write_vectored(bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(1) })).is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(1) })).write_all(buf)
    }

    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(1) })).write_all_vectored(bufs)
    }

    // Keep the default write_fmt so the `fmt::Arguments` are still evaluated.

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(1) })).flush()
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(2) })).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(2) })).write_vectored(bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(2) })).is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(2) })).write_all(buf)
    }

    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(2) })).write_all_vectored(bufs)
    }

    // Keep the default write_fmt so the `fmt::Arguments` are still evaluated.

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        ManuallyDrop::new(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(2) })).flush()
    }
}

pub const STDIN_BUF_SIZE: usize = 1024;

pub fn is_ebadf(err: &io::Error) -> bool {
    err.raw_os_error() == Some(SystemError::UnknownFileDescriptor)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}

