#[macro_use]
extern crate static_assertions;

pub use paste::paste;

mod hash_list;
pub use hash_list::HashList;

mod hash_string_macros;

mod hash_string;
pub use hash_string::HashString;

mod little32;
pub use little32::hash as hash_little32;
