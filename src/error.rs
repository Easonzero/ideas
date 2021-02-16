use std::io;
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
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    StringError(String),
}

macro_rules! derive_from {
    ($src:path, $dst:path) => {
        impl From<$src> for Error {
            fn from(err: $src) -> Error {
                $dst(err)
            }
        }
    };
}

derive_from!(serde_json::Error, Error::Serde);
derive_from!(sled::Error, Error::Sled);
derive_from!(crossterm::ErrorKind, Error::Crossterm);
derive_from!(FromUtf8Error, Error::Utf8);
derive_from!(String, Error::StringError);
derive_from!(io::Error, Error::Io);

pub type Result<T> = std::result::Result<T, Error>;
