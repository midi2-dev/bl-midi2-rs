pub mod debug;

mod bit_ops;
mod bounded;
mod encode_7bit;
mod slice_data;
mod truncate;

pub use bit_ops::BitOps;
pub use bounded::Bounded;
pub use encode_7bit::Encode7Bit;
pub use slice_data::SliceData;
pub use truncate::Truncate;

pub trait MessageTraits {
    const DUMMY: u8 = 0;
}

impl<T> MessageTraits for T where T: Clone + core::fmt::Debug + PartialEq {}
