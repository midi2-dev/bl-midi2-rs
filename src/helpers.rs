use crate::bounded::Bounded;

pub fn mask<U, T>(u: U) -> T 
where
    U: core::ops::BitAnd<U>,
    T: std::convert::TryFrom<<U as std::ops::BitAnd>::Output>,
    <T as TryFrom<<U as std::ops::BitAnd>::Output>>::Error: std::fmt::Debug,
    T: Bounded,
    T: std::convert::Into<U>,
{
    (u & T::absolute_max().into()).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_u8_to_u7() {
        let v: u8 = 0b0110_0111;
        assert_eq!(ux::u7::new(0b110_0111), mask(v));
    }

    #[test]
    fn mask_u8_to_u4() {
        let v: u8 = 0b0110_0111;
        assert_eq!(ux::u4::new(0b0111), mask(v));
    }
}

