pub mod builder;
pub mod debug;
pub mod getter;
pub mod sysex_message;

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

#[cfg(test)]
macro_rules! message_traits_test {
    ($t:ty) => {
        use crate::util::MessageTraits;
        #[test]
        fn traits() {
            let _ = <Message as MessageTraits>::DUMMY;
        }
    };
}

#[cfg(test)]
pub(crate) use message_traits_test;
