use crate::{
    ci::{DeviceId, Protocol},
    error::Error,
    message::system_exclusive_7bit as sysex7,
    message::system_exclusive_8bit as sysex8,
    result::Result,
    util::{BitOps, Encode7Bit, Truncate},
};

pub struct StandardDataIterator {
    data: [u8; 16],
    i: usize,
}

impl StandardDataIterator {
    pub fn new(device_id: DeviceId, category: u8, source: ux::u28, destination: ux::u28) -> Self {
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

fn device_id_from_u8(v: u8) -> Result<DeviceId> {
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
        if v != ux::u7::new(0x7E) {
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

pub fn protocol_data(protocol: &Protocol) -> ProtocolDataIterator {
    match protocol {
        Protocol::Midi1 {
            size_of_packet_extension,
            jitter_reduction_extension,
            version,
        } => {
            ProtocolDataIterator {
                buffer: [
                    0x1,
                    (*version).into(),
                    *0x0_u8
                        .set_bit(6, *size_of_packet_extension)
                        .set_bit(7, *jitter_reduction_extension),
                    0x0,
                    0x0, // reserved
                    0x0,
                    0x0,
                    0x0, // padding
                ],
                i: 0,
            }
        }
        Protocol::Midi2 {
            jitter_reduction_extension,
            version,
        } => {
            ProtocolDataIterator {
                buffer: [
                    0x2,
                    (*version).into(),
                    *0x0_u8.set_bit(7, *jitter_reduction_extension),
                    0x0,
                    0x0, // reserved
                    0x0,
                    0x0,
                    0x0, // padding
                ],
                i: 0,
            }
        }
    }
}

pub fn read_protocol<I: core::iter::Iterator<Item = u8>>(mut data: I) -> Result<Protocol> {
    let ty = data.next().unwrap();
    let version = data.next().unwrap();
    let flags = data.next().unwrap();

    // reserved data
    data.next().unwrap();
    data.next().unwrap();

    match ty {
        0x1 => Ok(Protocol::Midi1 {
            size_of_packet_extension: flags.bit(6),
            jitter_reduction_extension: flags.bit(7),
            version: version.truncate(),
        }),
        0x2 => Ok(Protocol::Midi2 {
            jitter_reduction_extension: flags.bit(7),
            version: version.truncate(),
        }),
        _ => Err(Error::InvalidData),
    }
}

pub fn validate_protocol_data<I: core::iter::Iterator<Item = u8>>(mut data: I) -> Result<()> {
    if let Some(v) = data.next() {
        if ![1u8, 2u8].iter().any(|&code| code == v) {
            return Err(Error::InvalidData);
        }
    }
    // todo: version assertion?
    if data.nth(3).is_some() {
        Ok(())
    } else {
        Err(Error::InvalidData)
    }
}

pub fn source_from_payload<I: core::iter::Iterator<Item = u8>>(mut payload: I) -> ux::u28 {
    payload.nth(4);
    ux::u28::from_u7s(&[
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
    ])
}

pub fn destination_from_payload<I: core::iter::Iterator<Item = u8>>(mut payload: I) -> ux::u28 {
    payload.nth(8);
    ux::u28::from_u7s(&[
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
        payload.next().unwrap(),
    ])
}

pub fn authority_level_from_payload<I: core::iter::Iterator<Item = u8>>(mut payload: I) -> ux::u7 {
    payload.nth(STANDARD_DATA_SIZE).unwrap().truncate()
}
