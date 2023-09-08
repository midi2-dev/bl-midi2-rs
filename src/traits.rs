use crate::{util::Truncate, *};

pub trait Message<'a, B: Buffer>: Sized {
    fn data(&self) -> &'a B::Data;
    fn from_data(buffer: &'a B::Data) -> Result<Self> {
        match Self::validate_data(buffer) {
            Ok(()) => Ok(Self::from_data_unchecked(buffer)),
            Err(e) => Err(e),
        }
    }
    fn from_data_unchecked(buffer: &'a B::Data) -> Self;
    fn validate_data(buffer: &'a B::Data) -> Result<()>;
}

pub trait Buildable<'a, B: Buffer>: Message<'a, B> {
    type Builder: Builder<'a, B, Message = Self>;
    fn builder(buffer: &'a mut B::Data) -> Self::Builder {
        Self::Builder::new(buffer)
    }
}

pub trait Builder<'a, B: Buffer> {
    type Message;
    fn new(buffer: &'a mut B::Data) -> Self;
    fn build(self) -> Result<Self::Message>;
}

pub trait GroupedMessage<'a>: Message<'a, Ump> {
    fn group(&self) -> u4;
}

pub trait GroupedBuilder<'a>: Builder<'a, Ump> {
    fn group(self, group: u4) -> Self;
}

pub trait SysexMessage<'a, B: Buffer>: Message<'a, B> {
    type PayloadIterator: core::iter::Iterator<Item = u8>;
    fn payload(&self) -> Self::PayloadIterator;
}

pub trait Byte: Copy {
    fn to_u8(self) -> u8;
    fn from_u8(v: u8) -> Self;
}

impl Byte for u8 {
    fn to_u8(self) -> u8 {
        self
    }
    fn from_u8(v: u8) -> Self {
        v
    }
}

impl Byte for u7 {
    fn to_u8(self) -> u8 {
        self.into()
    }
    fn from_u8(v: u8) -> Self {
        v.truncate()
    }
}

pub trait SysexGroupBuilder<'a, B: Buffer>: Builder<'a, B> {
    type Byte: Byte;
    fn payload<I: core::iter::Iterator<Item = Self::Byte>>(self, data: I) -> Self;
}

pub trait StreamedMessage<'a>: Message<'a, Ump> {
    fn stream_id(&self) -> u8;
}

pub trait StreamedBuilder<'a>: Builder<'a, Ump> {
    fn stream_id(self, id: u8) -> Self;
}
