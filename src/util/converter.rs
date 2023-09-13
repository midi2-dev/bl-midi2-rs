use crate::{
    buffer::{Buffer, Bytes, Ump},
    result::Result,
    util::{
        schema::{BytesSchema, UmpSchema},
        BitOps, Encode7Bit, Truncate,
    },
    *,
};

use super::schema::NumericalConstant;

pub trait Converter<B: Buffer, S> {
    fn from_buffer(data: &B::Data) -> Self;
    fn to_buffer(&self, data: &mut B::Data);
    fn validate(_data: &B::Data) -> Result<()> {
        Ok(())
    }
}

impl Converter<Ump, UmpSchema<0x000F_FFFF, 0x0, 0x0, 0x0>> for u20 {
    fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
        data[0].truncate()
    }
    fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
        data[0] |= u32::from(*self);
    }
}

macro_rules! u4_ump_numerical_constants_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl<const T: u32> Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>>
            for NumericalConstant<T>
        {
            fn from_buffer(_data: &<Ump as Buffer>::Data) -> Self {
                NumericalConstant()
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_nibble($nibble_index, T.truncate());
            }
            fn validate(data: &<Ump as Buffer>::Data) -> Result<()> {
                if u32::from(data[$buffer_index].nibble($nibble_index)) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u4_ump_numerical_constants_converter!(0xF000_0000, 0x0, 0x0, 0x0, 0, 0);
u4_ump_numerical_constants_converter!(0x0F00_0000, 0x0, 0x0, 0x0, 0, 1);
u4_ump_numerical_constants_converter!(0x00F0_0000, 0x0, 0x0, 0x0, 0, 2);
u4_ump_numerical_constants_converter!(0x000F_0000, 0x0, 0x0, 0x0, 0, 3);
u4_ump_numerical_constants_converter!(0x0000_F000, 0x0, 0x0, 0x0, 0, 4);
u4_ump_numerical_constants_converter!(0x0000_0F00, 0x0, 0x0, 0x0, 0, 5);
u4_ump_numerical_constants_converter!(0x0000_00F0, 0x0, 0x0, 0x0, 0, 6);
u4_ump_numerical_constants_converter!(0x0000_000F, 0x0, 0x0, 0x0, 0, 7);

macro_rules! u4_bytes_numerical_constants_converter {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl<const T: u32> Converter<Bytes, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for NumericalConstant<T>
        {
            fn from_buffer(_data: &<Bytes as Buffer>::Data) -> Self {
                NumericalConstant()
            }
            fn to_buffer(&self, data: &mut <Bytes as Buffer>::Data) {
                data[$buffer_index].set_nibble($nibble_index, T.truncate());
            }
            fn validate(data: &<Bytes as Buffer>::Data) -> Result<()> {
                if u32::from(data[$buffer_index].nibble($nibble_index)) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u4_bytes_numerical_constants_converter!(0xF0, 0x0, 0x0, 0, 0);
u4_bytes_numerical_constants_converter!(0x0F, 0x0, 0x0, 0, 1);
u4_bytes_numerical_constants_converter!(0x0, 0xF0, 0x0, 1, 0);
u4_bytes_numerical_constants_converter!(0x0, 0x0F, 0x0, 1, 1);
u4_bytes_numerical_constants_converter!(0x0, 0x0, 0xF0, 2, 0);
u4_bytes_numerical_constants_converter!(0x0, 0x0, 0x0F, 2, 1);

macro_rules! u8_ump_numerical_constants_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl<const T: u32> Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>>
            for NumericalConstant<T>
        {
            fn from_buffer(_data: &<Ump as Buffer>::Data) -> Self {
                NumericalConstant()
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_octet($octet_index, T.truncate());
            }
            fn validate(data: &<Ump as Buffer>::Data) -> Result<()> {
                if u32::from(data[$buffer_index].octet($octet_index)) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u8_ump_numerical_constants_converter!(0xFF00_0000, 0x0, 0x0, 0x0, 0, 0);
u8_ump_numerical_constants_converter!(0x00FF_0000, 0x0, 0x0, 0x0, 0, 1);
u8_ump_numerical_constants_converter!(0x0000_FF00, 0x0, 0x0, 0x0, 0, 2);
u8_ump_numerical_constants_converter!(0x0000_00FF, 0x0, 0x0, 0x0, 0, 3);

macro_rules! u8_bytes_numerical_constants_converter {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl<const T: u32> Converter<Bytes, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for NumericalConstant<T>
        {
            fn from_buffer(_data: &<Bytes as Buffer>::Data) -> Self {
                NumericalConstant()
            }
            fn to_buffer(&self, data: &mut <Bytes as Buffer>::Data) {
                data[$buffer_index] = T.truncate();
            }
            fn validate(data: &<Bytes as Buffer>::Data) -> Result<()> {
                if u32::from(data[$buffer_index]) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u8_bytes_numerical_constants_converter!(0xFF, 0x0, 0x0, 0);
u8_bytes_numerical_constants_converter!(0x0, 0xFF, 0x0, 1);
u8_bytes_numerical_constants_converter!(0x0, 0x0, 0xFF, 2);

macro_rules! bool_ump_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$bit_index:expr) => {
        impl Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>> for bool {
            fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
                data[$buffer_index].bit($bit_index)
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_bit($bit_index, *self);
            }
        }
    };
}

bool_ump_converter!(0x0000_0001, 0x0, 0x0, 0x0, 0, 31);
bool_ump_converter!(0x0000_0002, 0x0, 0x0, 0x0, 0, 30);

macro_rules! u4_ump_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>> for u4 {
            fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
                data[$buffer_index].nibble($nibble_index)
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_nibble($nibble_index, *self);
            }
        }
    };
}

u4_ump_converter!(0xF000_0000, 0x0, 0x0, 0x0, 0, 0);
u4_ump_converter!(0x0F00_0000, 0x0, 0x0, 0x0, 0, 1);
u4_ump_converter!(0x00F0_0000, 0x0, 0x0, 0x0, 0, 2);
u4_ump_converter!(0x000F_0000, 0x0, 0x0, 0x0, 0, 3);
u4_ump_converter!(0x0000_F000, 0x0, 0x0, 0x0, 0, 4);
u4_ump_converter!(0x0000_0F00, 0x0, 0x0, 0x0, 0, 5);
u4_ump_converter!(0x0000_00F0, 0x0, 0x0, 0x0, 0, 6);
u4_ump_converter!(0x0000_000F, 0x0, 0x0, 0x0, 0, 7);

macro_rules! u4_bytes_converter {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl Converter<Bytes, BytesSchema<$bytes1, $bytes2, $bytes3>> for u4 {
            fn from_buffer(data: &<Bytes as Buffer>::Data) -> Self {
                data[$buffer_index].nibble($nibble_index)
            }
            fn to_buffer(&self, data: &mut <Bytes as Buffer>::Data) {
                data[$buffer_index].set_nibble($nibble_index, *self);
            }
        }
    };
}

u4_bytes_converter!(0xF0, 0x0, 0x0, 0, 0);
u4_bytes_converter!(0x0F, 0x0, 0x0, 0, 1);
u4_bytes_converter!(0x0, 0xF0, 0x0, 1, 0);
u4_bytes_converter!(0x0, 0x0F, 0x0, 1, 1);
u4_bytes_converter!(0x0, 0x0, 0xF0, 2, 0);
u4_bytes_converter!(0x0, 0x0, 0x0F, 2, 1);

macro_rules! u7_ump_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>> for u7 {
            fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
                data[$buffer_index].octet($octet_index).truncate()
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_octet($octet_index, (*self).into());
            }
        }
    };
}

u7_ump_converter!(0x7F00_0000, 0x0, 0x0, 0x0, 0, 0);
u7_ump_converter!(0x007F_0000, 0x0, 0x0, 0x0, 0, 1);
u7_ump_converter!(0x0000_7F00, 0x0, 0x0, 0x0, 0, 2);
u7_ump_converter!(0x0000_007F, 0x0, 0x0, 0x0, 0, 3);
u7_ump_converter!(0x0, 0x7F00_0000, 0x0, 0x0, 1, 0);
u7_ump_converter!(0x0, 0x007F_0000, 0x0, 0x0, 1, 1);
u7_ump_converter!(0x0, 0x0000_7F00, 0x0, 0x0, 1, 2);
u7_ump_converter!(0x0, 0x0000_007F, 0x0, 0x0, 1, 3);

macro_rules! u7_bytes_converter {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl Converter<Bytes, BytesSchema<$bytes1, $bytes2, $bytes3>> for u7 {
            fn from_buffer(data: &<Bytes as Buffer>::Data) -> Self {
                data[$buffer_index].truncate()
            }
            fn to_buffer(&self, data: &mut <Bytes as Buffer>::Data) {
                data[$buffer_index] = (*self).into();
            }
        }
    };
}

u7_bytes_converter!(0x7F, 0x0, 0x0, 0);
u7_bytes_converter!(0x0, 0x7F, 0x0, 1);
u7_bytes_converter!(0x0, 0x0, 0x7F, 2);

macro_rules! u8_bytes_converter {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl Converter<Bytes, BytesSchema<$bytes1, $bytes2, $bytes3>> for u8 {
            fn from_buffer(data: &<Bytes as Buffer>::Data) -> Self {
                data[$buffer_index]
            }
            fn to_buffer(&self, data: &mut <Bytes as Buffer>::Data) {
                data[$buffer_index] = *self;
            }
        }
    };
}

u8_bytes_converter!(0xFF, 0x0, 0x0, 0);
u8_bytes_converter!(0x0, 0xFF, 0x0, 1);
u8_bytes_converter!(0x0, 0x0, 0xFF, 2);

macro_rules! u8_ump_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>> for u8 {
            fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
                data[$buffer_index].octet($octet_index)
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_octet($octet_index, *self);
            }
        }
    };
}

u8_ump_converter!(0xFF00_0000, 0x0, 0x0, 0x0, 0, 0);
u8_ump_converter!(0x00FF_0000, 0x0, 0x0, 0x0, 0, 1);
u8_ump_converter!(0x0000_FF00, 0x0, 0x0, 0x0, 0, 2);
u8_ump_converter!(0x0000_00FF, 0x0, 0x0, 0x0, 0, 3);
u8_ump_converter!(0x0, 0xFF00_0000, 0x0, 0x0, 1, 0);
u8_ump_converter!(0x0, 0x00FF_0000, 0x0, 0x0, 1, 1);
u8_ump_converter!(0x0, 0x0000_FF00, 0x0, 0x0, 1, 2);
u8_ump_converter!(0x0, 0x0000_00FF, 0x0, 0x0, 1, 3);

macro_rules! u16_ump_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$word_index:expr) => {
        impl Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>> for u16 {
            fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
                data[$buffer_index].word($word_index)
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index].set_word($word_index, *self);
            }
        }
    };
}

u16_ump_converter!(0xFFFF_0000, 0x0, 0x0, 0x0, 0, 0);
u16_ump_converter!(0x0000_FFFF, 0x0, 0x0, 0x0, 0, 1);
u16_ump_converter!(0x0, 0xFFFF_0000, 0x0, 0x0, 1, 0);
u16_ump_converter!(0x0, 0x0000_FFFF, 0x0, 0x0, 1, 1);
u16_ump_converter!(0x0, 0x0, 0xFFFF_0000, 0x0, 2, 0);
u16_ump_converter!(0x0, 0x0, 0x0000_FFFF, 0x0, 2, 1);
u16_ump_converter!(0x0, 0x0, 0x0, 0xFFFF_0000, 3, 0);
u16_ump_converter!(0x0, 0x0, 0x0, 0x0000_FFFF, 3, 1);

macro_rules! u32_ump_converter {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr) => {
        impl Converter<Ump, UmpSchema<$ump1, $ump2, $ump3, $ump4>> for u32 {
            fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
                data[$buffer_index]
            }
            fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
                data[$buffer_index] = *self;
            }
        }
    };
}

u32_ump_converter!(0xFFFF_FFFF, 0x0, 0x0, 0x0, 0);
u32_ump_converter!(0x0, 0xFFFF_FFFF, 0x0, 0x0, 1);
u32_ump_converter!(0x0, 0x0, 0xFFFF_FFFF, 0x0, 2);
u32_ump_converter!(0x0, 0x0, 0x0, 0xFFFF_FFFF, 3);

impl Converter<Bytes, BytesSchema<0x0, 0x7F, 0x7F>> for u14 {
    fn from_buffer(data: &<Bytes as Buffer>::Data) -> Self {
        u14::from_u7s(&[data[1].truncate(), data[2].truncate()])
    }
    fn to_buffer(&self, data: &mut <Bytes as Buffer>::Data) {
        let u7s = self.to_u7s();
        data[1] = u7s[0].into();
        data[2] = u7s[1].into();
    }
}

impl Converter<Ump, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>> for u14 {
    fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
        u14::from_u7s(&[data[0].octet(2).truncate(), data[0].octet(3).truncate()])
    }
    fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
        let u7s = self.to_u7s();
        data[0].set_octet(2, u7s[0].into());
        data[0].set_octet(3, u7s[1].into());
    }
}

impl Converter<Ump, UmpSchema<0x0000_0001, 0x0000_7F7F, 0x0, 0x0>> for Option<u14> {
    fn from_buffer(data: &<Ump as Buffer>::Data) -> Self {
        if data[0].bit(31) {
            Some(u14::from_u7s(&[
                data[1].octet(2).truncate(),
                data[1].octet(3).truncate(),
            ]))
        } else {
            None
        }
    }
    fn to_buffer(&self, data: &mut <Ump as Buffer>::Data) {
        match self {
            Some(v) => {
                let u7s = v.to_u7s();
                data[1].set_octet(2, u7s[0].into());
                data[1].set_octet(3, u7s[1].into());
                data[0].set_bit(31, true);
            }
            None => {
                data[0].set_bit(31, false);
                data[1].set_word(1, 0x0);
            }
        }
    }
}

impl<T> Converter<Ump, ()> for T {
    fn from_buffer(_data: &<Ump as Buffer>::Data) -> Self {
        unreachable!()
    }
    fn to_buffer(&self, _data: &mut <Ump as Buffer>::Data) {
        unreachable!()
    }
}

impl<T> Converter<Bytes, ()> for T {
    fn from_buffer(_data: &<Bytes as Buffer>::Data) -> Self {
        unreachable!()
    }
    fn to_buffer(&self, _data: &mut <Bytes as Buffer>::Data) {
        unreachable!()
    }
}
