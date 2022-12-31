use crate::{
    ci::{helpers, CiMessageDetail},
    error::Error,
    util::{builder, getter, sysex_message},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    source: ux::u28,
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
    const STATUS: u8 = 0x70;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
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
    fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        super::helpers::write_discovery_data(
            messages,
            &super::helpers::DiscoveryData {
                group: self.group,
                category: Message::STATUS,
                source: self.source,
                destination: ux::u28::new(0xFFF_FFFF),
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
    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(
        &self,
        messages: &[M],
    ) -> Result<(), Error> {
        helpers::validate_buffer_size(messages, super::DATA_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::{CiMessage, VERSION},
        message::system_exclusive_7bit as sysex7,
        message::system_exclusive_8bit as sysex8,
    };

    #[test]
    #[rustfmt::skip]
    fn to_sysex8() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x8),
                source: ux::u28::new(0x123_1000),
                device_manufacturer: ux::u21::new(0x13_8800),
                device_family: ux::u14::new(0x3999),
                device_model_number: ux::u14::new(0x1990),
                software_version: [
                    ux::u7::new(0x1),
                    ux::u7::new(0x6),
                    ux::u7::new(0x5),
                    ux::u7::new(0x31),
                ],
                protocol_negotiation_supported: true,
                profile_configuration_supported: true,
                property_exchange_supported: false,
                max_sysex_message_size: ux::u28::new(0x10_0000),
            }.try_to_sysex8(
                &mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ],
                0x31
            ).unwrap(),
            &[
                sysex8::Message::builder()
                    .stream_id(0x31)
                    .group(ux::u4::new(0x8))
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x70, // universal sysex sub-id 2
                        VERSION,
                        0b0000000, 0b0100000, 0b0001100, 0b0001001, // source muid
                        0x7F, 0x7F, 0x7F,  // destination muid
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x31)
                    .group(ux::u4::new(0x8))
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x7F, // destination muid
                        0b0000000, 0b0010000, 0b1001110, // device manufacturer
                        0b0011001, 0b1110011, // device family
                        0b0010000, 0b0110011, // device model number
                        0x1, 0x6, 0x5, 0x31, // software version
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x31)
                    .group(ux::u4::new(0x8))
                    .status(sysex8::Status::End)
                    .data(&[ 
                        0b0000_0110, // support flags
                        0x0, 0x0, 0b1000000, 0x0, // max sysex size
                    ])
                    .build(),
            ],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex8() {
        assert_eq!(
            Message::try_from_sysex8(&[
                sysex8::Message::builder()
                    .stream_id(0x31)
                    .group(ux::u4::new(0x8))
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x70, // universal sysex sub-id 2
                        VERSION,
                        0b0000000, 0b0100000, 0b0001100, 0b0001001, // source muid
                        0x7F, 0x7F, 0x7F,  // destination muid
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x31)
                    .group(ux::u4::new(0x8))
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x7F, // destination muid
                        0b0000000, 0b0010000, 0b1001110, // device manufacturer
                        0b0011001, 0b1110011, // device family
                        0b0010000, 0b0110011, // device model number
                        0x1, 0x6, 0x5, 0x31, // software version
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x31)
                    .group(ux::u4::new(0x8))
                    .status(sysex8::Status::End)
                    .data(&[ 
                        0b0000_0110, // support flags
                        0x0, 0x0, 0b1000000, 0x0, // max sysex size
                    ])
                    .build(),
            ]),
            Ok(Message {
                group: ux::u4::new(0x8),
                source: ux::u28::new(0x123_1000),
                device_manufacturer: ux::u21::new(0x13_8800),
                device_family: ux::u14::new(0x3999),
                device_model_number: ux::u14::new(0x1990),
                software_version: [
                    ux::u7::new(0x1),
                    ux::u7::new(0x6),
                    ux::u7::new(0x5),
                    ux::u7::new(0x31),
                ],
                protocol_negotiation_supported: true,
                profile_configuration_supported: true,
                property_exchange_supported: false,
                max_sysex_message_size: ux::u28::new(0x10_0000),
            }),
        )
    }

    #[test]
    #[rustfmt::skip]
    fn to_sysex7() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x8),
                source: ux::u28::new(0x123_1000),
                device_manufacturer: ux::u21::new(0x13_8800),
                device_family: ux::u14::new(0x3999),
                device_model_number: ux::u14::new(0x1990),
                software_version: [
                    ux::u7::new(0x1),
                    ux::u7::new(0x6),
                    ux::u7::new(0x5),
                    ux::u7::new(0x31),
                ],
                protocol_negotiation_supported: true,
                profile_configuration_supported: true,
                property_exchange_supported: false,
                max_sysex_message_size: ux::u28::new(0x10_0000),
            }.try_to_sysex7(
                &mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ],
            ).unwrap(),
            &[
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x70), // universal sysex sub-id 2
                        ux::u7::new(VERSION),
                        ux::u7::new(0b0000000), // source muid
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b0100000), 
                        ux::u7::new(0b0001100), ux::u7::new(0b0001001), // source muid
                        ux::u7::new(0x7F), ux::u7::new(0x7F),  // destination muid
                        ux::u7::new(0x7F),
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x7F), // destination muid
                        ux::u7::new(0b0000000), ux::u7::new(0b0010000), 
                        ux::u7::new(0b1001110), // device manufacturer
                        ux::u7::new(0b0011001), ux::u7::new(0b1110011), // device family
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b0010000), ux::u7::new(0b0110011), // device model number
                        ux::u7::new(0x1), ux::u7::new(0x6), // software version
                        ux::u7::new(0x5), ux::u7::new(0x31), 
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::End)
                    .data(&[ 
                        ux::u7::new(0b0000_0110), // support flags
                        ux::u7::new(0x0), ux::u7::new(0x0), 
                        ux::u7::new(0b1000000), ux::u7::new(0x0), // max sysex size
                    ])
                    .build(),
            ],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex7() {
        assert_eq!(
            Message::try_from_sysex7(&[
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x70), // universal sysex sub-id 2
                        ux::u7::new(VERSION),
                        ux::u7::new(0b0000000), // source muid
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b0100000), 
                        ux::u7::new(0b0001100), ux::u7::new(0b0001001), // source muid
                        ux::u7::new(0x7F), ux::u7::new(0x7F),  // destination muid
                        ux::u7::new(0x7F),
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x7F), // destination muid
                        ux::u7::new(0b0000000), ux::u7::new(0b0010000), 
                        ux::u7::new(0b1001110), // device manufacturer
                        ux::u7::new(0b0011001), ux::u7::new(0b1110011), // device family
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b0010000), ux::u7::new(0b0110011), // device model number
                        ux::u7::new(0x1), ux::u7::new(0x6), // software version
                        ux::u7::new(0x5), ux::u7::new(0x31), 
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::End)
                    .data(&[ 
                        ux::u7::new(0b0000_0110), // support flags
                        ux::u7::new(0x0), ux::u7::new(0x0), 
                        ux::u7::new(0b1000000), ux::u7::new(0x0), // max sysex size
                    ])
                    .build(),
            ]),
            Ok(Message {
                group: ux::u4::new(0x8),
                source: ux::u28::new(0x123_1000),
                device_manufacturer: ux::u21::new(0x13_8800),
                device_family: ux::u14::new(0x3999),
                device_model_number: ux::u14::new(0x1990),
                software_version: [
                    ux::u7::new(0x1),
                    ux::u7::new(0x6),
                    ux::u7::new(0x5),
                    ux::u7::new(0x31),
                ],
                protocol_negotiation_supported: true,
                profile_configuration_supported: true,
                property_exchange_supported: false,
                max_sysex_message_size: ux::u28::new(0x10_0000),
            }),
        )
    }
}
