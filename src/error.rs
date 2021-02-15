use std::string::FromUtf8Error;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Crossterm(#[cause] crossterm::ErrorKind),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Error {
        Error::Sled(err)
    }
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Error {
        Error::Crossterm(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::StringError(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
