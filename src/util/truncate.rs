use crate::util::Bounded;

pub trait Truncate : 
    Sized +
    Clone +
    Bounded + 
    core::ops::BitAnd<Self> + 
    core::ops::BitOr<Self> + 
{
    fn truncate<T>(&self) -> T
    where
        T: std::convert::TryFrom<<Self as std::ops::BitAnd>::Output>,
        <T as TryFrom<<Self as std::ops::BitAnd>::Output>>::Error: core::fmt::Debug,
        T: Bounded,
        T: std::convert::Into<Self>,
    {
        (self.clone() & T::absolute_max().into()).try_into().unwrap()
    }
}

impl<T> Truncate for T where T: 
    Sized +
    Clone +
    Bounded + 
    core::ops::BitAnd<Self> + 
    core::ops::BitOr<Self> + 
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_u8_to_u7() {
        assert_eq!(ux::u7::new(0b110_0111), 0b0110_0111_u8.truncate());
    }

    #[test]
    fn truncate_u8_to_u4() {
        assert_eq!(ux::u4::new(0b0111), 0b0110_0111_u8.truncate());
    }
}