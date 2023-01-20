use crate::{
    ci::{DeviceId, protocol::Protocol},
    error::Error,
    message::system_exclusive_8bit::Message as Sysex8Message,
    util::{sysex_message, BitOps, Encode7Bit, Truncate},
};

pub fn write_ci_data<'a, M>(
    group: ux::u4,
    device_id: DeviceId,
    category: u8,
    source: ux::u28,
    destination: ux::u28,
    payload: &[ux::u7],
    messages: &'a mut [M],
) -> &'a mut [M]
where
    M: sysex_message::SysexMessage,
{
    let mut messages_builder = sysex_message::SysexMessagesMut::builder(messages, group);
    messages_builder.datum(0x7E);
    messages_builder.datum(match device_id {
        DeviceId::MidiPort => 0x7F,
        DeviceId::Channel(v) => v.into(),
    });
    messages_builder.datum(0x0D);
    messages_builder.datum(category);
    messages_builder.datum(super::VERSION);
    messages_builder.datum(source.truncate::<u8>() & 0b0111_1111);
    messages_builder.datum((source >> 7).truncate::<u8>() & 0b0111_1111);
    messages_builder.datum((source >> 14).truncate::<u8>() & 0b0111_1111);
    messages_builder.datum((source >> 21).truncate::<u8>() & 0b0111_1111);
    messages_builder.datum(destination.truncate::<u8>() & 0b0111_1111);
    messages_builder.datum((destination >> 7).truncate::<u8>() & 0b0111_1111);
    messages_builder.datum((destination >> 14).truncate::<u8>() & 0b0111_1111);
    messages_builder.datum((destination >> 21).truncate::<u8>() & 0b0111_1111);
    for byte in payload {
        messages_builder.datum((*byte).into());
    }
    messages_builder.build().0
}

pub fn write_stream_id(messages: &mut [Sysex8Message], stream_id: u8) {
    for m in messages {
        *m.stream_id_mut() = stream_id;
    }
}

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

pub fn validate_sysex<M: sysex_message::SysexMessage>(
    messages: &[M],
    status: u8,
) -> Result<(), Error> {
    let messages = sysex_message::SysexMessages::new(messages);
    let l = messages.len();
    if !messages.valid()
        || l < 13
        || messages.datum(0) != 0x7E
        || messages.datum(2) != 0x0D
        || messages.datum(3) != status
    {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn validate_buffer_size<M: sysex_message::SysexMessage>(
    messages: &[M],
    min_size: usize,
) -> Result<(), Error> {
    let messages = sysex_message::SysexMessages::new(messages);
    if messages.max_len() < min_size {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
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