use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhpRustServerError {
    #[error("invalid closure provided")]
    InvalidClosureError(),
    #[error("data type not match with expected {expected:?}")]
    InvalidZTypeError {
        expected: String,
    },
    #[error("error while converting to string slice")]
    ConvertingToStringSliceError(),
}