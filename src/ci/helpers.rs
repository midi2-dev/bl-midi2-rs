use crate::{
    ci::DeviceId,
    util::{
        sysex_message::{SysexMessage, SysexMessages},
        Truncate,
    },
    message::system_exclusive_8bit::Message as Sysex8Message,
};

pub fn write_ci_data<'a, M> (
    group: ux::u4,
    device_id: DeviceId,
    category: u8,
    source: ux::u28,
    destination: ux::u28,
    payload: &[ux::u7],
    messages: &'a mut [M],
) -> &'a mut [M] where M : SysexMessage {
    let mut messages = SysexMessages::new(messages, group);
    messages.set_datum(0xF0);
    messages.set_datum(0x7E);
    messages.set_datum(
        match device_id {
            DeviceId::MidiPort => 0x7F,
            DeviceId::Channel(v) => v.into(),
    });
    messages.set_datum(0x0D);
    messages.set_datum(category);
    messages.set_datum(super::VERSION);
    messages.set_datum(source.truncate::<u8>() & 0b0111_1111);
    messages.set_datum((source >> 7).truncate::<u8>() & 0b0111_1111);
    messages.set_datum((source >> 14).truncate::<u8>() & 0b0111_1111);
    messages.set_datum((source >> 21).truncate::<u8>() & 0b0111_1111);
    messages.set_datum(destination.truncate::<u8>() & 0b0111_1111);
    messages.set_datum((destination >> 7).truncate::<u8>() & 0b0111_1111);
    messages.set_datum((destination >> 14).truncate::<u8>() & 0b0111_1111);
    messages.set_datum((destination >> 21).truncate::<u8>() & 0b0111_1111);
    for byte in payload {
        messages.set_datum((*byte).into());
    }
    messages.messages()
}

pub fn write_stream_id(messages: &mut [Sysex8Message], stream_id: u8) {
    for m in messages {
        *m.stream_id_mut() = stream_id;
    }
}