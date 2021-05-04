use std::fmt;
use std::io::Error as IOError;
use glob::{ GlobError, PatternError };
use serde_json::error::Error as JsonError;
use zip::result::ZipError;

pub enum GitCFError {
    IOError(Box<IOError>),
    JsonError(Box<JsonError>),
    GlobError(Box<GlobError>),
    PatternError(Box<PatternError>),
    StringError(String),
    ZipError(Box<ZipError>)
}

impl From<IOError> for GitCFError {
    fn from(err: IOError) -> GitCFError {
        GitCFError::IOError(
            Box::new(err)
        )
    }
}

impl From<JsonError> for GitCFError {
    fn from(err: JsonError) -> GitCFError {
        GitCFError::JsonError(
            Box::new(err)
        )
    }
}

impl From<GlobError> for GitCFError {
    fn from(err: GlobError) -> GitCFError {
        GitCFError::GlobError(
            Box::new(err)
        )
    }
}

impl From<PatternError> for GitCFError {
    fn from(err: PatternError) -> GitCFError {
        GitCFError::PatternError(
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
            GitCFError::GlobError(err) => {
                write!(f, "{}", err)
            },
            GitCFError::PatternError(err) => {
                write!(f, "{}", err)
            },
            GitCFError::ZipError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
