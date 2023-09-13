pub trait Buffer {
    type Data: ?Sized;
    const SIZE: usize;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ump();

impl Buffer for Ump {
    type Data = [u32];
    const SIZE: usize = 4;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes();

impl Buffer for Bytes {
    type Data = [u8];
    const SIZE: usize = 3;
}
