use core::io::BorrowedCursor;

use crate::io::{self, IoSlice, IoSliceMut, Read, Write};
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys::pal::io::convert_syscall_result;
use crate::sys::syscall::file::FileDescriptor;
use crate::sys::syscall::{syscall_read, syscall_write};
use crate::sys::{FromInner, IntoInner};

#[derive(Debug)]
#[repr(transparent)]
pub struct FileDesc(OwnedFd);

impl FromInner<OwnedFd> for FileDesc {
    fn from_inner(inner: OwnedFd) -> Self {
        Self(inner)
    }
}

impl IntoInner<OwnedFd> for FileDesc {
    fn into_inner(self) -> OwnedFd {
        self.0
    }
}

impl AsFd for FileDesc {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl IntoRawFd for FileDesc {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl AsRawFd for FileDesc {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FileDesc {
    #[inline]
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            convert_syscall_result(syscall_read(FileDescriptor::from(self.as_raw_fd()), buf.as_mut_ptr(), buf.len()))
        }
    }

    #[inline]
    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        unimplemented!("read_buf")
    }

    #[inline]
    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        let mut total = 0;

        for slice in bufs {
            total += self.read(slice.as_mut_slice())?;
        }

        Ok(total)
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        // Do not force `Chain<Empty, T>` or `Chain<T, Empty>` to use vectored
        // reads, unless the other reader is vectored.
        false
    }

    #[inline]
    pub fn read_exact(&self, mut buf: &mut [u8]) -> io::Result<()> {
        while !buf.is_empty() {
            let read = self.read(buf)?;

            let len = buf.len();
            buf = &mut buf[read..len];
        }

        Ok(())
    }

    #[inline]
    pub fn read_buf_exact(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        unimplemented!("read_buf_exact")
    }

    #[inline]
    pub fn read_to_end(&self, _buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut buf = [0; 1024];

        let mut total = 0;

        loop {
            let read = self.read(buf.as_mut_slice())?;

            total += read;

            if read == 0 {
                return Ok(total)
            }
        }
    }

    #[inline]
    pub fn read_to_string(&self, _buf: &mut String) -> io::Result<usize> {
        unimplemented!("read_to_string")
    }
}

impl FileDesc {
    #[inline]
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            convert_syscall_result(syscall_write(FileDescriptor::from(self.as_raw_fd()), buf.as_ptr(), buf.len()))
        }
    }

    #[inline]
    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let mut total = 0;

        for slice in bufs {
            total += self.write(slice.as_slice())?;
        }

        Ok(total)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        true
    }

    #[inline]
    pub fn write_all(&self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            let written = self.write(buf)?;

            let len = buf.len();
            buf = &buf[written..len];
        }

        Ok(())
    }

    #[inline]
    pub fn write_all_vectored(&self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        for slice in bufs {
            self.write_all(slice.as_slice())?;
        }

        Ok(())
    }

    // Keep the default write_fmt so the `fmt::Arguments` are still evaluated.

    #[inline]
    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

