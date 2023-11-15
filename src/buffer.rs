pub trait Buffer {
    type Data: core::default::Default + core::marker::Copy;
    type Size: generic_array::ArrayLength;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ump();

impl Buffer for Ump {
    type Data = u32;
    type Size = generic_array::typenum::U4;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes();

impl Buffer for Bytes {
    type Data = u8;
    type Size = generic_array::typenum::U3;
}
