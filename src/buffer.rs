pub trait Buffer {
    type Data: ?Sized;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ump();

impl Buffer for Ump {
    type Data = [u32];
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes();

impl Buffer for Bytes {
    type Data = [u8];
}
