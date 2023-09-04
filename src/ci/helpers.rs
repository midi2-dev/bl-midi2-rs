use crate::{
    ci::{DeviceId, SYSEX_START},
    error::Error,
    result::Result,
    util::{Encode7Bit, Truncate},
    *,
};

pub struct StandardDataIterator<'a, Repr>
where
    Repr: SysexGroupMessage<'a>,
    <Repr as Message<'a>>::Builder: SysexGroupBuilder<'a>,
{
    data: [<<Repr as Message<'a>>::Builder as SysexGroupBuilder<'a>>::Byte; 16],
    i: usize,
    _phantom: core::marker::PhantomData<&'a Repr>,
}

impl<'a, Repr> StandardDataIterator<'a, Repr>
where
    Repr: SysexGroupMessage<'a>,
    <Repr as Message<'a>>::Builder: SysexGroupBuilder<'a>,
    <<Repr as Message<'a>>::Builder as SysexGroupBuilder<'a>>::Byte: Byte,
{
    fn map(v: u8) -> <<Repr as Message<'a>>::Builder as SysexGroupBuilder<'a>>::Byte {
        <<<Repr as Message<'a>>::Builder as SysexGroupBuilder<'a>>::Byte as Byte>::from_u8(v)
    }
    pub fn new(device_id: DeviceId, category: u8, source: u28, destination: u28) -> Self {
        StandardDataIterator::<'a, Repr> {
            data: [
                Self::map(0x7E),
                match device_id {
                    DeviceId::MidiPort => Self::map(0x7F),
                    DeviceId::Channel(v) => Self::map(u8::from(v)),
                },
                Self::map(0x0D),
                Self::map(category),
                Self::map(super::VERSION),
                Self::map(source.truncate::<u8>() & 0b0111_1111),
                Self::map((source >> 7).truncate::<u8>() & 0b0111_1111),
                Self::map((source >> 14).truncate::<u8>() & 0b0111_1111),
                Self::map((source >> 21).truncate::<u8>() & 0b0111_1111),
                Self::map(destination.truncate::<u8>() & 0b0111_1111),
                Self::map((destination >> 7).truncate::<u8>() & 0b0111_1111),
                Self::map((destination >> 14).truncate::<u8>() & 0b0111_1111),
                Self::map((destination >> 21).truncate::<u8>() & 0b0111_1111),
                Self::map(0x0),
                Self::map(0x0),
                Self::map(0x0), // padding
            ],
            i: 0,
            _phantom: Default::default(),
        }
    }
}

impl<'a, Repr> core::iter::Iterator for StandardDataIterator<'a, Repr>
where
    Repr: SysexGroupMessage<'a>,
    <Repr as Message<'a>>::Builder: SysexGroupBuilder<'a>,
{
    type Item = <<Repr as Message<'a>>::Builder as SysexGroupBuilder<'a>>::Byte;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 13 {
            None
        } else {
            let current = self.i;
            self.i += 1;
            Some(self.data[current])
        }
    }
}

pub const STANDARD_DATA_SIZE: usize = 14;

pub fn device_id_from_u8(v: u8) -> Result<DeviceId> {
    if v == 0x7F {
        Ok(DeviceId::MidiPort)
    } else if v < 0x0F {
        Ok(DeviceId::Channel(v.try_into().unwrap()))
    } else {
        Err(Error::InvalidData)
    }
}

pub fn validate_sysex<'a, Repr>(buffer: &'a [u32], status: u8) -> Result<Repr>
where
    Repr: SysexGroupMessage<'a>,
    <Repr as Message<'a>>::Builder: SysexGroupBuilder<'a>,
{
    let messages = Repr::from_data(buffer)?;
    let mut payload = messages.payload();
    let Some(SYSEX_START) = payload.next() else {
        return Err(Error::InvalidData);
    };
    let Some(0x7E) = payload.next() else {
        return Err(Error::InvalidData);
    };
    if let Some(v) = payload.next() {
        device_id_from_u8(v)?;
    } else {
        return Err(Error::InvalidData);
    };
    // midi ci status code
    if let Some(v) = payload.next() {
        if v == status {
            return Err(Error::InvalidData);
        }
    };
    if let Some(v) = payload.next() {
        if v != status {
            return Err(Error::InvalidData);
        }
    };
    payload.next(); // todo: version compat
                    // source / destination
    let Some(_) = payload.nth(7) else {
        return Err(Error::InvalidData);
    };
    Ok(messages)
}

pub struct ProtocolDataIterator {
    buffer: [u8; 8],
    i: usize,
}

impl core::iter::Iterator for ProtocolDataIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 5 {
            None
        } else {
            let current = self.i;
            self.i += 1;
            Some(self.buffer[current])
        }
    }
}

pub fn destination_from_payload<I: core::iter::Iterator<Item = u8>>(mut payload: I) -> u28 {
    payload.nth(9);
    u28::from_u7s(&[
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
    ])
}

pub fn source_from_payload<I: core::iter::Iterator<Item = u8>>(mut payload: I) -> u28 {
    payload.nth(5);
    u28::from_u7s(&[
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
    ])
}
