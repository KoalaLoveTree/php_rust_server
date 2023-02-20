use std::error::Error;
use std::fmt::{Display, Formatter};
use phper::classes::{ClassEntity, ClassEntry};
use phper::error;
use phper::errors::{exception_class, Throwable};
use thiserror::Error;

const EXCEPTION_INVALID_CLOSURE_CLASS_NAME: &str = "HttpServer\\InvalidClosureException";
const EXCEPTION_INVALID_Z_TYPE_CLASS_NAME: &str = "HttpServer\\InvalidZTypeException";
const EXCEPTION_CONVERTING_TO_STRING_SLICE_CLASS_NAME: &str = "HttpServer\\ConvertingToStringSliceException";
const EXCEPTION_UNDEFINED_CLASS_NAME: &str = "HttpServer\\UndefinedException";

#[derive(thiserror::Error, Debug)]
pub enum PhpRustServerError {
    InvalidClosureError(InvalidClosureError),
    InvalidZTypeError(InvalidZTypeError),
    ConvertingToStringSliceError(ConvertingToStringSliceError),
}

impl Display for PhpRustServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "There is an error: {:?}", self)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct InvalidClosureError(pub Box<dyn Error>);

// impl Error for InvalidClosureError {}
//
// impl Display for InvalidClosureError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "There is an error: {}", self.0)
//     }
// }

impl Throwable for InvalidClosureError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_INVALID_CLOSURE_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}


#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct InvalidZTypeError(pub Box<dyn Error>);
//
// impl Error for InvalidZTypeError {}
//
// impl Display for InvalidZTypeError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "There is an error: {}", self.0)
//     }
// }

impl Throwable for InvalidZTypeError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_INVALID_Z_TYPE_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}


#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ConvertingToStringSliceError(pub Box<dyn Error>);
//
// impl Error for ConvertingToStringSliceError {}
//
// impl Display for ConvertingToStringSliceError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "There is an error: {}", self.0)
//     }
// }

impl Throwable for ConvertingToStringSliceError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CONVERTING_TO_STRING_SLICE_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}

pub fn make_exception_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(EXCEPTION_UNDEFINED_CLASS_NAME);
    class.extends(exception_class);
    class
}


impl From<InvalidClosureError> for phper::Error {
    fn from(e: InvalidClosureError) -> Self {
        phper::Error::throw(e)
    }
}


impl From<InvalidZTypeError> for phper::Error {
    fn from(e: InvalidZTypeError) -> Self {
        phper::Error::throw(e)
    }
}


impl From<ConvertingToStringSliceError> for phper::Error {
    fn from(e: ConvertingToStringSliceError) -> Self {
        phper::Error::throw(e)
    }
}