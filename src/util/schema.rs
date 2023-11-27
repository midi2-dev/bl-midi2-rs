use crate::{
    buffer::{Buffer, Bytes, Ump},
    result::Result,
    util::{BitOps, Encode7Bit, Truncate},
    *,
};
pub struct UmpSchema<const D1: u32, const D2: u32, const D3: u32, const D4: u32>();
pub struct BytesSchema<const B1: u8, const B2: u8, const B3: u8>();

pub trait Schema {}
impl<const B1: u8, const B2: u8, const B3: u8> Schema for BytesSchema<B1, B2, B3> {}
impl<const D1: u32, const D2: u32, const D3: u32, const D4: u32> Schema
    for UmpSchema<D1, D2, D3, D4>
{
}
impl Schema for () {}

pub trait Property<T, UmpSchema: Schema, BytesSchema: Schema>: Buffer {
    fn get(data: &[<Self as Buffer>::Data]) -> T;
    fn write(data: &mut [<Self as Buffer>::Data], v: T);
    fn validate(_data: &[<Self as Buffer>::Data]) -> Result<()> {
        Ok(())
    }
}

impl Property<u20, UmpSchema<0x000F_FFFF, 0x0, 0x0, 0x0>, ()> for Ump {
    fn get(data: &[<Ump as Buffer>::Data]) -> u20 {
        data[0].truncate()
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: u20) {
        data[0] |= u32::from(v);
    }
}

macro_rules! u4_ump_numerical_constants_property_imp {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl<const T: u32, BytesSchema: Schema>
            Property<NumericalConstant<T>, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(_data: &[<Ump as Buffer>::Data]) -> NumericalConstant<T> {
                NumericalConstant()
            }
            fn write(data: &mut [<Ump as Buffer>::Data], _: NumericalConstant<T>) {
                data[$buffer_index].set_nibble($nibble_index, T.truncate());
            }
            fn validate(data: &[<Ump as Buffer>::Data]) -> Result<()> {
                if u32::from(data[$buffer_index].nibble($nibble_index)) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u4_ump_numerical_constants_property_imp!(0xF000_0000, 0x0, 0x0, 0x0, 0, 0);
u4_ump_numerical_constants_property_imp!(0x0F00_0000, 0x0, 0x0, 0x0, 0, 1);
u4_ump_numerical_constants_property_imp!(0x00F0_0000, 0x0, 0x0, 0x0, 0, 2);
u4_ump_numerical_constants_property_imp!(0x000F_0000, 0x0, 0x0, 0x0, 0, 3);
u4_ump_numerical_constants_property_imp!(0x0000_F000, 0x0, 0x0, 0x0, 0, 4);
u4_ump_numerical_constants_property_imp!(0x0000_0F00, 0x0, 0x0, 0x0, 0, 5);
u4_ump_numerical_constants_property_imp!(0x0000_00F0, 0x0, 0x0, 0x0, 0, 6);
u4_ump_numerical_constants_property_imp!(0x0000_000F, 0x0, 0x0, 0x0, 0, 7);

macro_rules! u4_bytes_numerical_constants_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl<const T: u32, UmpSchema: Schema>
            Property<NumericalConstant<T>, UmpSchema, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for Bytes
        {
            fn get(_data: &[<Bytes as Buffer>::Data]) -> NumericalConstant<T> {
                NumericalConstant()
            }
            fn write(data: &mut [<Bytes as Buffer>::Data], _: NumericalConstant<T>) {
                data[$buffer_index].set_nibble($nibble_index, T.truncate());
            }
            fn validate(data: &[<Bytes as Buffer>::Data]) -> Result<()> {
                if u32::from(data[$buffer_index].nibble($nibble_index)) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u4_bytes_numerical_constants_property_impl!(0xF0, 0x0, 0x0, 0, 0);
u4_bytes_numerical_constants_property_impl!(0x0F, 0x0, 0x0, 0, 1);
u4_bytes_numerical_constants_property_impl!(0x0, 0xF0, 0x0, 1, 0);
u4_bytes_numerical_constants_property_impl!(0x0, 0x0F, 0x0, 1, 1);
u4_bytes_numerical_constants_property_impl!(0x0, 0x0, 0xF0, 2, 0);
u4_bytes_numerical_constants_property_impl!(0x0, 0x0, 0x0F, 2, 1);

macro_rules! u8_ump_numerical_constants_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl<const T: u32, BytesSchema: Schema>
            Property<NumericalConstant<T>, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(_data: &[<Ump as Buffer>::Data]) -> NumericalConstant<T> {
                NumericalConstant()
            }
            fn write(data: &mut [<Ump as Buffer>::Data], _: NumericalConstant<T>) {
                data[$buffer_index].set_octet($octet_index, T.truncate());
            }
            fn validate(data: &[<Ump as Buffer>::Data]) -> Result<()> {
                if u32::from(data[$buffer_index].octet($octet_index)) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u8_ump_numerical_constants_property_impl!(0xFF00_0000, 0x0, 0x0, 0x0, 0, 0);
u8_ump_numerical_constants_property_impl!(0x00FF_0000, 0x0, 0x0, 0x0, 0, 1);
u8_ump_numerical_constants_property_impl!(0x0000_FF00, 0x0, 0x0, 0x0, 0, 2);
u8_ump_numerical_constants_property_impl!(0x0000_00FF, 0x0, 0x0, 0x0, 0, 3);

macro_rules! u8_bytes_numerical_constants_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl<UmpSchema: Schema, const T: u32>
            Property<NumericalConstant<T>, UmpSchema, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for Bytes
        {
            fn get(_data: &[<Bytes as Buffer>::Data]) -> NumericalConstant<T> {
                NumericalConstant()
            }
            fn write(data: &mut [<Bytes as Buffer>::Data], _: NumericalConstant<T>) {
                data[$buffer_index] = T.truncate();
            }
            fn validate(data: &[<Bytes as Buffer>::Data]) -> Result<()> {
                if u32::from(data[$buffer_index]) == T {
                    Ok(())
                } else {
                    Err(Error::InvalidData)
                }
            }
        }
    };
}

u8_bytes_numerical_constants_property_impl!(0xFF, 0x0, 0x0, 0);
u8_bytes_numerical_constants_property_impl!(0x0, 0xFF, 0x0, 1);
u8_bytes_numerical_constants_property_impl!(0x0, 0x0, 0xFF, 2);

macro_rules! bool_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$bit_index:expr) => {
        impl<BytesSchema: Schema> Property<bool, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(data: &[<Ump as Buffer>::Data]) -> bool {
                data[$buffer_index].bit($bit_index)
            }
            fn write(data: &mut [<Ump as Buffer>::Data], v: bool) {
                data[$buffer_index].set_bit($bit_index, v);
            }
        }
    };
}

bool_ump_property_impl!(0x0000_0001, 0x0, 0x0, 0x0, 0, 31);
bool_ump_property_impl!(0x0000_0002, 0x0, 0x0, 0x0, 0, 30);

macro_rules! u4_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl<BytesSchema: Schema> Property<u4, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(data: &[<Ump as Buffer>::Data]) -> u4 {
                data[$buffer_index].nibble($nibble_index)
            }
            fn write(data: &mut [<Ump as Buffer>::Data], v: u4) {
                data[$buffer_index].set_nibble($nibble_index, v);
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

macro_rules! u4_bytes_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr,$nibble_index:expr) => {
        impl<UmpSchema: Schema> Property<u4, UmpSchema, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for Bytes
        {
            fn get(data: &[<Bytes as Buffer>::Data]) -> u4 {
                data[$buffer_index].nibble($nibble_index)
            }
            fn write(data: &mut [<Bytes as Buffer>::Data], v: u4) {
                data[$buffer_index].set_nibble($nibble_index, v);
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
        impl<BytesSchema: Schema> Property<u7, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(data: &[<Ump as Buffer>::Data]) -> u7 {
                data[$buffer_index].octet($octet_index).truncate()
            }
            fn write(data: &mut [<Ump as Buffer>::Data], v: u7) {
                data[$buffer_index].set_octet($octet_index, (v).into());
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
        impl<UmpSchema: Schema> Property<u7, UmpSchema, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for Bytes
        {
            fn get(data: &[<Bytes as Buffer>::Data]) -> u7 {
                data[$buffer_index].truncate()
            }
            fn write(data: &mut [<Bytes as Buffer>::Data], v: u7) {
                data[$buffer_index] = (v).into();
            }
        }
    };
}

u7_bytes_property_impl!(0x7F, 0x0, 0x0, 0);
u7_bytes_property_impl!(0x0, 0x7F, 0x0, 1);
u7_bytes_property_impl!(0x0, 0x0, 0x7F, 2);

macro_rules! u8_bytes_property_impl {
    ($bytes1:expr,$bytes2:expr,$bytes3:expr,$buffer_index:expr) => {
        impl<UmpSchema: Schema> Property<u8, UmpSchema, BytesSchema<$bytes1, $bytes2, $bytes3>>
            for Bytes
        {
            fn get(data: &[<Bytes as Buffer>::Data]) -> u8 {
                data[$buffer_index]
            }
            fn write(data: &mut [<Bytes as Buffer>::Data], v: u8) {
                data[$buffer_index] = v;
            }
        }
    };
}

u8_bytes_property_impl!(0xFF, 0x0, 0x0, 0);
u8_bytes_property_impl!(0x0, 0xFF, 0x0, 1);
u8_bytes_property_impl!(0x0, 0x0, 0xFF, 2);

macro_rules! u8_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl<BytesSchema: Schema> Property<u8, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(data: &[<Ump as Buffer>::Data]) -> u8 {
                data[$buffer_index].octet($octet_index)
            }
            fn write(data: &mut [<Ump as Buffer>::Data], v: u8) {
                data[$buffer_index].set_octet($octet_index, v);
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

macro_rules! u16_ump_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$word_index:expr) => {
        impl<BytesSchema: Schema> Property<u16, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(data: &[<Ump as Buffer>::Data]) -> u16 {
                data[$buffer_index].word($word_index)
            }
            fn write(data: &mut [<Ump as Buffer>::Data], v: u16) {
                data[$buffer_index].set_word($word_index, v);
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
        impl<BytesSchema: Schema> Property<u32, UmpSchema<$ump1, $ump2, $ump3, $ump4>, BytesSchema>
            for Ump
        {
            fn get(data: &[<Ump as Buffer>::Data]) -> u32 {
                data[$buffer_index]
            }
            fn write(data: &mut [<Ump as Buffer>::Data], v: u32) {
                data[$buffer_index] = v;
            }
        }
    };
}

u32_ump_property_impl!(0xFFFF_FFFF, 0x0, 0x0, 0x0, 0);
u32_ump_property_impl!(0x0, 0xFFFF_FFFF, 0x0, 0x0, 1);
u32_ump_property_impl!(0x0, 0x0, 0xFFFF_FFFF, 0x0, 2);
u32_ump_property_impl!(0x0, 0x0, 0x0, 0xFFFF_FFFF, 3);

impl<UmpSchema: Schema> Property<u14, UmpSchema, BytesSchema<0x0, 0x7F, 0x7F>> for Bytes {
    fn get(data: &[<Bytes as Buffer>::Data]) -> u14 {
        u14::from_u7s(&[data[1].truncate(), data[2].truncate()])
    }
    fn write(data: &mut [<Bytes as Buffer>::Data], v: u14) {
        let mut u7s = [u7::default(); 2];
        v.to_u7s(&mut u7s);
        data[1] = u7s[0].into();
        data[2] = u7s[1].into();
    }
}

impl<BytesSchema: Schema> Property<u14, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>, BytesSchema>
    for Ump
{
    fn get(data: &[<Ump as Buffer>::Data]) -> u14 {
        u14::from_u7s(&[data[0].octet(2).truncate(), data[0].octet(3).truncate()])
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: u14) {
        let mut u7s = [u7::default(); 2];
        v.to_u7s(&mut u7s);
        data[0].set_octet(2, u7s[0].into());
        data[0].set_octet(3, u7s[1].into());
    }
}

impl<BytesSchema: Schema>
    Property<Option<u14>, UmpSchema<0x0000_0001, 0x0000_7F7F, 0x0, 0x0>, BytesSchema> for Ump
{
    fn get(data: &[<Ump as Buffer>::Data]) -> Option<u14> {
        if data[0].bit(31) {
            Some(u14::from_u7s(&[
                data[1].octet(2).truncate(),
                data[1].octet(3).truncate(),
            ]))
        } else {
            None
        }
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: Option<u14>) {
        match v {
            Some(v) => {
                let mut u7s = [u7::default(); 2];
                v.to_u7s(&mut u7s);
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

impl<T, UmpSchema: Schema> Property<T, UmpSchema, ()> for Bytes {
    fn get(_: &[<Self as Buffer>::Data]) -> T {
        unreachable!()
    }
    fn write(_: &mut [<Self as Buffer>::Data], _: T) {}
}

impl<T, BytesSchema: Schema> Property<T, (), BytesSchema> for Ump {
    fn get(_: &[<Self as Buffer>::Data]) -> T {
        unreachable!()
    }
    fn write(_: &mut [<Self as Buffer>::Data], _: T) {}
}

#[derive(Default)]
pub struct NumericalConstant<const T: u32>();
