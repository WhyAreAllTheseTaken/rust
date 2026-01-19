use super::unsupported;
use crate::ffi::{OsStr, OsString};
use crate::marker::PhantomData;
use crate::path::{self, PathBuf};
use crate::{fmt, io};
use crate::arch::asm;
use crate::env;
use crate::string::ToString;
use kernel_call::{SystemError, syscall_exit};

pub fn errno() -> SystemError {
    SystemError::None
}

pub fn error_string(errno: SystemError) -> String {
    errno.to_string()
}

pub fn getcwd() -> io::Result<PathBuf> {
    match env::var("PWD") {
        Ok(value) => Ok(PathBuf::from(value)),
        Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err))
    }
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub fn split_paths(unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}

impl crate::error::Error for JoinPathsError {}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub fn temp_dir() -> PathBuf {
    PathBuf::from(env::var("TEMP").unwrap_or_else(|_| "/tmp/".to_string()))
}

pub fn home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

pub fn exit(code: i32) -> ! {
    unsafe {
        syscall_exit(code as usize)
    }
}

pub fn getpid() -> u32 {
    panic!("no pids on this platform")
}
