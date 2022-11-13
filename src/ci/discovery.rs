use crate::{
    ci::{helpers, CiMessageDetail, DeviceId},
    error::Error,
    util::{getter, sysex_message, BitOps, Truncate},
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

impl Message {
    const DATA_SIZE: usize = 29;
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
    fn to_sysex<'a, M: sysex_message::SysexMessage>(
        &self,
        messages: &'a mut [M],
    ) -> &'a mut [M] {
        let ret = helpers::write_ci_data(
            self.group,
            DeviceId::MidiPort,
            0x70,
            self.source,
            ux::u28::new(0xFFF_FFFF),
            &[
                self.device_manufacturer.truncate(),
                (self.device_manufacturer >> 7).truncate(),
                (self.device_manufacturer >> 14).truncate(),
                self.device_family.truncate(),
                (self.device_family >> 7).truncate(),
                self.device_model_number.truncate(),
                (self.device_model_number >> 7).truncate(),
                self.software_version[0],
                self.software_version[1],
                self.software_version[2],
                self.software_version[3],
                {
                    let flags = *0x0_u8
                        .set_bit(6, self.protocol_negotiation_supported)
                        .set_bit(5, self.profile_configuration_supported)
                        .set_bit(4, self.property_exchange_supported);
                    ux::u7::new(flags)
                },
                self.max_sysex_message_size.truncate(),
                (self.max_sysex_message_size >> 7).truncate(),
                (self.max_sysex_message_size >> 14).truncate(),
                (self.max_sysex_message_size >> 21).truncate(),
            ],
            messages,
        );
        ret
    }

    fn from_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Self {
        let standard_data = helpers::read_standard_data(messages);
        let messages = sysex_message::SysexMessages(messages);
        let support_flags = messages.datum(24);
        Message {
            group: messages.group(),
            source: standard_data.source,
            device_manufacturer: ux::u21::from(messages.datum(13) & 0b0111_1111)
                | ux::u21::from(messages.datum(14) & 0b0111_1111) << 7
                | ux::u21::from(messages.datum(15) & 0b0111_1111) << 14,
            device_family: ux::u14::from(messages.datum(16) & 0b0111_1111)
                | ux::u14::from(messages.datum(17) & 0b0111_1111) << 7,
            device_model_number: ux::u14::from(messages.datum(18) & 0b0111_1111)
                | ux::u14::from(messages.datum(19) & 0b0111_1111) << 7,
            software_version: [
                messages.datum(20).truncate(),
                messages.datum(21).truncate(),
                messages.datum(22).truncate(),
                messages.datum(23).truncate(),
            ],
            protocol_negotiation_supported: support_flags.bit(6),
            profile_configuration_supported: support_flags.bit(5),
            property_exchange_supported: support_flags.bit(4),
            max_sysex_message_size: ux::u28::from(messages.datum(25) & 0b0111_1111)
                | ux::u28::from(messages.datum(26) & 0b0111_1111) << 7
                | ux::u28::from(messages.datum(27) & 0b0111_1111) << 14
                | ux::u28::from(messages.datum(28) & 0b0111_1111) << 21,
        }
    }
    fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        helpers::validate_sysex(messages)?;
        let messages = sysex_message::SysexMessages(messages);
        if messages.len() != Message::DATA_SIZE || messages.datum(1) != 0x7F || messages.datum(3) != 0x70 {
            Err(Error::InvalidData)
        } else {
            Ok(())
        }
    }
    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        let messages = sysex_message::SysexMessages(messages);
        if messages.max_len() < Message::DATA_SIZE {
            Err(Error::BufferOverflow)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::{VERSION, CiMessage},
        message::system_exclusive_8bit as sysex8,
        message::system_exclusive_7bit as sysex7,
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
