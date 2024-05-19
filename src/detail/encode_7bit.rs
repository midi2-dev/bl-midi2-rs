use ux::{u14, u21, u28, u7};

pub trait Byte: Copy {
    fn to_u8(self) -> u8;
    fn from_u8(v: u8) -> Self;
}

impl Byte for u7 {
    fn to_u8(self) -> u8 {
        self.into()
    }
    fn from_u8(v: u8) -> Self {
        u7::new(v & 0x7F)
    }
}

impl Byte for u8 {
    fn from_u8(v: u8) -> Self {
        v & 0x7F
    }
    fn to_u8(self) -> u8 {
        self & 0x7F
    }
}

pub trait Encode7Bit {
    fn from_u7s<T: Byte>(u7s: &[T]) -> Self;
    fn to_u7s<T: Byte>(&self, data: &mut [T]);
}

impl Encode7Bit for u28 {
    fn to_u7s<T: Byte>(&self, data: &mut [T]) {
        debug_assert!(data.len() == 4);

        let v = u32::from(*self);

        data[0] = T::from_u8((v >> 7 * 0) as u8);
        data[1] = T::from_u8((v >> 7 * 1) as u8);
        data[2] = T::from_u8((v >> 7 * 2) as u8);
        data[3] = T::from_u8((v >> 7 * 3) as u8);
    }
    fn from_u7s<T: Byte>(u7s: &[T]) -> Self {
        debug_assert!(u7s.len() == 4);

        let mut ret: u28 = Default::default();

        ret |= u28::from(u7s[0].to_u8() & 0x7F) << 7 * 0;
        ret |= u28::from(u7s[1].to_u8() & 0x7F) << 7 * 1;
        ret |= u28::from(u7s[2].to_u8() & 0x7F) << 7 * 2;
        ret |= u28::from(u7s[3].to_u8() & 0x7F) << 7 * 3;

        ret
    }
}

impl Encode7Bit for u21 {
    fn to_u7s<T: Byte>(&self, data: &mut [T]) {
        debug_assert!(data.len() == 3);

        let v = u32::from(*self);

        data[0] = T::from_u8((v >> 7 * 0) as u8);
        data[1] = T::from_u8((v >> 7 * 1) as u8);
        data[2] = T::from_u8((v >> 7 * 2) as u8);
    }
    fn from_u7s<T: Byte>(u7s: &[T]) -> Self {
        debug_assert!(u7s.len() == 3);

        let mut ret: u21 = Default::default();

        ret |= u21::from(u7s[0].to_u8() & 0x7F) << 7 * 0;
        ret |= u21::from(u7s[1].to_u8() & 0x7F) << 7 * 1;
        ret |= u21::from(u7s[2].to_u8() & 0x7F) << 7 * 2;

        ret
    }
}

impl Encode7Bit for u14 {
    fn to_u7s<T: Byte>(&self, data: &mut [T]) {
        debug_assert!(data.len() == 2);

        let v = u16::from(*self);

        data[0] = T::from_u8((v >> 7 * 0) as u8);
        data[1] = T::from_u8((v >> 7 * 1) as u8);
    }
    fn from_u7s<T: Byte>(u7s: &[T]) -> Self {
        debug_assert!(u7s.len() == 2);

        let mut ret: u14 = Default::default();

        ret |= u14::from(u7s[0].to_u8() & 0x7F) << 7 * 0;
        ret |= u14::from(u7s[1].to_u8() & 0x7F) << 7 * 1;

        ret
    }
}
