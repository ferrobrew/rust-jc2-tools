pub use binrw::{BinRead, BinWrite};

pub trait BinReadWrite: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()> {}

impl<T> BinReadWrite for T where T: BinRead<Args<'static> = ()> + BinWrite<Args<'static> = ()> {}

pub mod archive;
pub mod length;
pub mod math;
pub mod property_container;
pub mod render_block_model;
