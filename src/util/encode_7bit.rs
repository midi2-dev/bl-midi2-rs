use crate::util::Truncate;

pub trait Encode7Bit<const N: usize>:
    Sized
    + core::default::Default
    + core::convert::From<u8>
    + core::convert::From<ux::u7>
    + core::ops::BitOrAssign<Self>
    + core::ops::BitAnd<Self, Output = Self>
    + core::ops::Shl<usize, Output = Self>
    + core::ops::Shr<usize, Output = Self>
    + Truncate
    + core::marker::Copy
where
    ux::u7: core::convert::TryFrom<Self>,
    <ux::u7 as core::convert::TryFrom<Self>>::Error: core::fmt::Debug,
{
    fn from_u7s(u7s: &[u8; N]) -> Self {
        let mut ret: Self = Default::default();
        for (i, v) in u7s.iter().enumerate() {
            ret |= (Self::from(*v) & Self::from(0b0111_1111_u8)) << (7_usize * i);
        }
        ret
    }

    fn to_u7s(&self) -> [ux::u7; N] {
        let mut ret = [ux::u7::default(); N];
        for (i, v) in ret.iter_mut().enumerate() {
            *v = (*self >> (i * 7_usize)).truncate()
        }
        ret
    }
}

impl Encode7Bit<4> for ux::u28 {}
impl Encode7Bit<3> for ux::u21 {}
impl Encode7Bit<2> for ux::u14 {}
