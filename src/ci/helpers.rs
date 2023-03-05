use crate::{
    result::Result,
    ci::DeviceId,
    error::Error,
    util::Truncate,
    message::system_exclusive_8bit as sysex8,
    message::system_exclusive_7bit as sysex7,
};

pub struct StandardDataIterator {
    data: [u8; 16],
    i: usize,
}

impl StandardDataIterator {
    pub fn new(
        device_id: DeviceId,
        category: u8,
        source: ux::u28,
        destination: ux::u28,
    ) -> Self {
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
                0x0, 0x0, 0x0, // padding
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

pub fn validate_sysex8(
    buffer: &[u32],
    status: u8,
) -> Result<sysex8::Sysex8MessageGroup> {
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

pub fn validate_sysex7(
    buffer: &[u32],
    status: u8,
) -> Result<sysex7::Sysex7MessageGroup> {
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


/*
pub struct StandardData {
    pub device_id: DeviceId,
    pub source: ux::u28,
    pub destination: ux::u28,
}

pub fn read_standard_data<M: sysex_message::SysexMessage>(messages: &[M]) -> StandardData {
    let messages = sysex_message::SysexMessages::new(messages);
    StandardData {
        device_id: match messages.datum(1) {
            0x7F => DeviceId::MidiPort,
            v => DeviceId::Channel(v.truncate()),
        },
        source: ux::u28::from_u7s(&[
            messages.datum(5),
            messages.datum(6),
            messages.datum(7),
            messages.datum(8),
        ]),
        destination: ux::u28::from_u7s(&[
            messages.datum(9),
            messages.datum(10),
            messages.datum(11),
            messages.datum(12),
        ]),
    }
}

pub fn read_protocol<M>(messages: &sysex_message::SysexMessages<M>, first_byte: usize) -> Protocol
where
    M: sysex_message::SysexMessage,
{
    match messages.datum(first_byte) {
        0x1 => {
            let flags = messages.datum(first_byte + 2);
            Protocol::Midi1 {
                size_of_packet_extension: flags.bit(6),
                jitter_reduction_extension: flags.bit(7),
                version: messages.datum(first_byte + 1).truncate(),
            }
        },
        0x2 => {
            Protocol::Midi2 {
                jitter_reduction_extension: messages.datum(first_byte + 2).bit(7),
                version: messages.datum(first_byte + 1).truncate(),
            }
        }
        _ => panic!(),
    }
}

pub fn validate_protocol_data(data: &[u8]) -> Result<(), Error> {
    // todo: version assertion?
    if ![1u8, 2u8].iter().any(|&v| v == data[0]) {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn protocol_data<'a, 'b>(protocol: &'a Protocol, buff: &'b mut [ux::u7]) -> &'b [ux::u7] {
    match protocol {
        Protocol::Midi1 {
            size_of_packet_extension,
            jitter_reduction_extension,
            version,
        } => {
            buff[0] = ux::u7::new(0x1);
            buff[1] = *version;
            buff[2] = ux::u7::new(
                *0x0_u8
                    .set_bit(6, *size_of_packet_extension)
                    .set_bit(7, *jitter_reduction_extension),
            );
            buff[3] = ux::u7::default();
            buff[4] = ux::u7::default();
        }
        Protocol::Midi2 {
            jitter_reduction_extension,
            version,
        } => {
            buff[0] = ux::u7::new(0x2);
            buff[1] = *version;
            buff[2] = ux::u7::new(*0x0_u8.set_bit(7, *jitter_reduction_extension));
            buff[3] = ux::u7::default();
            buff[4] = ux::u7::default();
        }
    }
    buff
}
*/