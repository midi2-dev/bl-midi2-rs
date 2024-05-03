use crate::{
    numeric_types::{u14, u4, u7},
    util::{BitOps, Encode7Bit},
};

pub trait UmpSchema {}
pub trait BytesSchema {}

pub trait UmpSchemaRepr<U: UmpSchema>: Sized {
    fn read(buffer: &[u32]) -> Self;
    fn write(buffer: &mut [u32], value: Self);
}

pub trait BytesSchemaRepr<U: BytesSchema>: Sized {
    fn read(buffer: &[u8]) -> Self;
    fn write(buffer: &mut [u8], value: Self);
}

pub struct Ump<const D1: u32, const D2: u32, const D3: u32, const D4: u32>;
pub struct Bytes<const B1: u8, const B2: u8, const B3: u8>;

impl<const D1: u32, const D2: u32, const D3: u32, const D4: u32> UmpSchema for Ump<D1, D2, D3, D4> {}
impl<const B1: u8, const B2: u8, const B3: u8> BytesSchema for Bytes<B1, B2, B3> {}

macro_rules! bool_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$bit_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for bool {
            fn read(buffer: &[u32]) -> Self {
                buffer[$buffer_index].bit($bit_index)
            }
            fn write(buffer: &mut [u32], v: Self) {
                buffer[$buffer_index].set_bit($bit_index, v);
            }
        }
    };
}

bool_ump_property_impl!(0x0000_0001, 0x0, 0x0, 0x0, 0, 31);
bool_ump_property_impl!(0x0000_0002, 0x0, 0x0, 0x0, 0, 30);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0000_0000_0001,
    0x0,
    0x0,
    1,
    31
);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0000_0000_0010,
    0x0,
    0x0,
    1,
    30
);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0000_0000_0100,
    0x0,
    0x0,
    1,
    29
);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0000_0000_1000,
    0x0,
    0x0,
    1,
    28
);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0000_0001_0000,
    0x0,
    0x0,
    1,
    27
);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0001_0000_0000,
    0x0,
    0x0,
    1,
    23
);
bool_ump_property_impl!(
    0x0,
    0b0000_0000_0000_0000_0000_0010_0000_0000,
    0x0,
    0x0,
    1,
    22
);
bool_ump_property_impl!(
    0b0000_0000_0000_0000_1000_0000_0000_0000,
    0x0,
    0x0,
    0x0,
    0,
    16
);
bool_ump_property_impl!(
    0x0,
    0b1000_0000_0000_0000_0000_0000_0000_0000,
    0x0,
    0x0,
    1,
    0
);

macro_rules! u4_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for u4 {
            fn read(buffer: &[u32]) -> Self {
                buffer[$buffer_index].nibble($nibble_index)
            }
            fn write(buffer: &mut [u32], v: Self) {
                buffer[$buffer_index].set_nibble($nibble_index, v);
            }
        }
    };
}

u4_ump_property_impl!(0xF000_0000, 0x0, 0x0, 0x0, 0, 0);
u4_ump_property_impl!(0x0F00_0000, 0x0, 0x0, 0x0, 0, 1);
u4_ump_property_impl!(0x00F0_0000, 0x0, 0x0, 0x0, 0, 2);
u4_ump_property_impl!(0x000F_0000, 0x0, 0x0, 0x0, 0, 3);
u4_ump_property_impl!(0x0000_F000, 0x0, 0x0, 0x0, 0, 4);
u4_ump_property_impl!(0x0000_0F00, 0x0, 0x0, 0x0, 0, 5);
u4_ump_property_impl!(0x0000_00F0, 0x0, 0x0, 0x0, 0, 6);
u4_ump_property_impl!(0x0000_000F, 0x0, 0x0, 0x0, 0, 7);
u4_ump_property_impl!(0x0, 0x0F00_0000, 0x0, 0x0, 1, 1);

macro_rules! u4_bytes_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl BytesSchemaRepr<Bytes<$bytes1, $bytes2, $bytes3>> for u4 {
            fn read(buffer: &[u8]) -> Self {
                buffer[$buffer_index].nibble($nibble_index)
            }
            fn write(buffer: &mut [u8], v: Self) {
                buffer[$buffer_index].set_nibble($nibble_index, v);
            }
        }
    };
}

u4_bytes_property_impl!(0xF0, 0x0, 0x0, 0, 0);
u4_bytes_property_impl!(0x0F, 0x0, 0x0, 0, 1);
u4_bytes_property_impl!(0x0, 0xF0, 0x0, 1, 0);
u4_bytes_property_impl!(0x0, 0x0F, 0x0, 1, 1);
u4_bytes_property_impl!(0x0, 0x0, 0xF0, 2, 0);
u4_bytes_property_impl!(0x0, 0x0, 0x0F, 2, 1);

macro_rules! u7_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for u7 {
            fn read(buffer: &[u32]) -> Self {
                buffer[$buffer_index].septet($octet_index)
            }
            fn write(buffer: &mut [u32], v: Self) {
                buffer[$buffer_index].set_septet($octet_index, (v));
            }
        }
    };
}

u7_ump_property_impl!(0x7F00_0000, 0x0, 0x0, 0x0, 0, 0);
u7_ump_property_impl!(0x007F_0000, 0x0, 0x0, 0x0, 0, 1);
u7_ump_property_impl!(0x0000_7F00, 0x0, 0x0, 0x0, 0, 2);
u7_ump_property_impl!(0x0000_007F, 0x0, 0x0, 0x0, 0, 3);
u7_ump_property_impl!(0x0, 0x7F00_0000, 0x0, 0x0, 1, 0);
u7_ump_property_impl!(0x0, 0x007F_0000, 0x0, 0x0, 1, 1);
u7_ump_property_impl!(0x0, 0x0000_7F00, 0x0, 0x0, 1, 2);
u7_ump_property_impl!(0x0, 0x0000_007F, 0x0, 0x0, 1, 3);

macro_rules! u7_bytes_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl BytesSchemaRepr<Bytes<$bytes1, $bytes2, $bytes3>> for u7 {
            fn read(buffer: &[u8]) -> Self {
                buffer[$buffer_index].septet(0)
            }
            fn write(buffer: &mut [u8], v: Self) {
                buffer[$buffer_index].set_septet(0, v);
            }
        }
    };
}

u7_bytes_property_impl!(0x7F, 0x0, 0x0, 0);
u7_bytes_property_impl!(0x0, 0x7F, 0x0, 1);
u7_bytes_property_impl!(0x0, 0x0, 0x7F, 2);

macro_rules! u8_bytes_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl BytesSchemaRepr<Bytes<$bytes1, $bytes2, $bytes3>> for u8 {
            fn read(buffer: &[u8]) -> Self {
                buffer[$buffer_index]
            }
            fn write(buffer: &mut [u8], v: Self) {
                buffer[$buffer_index] = v;
            }
        }
    };
}

u8_bytes_property_impl!(0xFF, 0x0, 0x0, 0);
u8_bytes_property_impl!(0x0, 0xFF, 0x0, 1);
u8_bytes_property_impl!(0x0, 0x0, 0xFF, 2);

macro_rules! u8_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for u8 {
            fn read(buffer: &[u32]) -> Self {
                buffer[$buffer_index].octet($octet_index)
            }
            fn write(buffer: &mut [u32], v: Self) {
                buffer[$buffer_index].set_octet($octet_index, v);
            }
        }
    };
}

u8_ump_property_impl!(0xFF00_0000, 0x0, 0x0, 0x0, 0, 0);
u8_ump_property_impl!(0x00FF_0000, 0x0, 0x0, 0x0, 0, 1);
u8_ump_property_impl!(0x0000_FF00, 0x0, 0x0, 0x0, 0, 2);
u8_ump_property_impl!(0x0000_00FF, 0x0, 0x0, 0x0, 0, 3);
u8_ump_property_impl!(0x0, 0xFF00_0000, 0x0, 0x0, 1, 0);
u8_ump_property_impl!(0x0, 0x00FF_0000, 0x0, 0x0, 1, 1);
u8_ump_property_impl!(0x0, 0x0000_FF00, 0x0, 0x0, 1, 2);
u8_ump_property_impl!(0x0, 0x0000_00FF, 0x0, 0x0, 1, 3);
u8_ump_property_impl!(0x0, 0x0, 0xFF00_0000, 0x0, 2, 0);
u8_ump_property_impl!(0x0, 0x0, 0x00FF_0000, 0x0, 2, 1);
u8_ump_property_impl!(0x0, 0x0, 0x0000_FF00, 0x0, 2, 2);
u8_ump_property_impl!(0x0, 0x0, 0x0000_00FF, 0x0, 2, 3);
u8_ump_property_impl!(0x0, 0x0, 0x0, 0xFF00_0000, 3, 0);
u8_ump_property_impl!(0x0, 0x0, 0x0, 0x00FF_0000, 3, 1);
u8_ump_property_impl!(0x0, 0x0, 0x0, 0x0000_FF00, 3, 2);
u8_ump_property_impl!(0x0, 0x0, 0x0, 0x0000_00FF, 3, 3);

macro_rules! u14_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$word_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for u14 {
            fn read(buffer: &[u32]) -> Self {
                u14::from_u7s(&[
                    buffer[$buffer_index].septet(2 * $word_index),
                    buffer[$buffer_index].septet(2 * $word_index + 1),
                ])
            }
            fn write(buffer: &mut [u32], v: Self) {
                let mut u7s = [u7::default(); 2];
                v.to_u7s(&mut u7s);
                buffer[$buffer_index].set_septet(2 * $word_index, u7s[0]);
                buffer[$buffer_index].set_septet(2 * $word_index + 1, u7s[1]);
            }
        }
    };
}

u14_ump_property_impl!(0x7F7F_0000, 0x0, 0x0, 0x0, 0, 0);
u14_ump_property_impl!(0x0000_7F7F, 0x0, 0x0, 0x0, 0, 1);
u14_ump_property_impl!(0x0, 0x7F7F_0000, 0x0, 0x0, 1, 0);
u14_ump_property_impl!(0x0, 0x0000_7F7F, 0x0, 0x0, 1, 1);
u14_ump_property_impl!(0x0, 0x0, 0x7F7F_0000, 0x0, 2, 0);
u14_ump_property_impl!(0x0, 0x0, 0x0000_7F7F, 0x0, 2, 1);
u14_ump_property_impl!(0x0, 0x0, 0x0, 0x7F7F_0000, 3, 0);
u14_ump_property_impl!(0x0, 0x0, 0x0, 0x0000_7F7F, 3, 1);

impl BytesSchemaRepr<Bytes<0x0, 0x7F, 0x7F>> for u14 {
    fn read(buffer: &[u8]) -> Self {
        u14::from_u7s(&[buffer[1].septet(0), buffer[2].septet(0)])
    }
    fn write(buffer: &mut [u8], v: Self) {
        let mut u7s = [u7::default(); 2];
        v.to_u7s(&mut u7s);
        buffer[1].set_septet(0, u7s[0]);
        buffer[2].set_septet(0, u7s[1]);
    }
}

macro_rules! u16_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$word_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for u16 {
            fn read(buffer: &[u32]) -> Self {
                buffer[$buffer_index].word($word_index)
            }
            fn write(buffer: &mut [u32], v: Self) {
                buffer[$buffer_index].set_word($word_index, v);
            }
        }
    };
}

u16_ump_property_impl!(0xFFFF_0000, 0x0, 0x0, 0x0, 0, 0);
u16_ump_property_impl!(0x0000_FFFF, 0x0, 0x0, 0x0, 0, 1);
u16_ump_property_impl!(0x0, 0xFFFF_0000, 0x0, 0x0, 1, 0);
u16_ump_property_impl!(0x0, 0x0000_FFFF, 0x0, 0x0, 1, 1);
u16_ump_property_impl!(0x0, 0x0, 0xFFFF_0000, 0x0, 2, 0);
u16_ump_property_impl!(0x0, 0x0, 0x0000_FFFF, 0x0, 2, 1);
u16_ump_property_impl!(0x0, 0x0, 0x0, 0xFFFF_0000, 3, 0);
u16_ump_property_impl!(0x0, 0x0, 0x0, 0x0000_FFFF, 3, 1);

macro_rules! u32_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr) => {
        impl UmpSchemaRepr<Ump<$ump1, $ump2, $ump3, $ump4>> for u32 {
            fn read(buffer: &[u32]) -> Self {
                buffer[$buffer_index]
            }
            fn write(buffer: &mut [u32], v: Self) {
                buffer[$buffer_index] = v;
            }
        }
    };
}

u32_ump_property_impl!(0xFFFF_FFFF, 0x0, 0x0, 0x0, 0);
u32_ump_property_impl!(0x0, 0xFFFF_FFFF, 0x0, 0x0, 1);
u32_ump_property_impl!(0x0, 0x0, 0xFFFF_FFFF, 0x0, 2);
u32_ump_property_impl!(0x0, 0x0, 0x0, 0xFFFF_FFFF, 3);
