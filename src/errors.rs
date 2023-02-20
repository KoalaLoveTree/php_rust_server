use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhpRustServerError {
    #[error("Internal Server Error: {message:?}")]
    InternalServerError { message: String },
}
