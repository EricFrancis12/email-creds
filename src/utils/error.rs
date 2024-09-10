use std::{
    error,
    io::{Error, ErrorKind},
};

pub fn fmt_err<E: Into<Box<dyn error::Error + Send + Sync>>>(err: E, kind: ErrorKind) -> Error {
    Error::new(kind, err)
}
