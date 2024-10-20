use thiserror::Error;

#[derive(Error, Debug)]
pub enum VecError {
    #[error("invalid length")]
    InvalidLength,
}

mod length_vec;
pub use length_vec::*;
