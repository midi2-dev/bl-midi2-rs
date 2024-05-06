use crate::detail::Bounded;

pub trait Truncate:
    Sized + Clone + Bounded + core::ops::BitAnd<Self> + core::ops::BitOr<Self>
{
    fn truncate<T>(&self) -> T
    where
        T: core::convert::TryFrom<<Self as core::ops::BitAnd>::Output>,
        <T as TryFrom<<Self as core::ops::BitAnd>::Output>>::Error: core::fmt::Debug,
        T: Bounded,
        T: core::convert::Into<Self>,
    {
        (self.clone() & T::absolute_max().into())
            .try_into()
            .unwrap()
    }
}

impl<T> Truncate for T where
    T: Sized + Clone + Bounded + core::ops::BitAnd<Self> + core::ops::BitOr<Self>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use ux::{u4, u7};

    #[test]
    fn truncate_u8_to_u7() {
        assert_eq!(u7::new(0b110_0111), 0b0110_0111_u8.truncate());
    }

    #[test]
    fn truncate_u8_to_u4() {
        assert_eq!(u4::new(0b0111), 0b0110_0111_u8.truncate());
    }
}
