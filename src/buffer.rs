pub trait Buffer {
    type Data: core::default::Default + core::marker::Copy;
    const SIZE: usize;
}

pub struct Bytes;
pub struct Ump;

impl Buffer for Ump {
    type Data = u32;
    const SIZE: usize = 4;
}

impl Buffer for Bytes {
    type Data = u8;
    const SIZE: usize = 3;
}
