use crate::{
    error::Error,
    util::{
        sysex_message,
        Truncate,
        BitOps,
    },
    ci::{helpers, DeviceId}, 
};

pub struct DiscoveryData {
    pub group: ux::u4,
    pub category: u8,
    pub source: ux::u28,
    pub destination: ux::u28,
    pub device_manufacturer: ux::u21,
    pub device_family: ux::u14,
    pub device_model_number: ux::u14,
    pub software_version: [ux::u7; 4],
    pub protocol_negotiation_supported: bool,
    pub profile_configuration_supported: bool,
    pub property_exchange_supported: bool,
    pub max_sysex_message_size: ux::u28,
}

pub fn write_discovery_data<'a, M: sysex_message::SysexMessage>(
    messages: &'a mut [M],
    data: &DiscoveryData,
) -> &'a mut [M] {
    helpers::write_ci_data(
        data.group,
        DeviceId::MidiPort,
        data.category,
        data.source,
        data.destination,
        &[
            data.device_manufacturer.truncate(),
            (data.device_manufacturer >> 7).truncate(),
            (data.device_manufacturer >> 14).truncate(),
            data.device_family.truncate(),
            (data.device_family >> 7).truncate(),
            data.device_model_number.truncate(),
            (data.device_model_number >> 7).truncate(),
            data.software_version[0],
            data.software_version[1],
            data.software_version[2],
            data.software_version[3],
            {
                let flags = *0x0_u8
                    .set_bit(6, data.protocol_negotiation_supported)
                    .set_bit(5, data.profile_configuration_supported)
                    .set_bit(4, data.property_exchange_supported);
                ux::u7::new(flags)
            },
            data.max_sysex_message_size.truncate(),
            (data.max_sysex_message_size >> 7).truncate(),
            (data.max_sysex_message_size >> 14).truncate(),
            (data.max_sysex_message_size >> 21).truncate(),
        ],
        messages,
    )
}

pub fn device_manufacturer<M: sysex_message::SysexMessage>(messages: &sysex_message::SysexMessages<M>) -> ux::u21 {
    ux::u21::from(messages.datum(13) & 0b0111_1111)
    | ux::u21::from(messages.datum(14) & 0b0111_1111) << 7
    | ux::u21::from(messages.datum(15) & 0b0111_1111) << 14
}

pub fn device_family<M: sysex_message::SysexMessage>(messages: &sysex_message::SysexMessages<M>) -> ux::u14 {
    ux::u14::from(messages.datum(16) & 0b0111_1111)
    | ux::u14::from(messages.datum(17) & 0b0111_1111) << 7
}

pub fn device_model_number<M: sysex_message::SysexMessage>(messages: &sysex_message::SysexMessages<M>) -> ux::u14 {
    ux::u14::from(messages.datum(18) & 0b0111_1111)
    | ux::u14::from(messages.datum(19) & 0b0111_1111) << 7
}

pub fn software_version<M: sysex_message::SysexMessage>(messages: &sysex_message::SysexMessages<M>) -> [ux::u7; 4] {
    [
        messages.datum(20).truncate(),
        messages.datum(21).truncate(),
        messages.datum(22).truncate(),
        messages.datum(23).truncate(),
    ]
}
pub fn max_sysex_message_size<M: sysex_message::SysexMessage>(messages: &sysex_message::SysexMessages<M>) -> ux::u28 {
    ux::u28::from(messages.datum(25) & 0b0111_1111)
    | ux::u28::from(messages.datum(26) & 0b0111_1111) << 7
    | ux::u28::from(messages.datum(27) & 0b0111_1111) << 14
    | ux::u28::from(messages.datum(28) & 0b0111_1111) << 21
}

pub struct SupportFlags {
    pub protocol_negotiation_supported: bool,
    pub profile_configuration_supported: bool,
    pub property_exchange_supported: bool,
}

pub fn support_flags<M: sysex_message::SysexMessage>(messages: &sysex_message::SysexMessages<M>) -> SupportFlags {
    let byte = messages.datum(24);
    SupportFlags {
        protocol_negotiation_supported: byte.bit(6),
        profile_configuration_supported: byte.bit(5),
        property_exchange_supported: byte.bit(4),
    }
}

pub fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M], size: usize) -> Result<(), Error> {
    let messages = sysex_message::SysexMessages(messages);
    if messages.len() != size || messages.datum(1) != 0x7F {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(messages: &[M], size: usize) -> Result<(), Error> {
    let messages = sysex_message::SysexMessages(messages);
    if messages.max_len() < size {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
    }
}