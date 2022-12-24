use crate::{
    ci::{helpers, CiMessageDetail},
    error::Error,
    util::{builder, getter, sysex_message}
};

pub struct Message {
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    device_manufacturer: ux::u21,
    device_family: ux::u14,
    device_model_number: ux::u14,
    software_version: [ux::u7; 4],
    protocol_negotiation_supported: bool,
    profile_configuration_supported: bool,
    property_exchange_supported: bool,
    max_sysex_message_size: ux::u28,
}

builder::builder!(
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    device_manufacturer: ux::u21,
    device_family: ux::u14,
    device_model_number: ux::u14,
    software_version: [ux::u7; 4],
    protocol_negotiation_supported: bool,
    profile_configuration_supported: bool,
    property_exchange_supported: bool,
    max_sysex_message_size: ux::u28
);

impl Message {
    const STATUS: u8 = 0x71;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
    getter::getter!(destination, ux::u28);
    getter::getter!(device_manufacturer, ux::u21);
    getter::getter!(device_family, ux::u14);
    getter::getter!(device_model_number, ux::u14);
    getter::getter!(software_version, [ux::u7; 4]);
    getter::getter!(protocol_negotiation_supported, bool);
    getter::getter!(profile_configuration_supported, bool);
    getter::getter!(property_exchange_supported, bool);
    getter::getter!(max_sysex_message_size, ux::u28);
}

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: sysex_message::SysexMessage>(
        &self,
        messages: &'a mut [M],
    ) -> &'a mut [M] {
        super::helpers::write_discovery_data(
            messages,
            &super::helpers::DiscoveryData {
                group: self.group,
                category: Message::STATUS,
                source: self.source,
                destination: self.destination,
                device_manufacturer: self.device_manufacturer,
                device_family: self.device_family,
                device_model_number: self.device_model_number,
                software_version: self.software_version,
                protocol_negotiation_supported: self.protocol_negotiation_supported,
                profile_configuration_supported: self.profile_configuration_supported,
                property_exchange_supported: self.property_exchange_supported,
                max_sysex_message_size: self.max_sysex_message_size,
            },
        )
    }

    fn from_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Self {
        let standard_data = helpers::read_standard_data(messages);
        let messages = sysex_message::SysexMessages(messages);
        let support_flags = super::helpers::support_flags(&messages);
        Message {
            group: messages.group(),
            source: standard_data.source,
            destination: standard_data.destination,
            device_manufacturer: super::helpers::device_manufacturer(&messages),
            device_family: super::helpers::device_family(&messages),
            device_model_number: super::helpers::device_model_number(&messages),
            software_version: super::helpers::software_version(&messages),
            max_sysex_message_size: super::helpers::max_sysex_message_size(&messages),
            protocol_negotiation_supported: support_flags.protocol_negotiation_supported,
            profile_configuration_supported: support_flags.profile_configuration_supported,
            property_exchange_supported: support_flags.property_exchange_supported,
        }
    }
    fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        helpers::validate_sysex(messages, Message::STATUS)?;
        super::helpers::validate_sysex(messages, super::DATA_SIZE)
    }
    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        helpers::validate_buffer_size(messages, super::DATA_SIZE)
    }
}