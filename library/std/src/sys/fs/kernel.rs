use kernel_call::{OpenFlags, syscall_open, syscall_read};

use crate::ffi::OsString;
use crate::fmt;
use crate::fs::TryLockError;
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, Read, SeekFrom, Write};
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use crate::path::{Path, PathBuf};
pub use crate::sys::fs::common::Dir;
use crate::sys::pal::io::{convert_syscall_desc_result, convert_syscall_result};
use crate::sys::time::SystemTime;
use crate::sys::{IntoInner, unsupported};
use crate::sys::FromInner;

use crate::sys::fd::FileDesc;

pub struct File(FileDesc);

impl FromInner<FileDesc> for File {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}

impl IntoInner<FileDesc> for File {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl AsFd for File {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

pub struct FileAttr(!);

pub struct ReadDir(FileDesc);

pub struct DirEntry(!);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    flags: OpenFlags
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

pub struct FilePermissions(!);

pub struct FileType(!);

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        unimplemented!("file")
    }

    pub fn perm(&self) -> FilePermissions {
        unimplemented!("file")
    }

    pub fn file_type(&self) -> FileType {
        unimplemented!("file")
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        unimplemented!("file")
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        unimplemented!("file")
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        unimplemented!("file")
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        unimplemented!("file")
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        unimplemented!("file")
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        unimplemented!("file")
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        unimplemented!("file")
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        unimplemented!("file")
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!("file")
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        unimplemented!("file")
    }

    pub fn is_file(&self) -> bool {
        unimplemented!("file")
    }

    pub fn is_symlink(&self) -> bool {
        unimplemented!("file")
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        unimplemented!("file")
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, _other: &FileType) -> bool {
        unimplemented!("file")
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, _h: &mut H) {
        unimplemented!("file")
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!("file")
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!("file")
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        unimplemented!("file")
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!("file")
    }

    pub fn file_name(&self) -> OsString {
        unimplemented!("file")
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unimplemented!("file")
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unimplemented!("file")
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {flags: OpenFlags::EMPTY}
    }

    pub fn read(&mut self, read: bool) {
        if read {
            self.flags |= OpenFlags::READ;
        } else {
            self.flags &= !OpenFlags::READ;
        }
    }
    pub fn write(&mut self, write: bool) {
        if write {
            self.flags |= OpenFlags::WRITE;
        } else {
            self.flags &= !OpenFlags::WRITE;
        }
    }
    pub fn append(&mut self, append: bool) {
        if append {
            self.flags |= OpenFlags::APPEND;
        } else {
            self.flags &= !OpenFlags::APPEND;
        }
    }
    pub fn truncate(&mut self, truncate: bool) {
        if truncate {
            self.flags |= OpenFlags::TRUNCATE;
        } else {
            self.flags &= !OpenFlags::TRUNCATE;
        }
    }
    pub fn create(&mut self, create: bool) {
        if create {
            self.flags |= OpenFlags::CREATE;
        } else {
            self.flags &= !OpenFlags::CREATE;
        }
    }
    pub fn create_new(&mut self, create_new: bool) {
        if create_new {
            self.flags |= OpenFlags::REQUIRE_CREATE;
        } else {
            self.flags &= !OpenFlags::REQUIRE_CREATE;
        }
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let bytes = path.as_u8_slice();

        let desc = convert_syscall_desc_result(unsafe { syscall_open(bytes.as_ptr(), bytes.len(), opts.flags) })?;

        Ok(Self(FileDesc::from_inner(unsafe { OwnedFd::from_raw_fd(desc) })))
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unimplemented!("attr")
    }

    pub fn fsync(&self) -> io::Result<()> {
        unimplemented!("fsync")
    }

    pub fn datasync(&self) -> io::Result<()> {
        unimplemented!("datasync")
    }

    pub fn lock(&self) -> io::Result<()> {
        unimplemented!("lock")
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        unimplemented!("lock_shared")
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        unimplemented!("try_lock")
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        unimplemented!("try_lock_shared")
    }

    pub fn unlock(&self) -> io::Result<()> {
        unimplemented!("unlock")
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        unimplemented!("seek")
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        self.0.read_buf(cursor)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    pub fn flush(&self) -> io::Result<()> {
        self.0.flush()
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        unimplemented!("seek")
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        unimplemented!("size")
    }

    pub fn tell(&self) -> io::Result<u64> {
        unimplemented!("tell")
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unimplemented!("duplicate")
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unimplemented!("chmod")
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        unimplemented!("set_times")
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("desc", &self.as_raw_fd())
            .finish()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    match perm.0 {}
}

pub fn set_times(_p: &Path, _times: FileTimes) -> io::Result<()> {
    unsupported()
}

pub fn set_times_nofollow(_p: &Path, _times: FileTimes) -> io::Result<()> {
    unsupported()
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn exists(_path: &Path) -> io::Result<bool> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}

