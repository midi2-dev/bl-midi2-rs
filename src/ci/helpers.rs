use crate::{
    ci::DeviceId,
    error::Error,
    message::system_exclusive_7bit as sysex7,
    message::system_exclusive_8bit as sysex8,
    result::Result,
    util::{Encode7Bit, Truncate},
    *,
};

pub struct StandardDataIterator {
    data: [u8; 16],
    i: usize,
}

impl StandardDataIterator {
    pub fn new(device_id: DeviceId, category: u8, source: u28, destination: u28) -> Self {
        StandardDataIterator {
            data: [
                0x7E,
                match device_id {
                    DeviceId::MidiPort => 0x7F,
                    DeviceId::Channel(v) => v.into(),
                },
                0x0D,
                category,
                super::VERSION,
                source.truncate::<u8>() & 0b0111_1111,
                (source >> 7).truncate::<u8>() & 0b0111_1111,
                (source >> 14).truncate::<u8>() & 0b0111_1111,
                (source >> 21).truncate::<u8>() & 0b0111_1111,
                destination.truncate::<u8>() & 0b0111_1111,
                (destination >> 7).truncate::<u8>() & 0b0111_1111,
                (destination >> 14).truncate::<u8>() & 0b0111_1111,
                (destination >> 21).truncate::<u8>() & 0b0111_1111,
                0x0,
                0x0,
                0x0, // padding
            ],
            i: 0,
        }
    }
}

impl core::iter::Iterator for StandardDataIterator {
    type Item = u8;
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

pub const STANDARD_DATA_SIZE: usize = 13;

pub fn validate_sysex8(buffer: &[u32], status: u8) -> Result<sysex8::Sysex8MessageGroup> {
    let messages = sysex8::Sysex8MessageGroup::from_data(buffer)?;
    let mut payload = messages.payload();
    let Some(0x7E) = payload.next() else {
        return Err(Error::InvalidData);
    };
    if let Some(v) = payload.next() {
        device_id_from_u8(v)?;
    } else {
        return Err(Error::InvalidData);
    };
    // midi ci status code
    let Some(0x0D) = payload.next() else {
        return Err(Error::InvalidData);
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

pub fn device_id_from_u8(v: u8) -> Result<DeviceId> {
    if v == 0x7F {
        Ok(DeviceId::MidiPort)
    } else if v < 0x0F {
        Ok(DeviceId::Channel(v.try_into().unwrap()))
    } else {
        Err(Error::InvalidData)
    }
}

pub fn validate_sysex7(buffer: &[u32], status: u8) -> Result<sysex7::Sysex7MessageGroup> {
    let messages = sysex7::Sysex7MessageGroup::from_data(buffer)?;
    let mut payload = messages.payload();
    if let Some(v) = payload.next() {
        if v != u7::new(0x7E) {
            return Err(Error::InvalidData);
        }
    };
    if let Some(v) = payload.next() {
        device_id_from_u8(v.into())?;
    } else {
        return Err(Error::InvalidData);
    };
    // midi ci status code
    if let Some(v) = payload.next() {
        if u8::from(v) == status {
            return Err(Error::InvalidData);
        }
    };
    if let Some(v) = payload.next() {
        if u8::from(v) != status {
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
    payload.nth(8);
    u28::from_u7s(&[
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
    ])
}
