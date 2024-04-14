use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringError {
    #[error("invalid length")]
    InvalidLength,
}

mod length_string;
pub use length_string::*;
