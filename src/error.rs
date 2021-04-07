use std::io::Error;
use std::fmt;
use zip::result::ZipError;

pub enum GitCFError<'a> {
    IOError(Box<Error>),
    StringError(&'a str),
    ZipError(Box<ZipError>)
}

impl<'a> From<Error> for GitCFError<'a> {
    fn from(err: Error) -> GitCFError<'a> {
        GitCFError::IOError(
            Box::new(err)
        )
    }
}

impl<'a> From<&'a str> for GitCFError<'a> {
    fn from(err: &'a str) -> GitCFError<'a> {
        GitCFError::StringError(
            err
        )
    }
}

impl<'a> From<ZipError> for GitCFError<'a> {
    fn from(err: ZipError) -> GitCFError<'a> {
        GitCFError::ZipError(
            Box::new(err)
        )
    }
}

impl<'a> fmt::Display for GitCFError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitCFError::IOError(err) => {
                write!(f, "{}", err)
            },
            GitCFError::StringError(err) => {
                write!(f, "{}", err)
            },
            GitCFError::ZipError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
