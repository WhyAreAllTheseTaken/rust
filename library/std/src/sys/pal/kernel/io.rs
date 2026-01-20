use kernel_call::{FileDescriptor, SystemError, UnionResult};
use crate::{io, sys::decode_error_kind};

pub fn convert_syscall_result(result: UnionResult<usize>) -> Result<usize, io::Error> {
    let result: Result<usize, SystemError> = result.into();

    result.map_err(|err| io::Error::new(decode_error_kind(err), Box::new(err)))
}

pub fn convert_syscall_desc_result(result: UnionResult<FileDescriptor>) -> Result<usize, io::Error> {
    let result: Result<FileDescriptor, SystemError> = result.into();

    result.map_err(|err| io::Error::new(decode_error_kind(err), Box::new(err)))
        .map(|x| x.into())
}

