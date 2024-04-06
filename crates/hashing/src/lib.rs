#[macro_use]
extern crate static_assertions;

mod hash_string;
pub use hash_string::HashString;

mod little32;
pub use little32::hash as hash_little32;
