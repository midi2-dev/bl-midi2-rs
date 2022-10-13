use crate::{
    packet::Packet,
};

pub mod bounded;
mod slice_data;

pub use bounded::Bounded;

pub fn truncate<U, T>(u: U) -> T
where
    U: core::ops::BitAnd<U>,
    T: std::convert::TryFrom<<U as std::ops::BitAnd>::Output>,
    <T as TryFrom<<U as std::ops::BitAnd>::Output>>::Error: core::fmt::Debug,
    T: Bounded,
    T: std::convert::Into<U>,
{
    (u & T::absolute_max().into()).try_into().unwrap()
}

pub trait MessageTraits {
    const DUMMY: u8 = 0;
}

impl<T> MessageTraits for T where T:
    Clone +
    core::fmt::Debug +
    PartialEq +
    core::convert::TryFrom<Packet> +
    core::convert::Into<Packet> +
{}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_u8_to_u7() {
        let v: u8 = 0b0110_0111;
        assert_eq!(ux::u7::new(0b110_0111), truncate(v));
    }

    #[test]
    fn truncate_u8_to_u4() {
        let v: u8 = 0b0110_0111;
        assert_eq!(ux::u4::new(0b0111), truncate(v));
    }
}
