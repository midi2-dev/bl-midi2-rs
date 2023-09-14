pub trait Buffer {
    type Data: ?Sized;
    const SIZE: usize;
    fn len(buffer: &Self::Data) -> usize;
    fn crop(buffer: &mut Self::Data) -> &mut Self::Data;
    fn clear(buffer: &mut Self::Data) -> &mut Self::Data;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ump();

impl Buffer for Ump {
    type Data = [u32];
    const SIZE: usize = 4;
    fn len(buffer: &Self::Data) -> usize {
        buffer.len()
    }
    fn crop(buffer: &mut Self::Data) -> &mut Self::Data {
        &mut buffer[..Self::SIZE]
    }
    fn clear(buffer: &mut Self::Data) -> &mut Self::Data {
        for d in &mut *buffer {
            *d = 0x0;
        }
        buffer
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes();

impl Buffer for Bytes {
    type Data = [u8];
    const SIZE: usize = 3;
    fn len(buffer: &Self::Data) -> usize {
        buffer.len()
    }
    fn crop(buffer: &mut Self::Data) -> &mut Self::Data {
        &mut buffer[..Self::SIZE]
    }
    fn clear(buffer: &mut Self::Data) -> &mut Self::Data {
        for d in &mut *buffer {
            *d = 0x0;
        }
        buffer
    }
}
