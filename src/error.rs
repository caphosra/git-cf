use std::fmt;
use std::io::Error;
use zip::result::ZipError;

pub enum GitCFError {
    IOError(Box<Error>),
    JsonError(Box<serde_json::error::Error>),
    StringError(String),
    ZipError(Box<ZipError>)
}

impl From<Error> for GitCFError {
    fn from(err: Error) -> GitCFError {
        GitCFError::IOError(
            Box::new(err)
        )
    }
}

impl From<serde_json::error::Error> for GitCFError {
    fn from(err: serde_json::error::Error) -> GitCFError {
        GitCFError::JsonError(
            Box::new(err)
        )
    }
}

impl From<ZipError> for GitCFError {
    fn from(err: ZipError) -> GitCFError {
        GitCFError::ZipError(
            Box::new(err)
        )
    }
}

impl fmt::Display for GitCFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitCFError::IOError(err) => {
                write!(f, "{}", err)
            },
            GitCFError::JsonError(err) => {
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
