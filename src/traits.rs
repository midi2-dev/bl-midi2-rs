use crate::{util::Truncate, *};

pub trait Message<'a>: Sized {
    type Builder: Builder<'a, Message = Self>;
    fn builder(buffer: &'a mut [u32]) -> Self::Builder {
        Self::Builder::new(buffer)
    }
    fn data(&self) -> &'a [u32];
    fn from_data(buffer: &'a [u32]) -> Result<Self> {
        match Self::validate_data(buffer) {
            Ok(()) => Ok(Self::from_data_unchecked(buffer)),
            Err(e) => Err(e),
        }
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()>;
}

pub trait Builder<'a> {
    type Message;
    fn new(buffer: &'a mut [u32]) -> Self;
    fn build(self) -> Result<Self::Message>;
}

pub trait GroupedMessage<'a>
where
    Self: Message<'a>,
    <Self as Message<'a>>::Builder: GroupedBuilder<'a>,
{
    fn group(&self) -> u4;
}

pub trait GroupedBuilder<'a>: Builder<'a> {
    fn group(self, group: u4) -> Self;
}

pub trait SysexGroupMessage<'a>
where
    Self: Message<'a>,
    <Self as Message<'a>>::Builder: SysexGroupBuilder<'a>,
{
    type PayloadIterator: core::iter::Iterator<Item = u8>;
    type Message: Message<'a>;
    type MessageIterator: core::iter::Iterator<Item = Self::Message>;
    fn payload(&self) -> Self::PayloadIterator;
    fn messages(&self) -> Self::MessageIterator;
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

pub trait SysexGroupBuilder<'a>: Builder<'a> {
    type Byte: Byte;
    fn payload<I: core::iter::Iterator<Item = Self::Byte>>(self, data: I) -> Self;
}

pub trait StreamedMessage<'a>
where
    Self: Message<'a>,
    <Self as Message<'a>>::Builder: StreamedBuilder<'a>,
{
    fn stream_id(&self) -> u8;
}

pub trait StreamedBuilder<'a>: Builder<'a> {
    fn stream_id(self, id: u8) -> Self;
}
