use crate::{
    error::Error,
    ci::{helpers as ci_helpers, CiMessageDetail, DeviceId},
    util::{builder, getter, sysex_message, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
}

impl Message {
    const STATUS: u8 = 0x15;
    const DATA_SIZE: usize = 14;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
    getter::getter!(destination, ux::u28);
    getter::getter!(authority_level, ux::u7);
}

builder::builder!(
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7
);

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        ci_helpers::write_ci_data(
            self.group,
            DeviceId::MidiPort,
            Message::STATUS,
            self.source,
            self.destination,
            &[self.authority_level],
            messages,
        )
    }

    fn from_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Self {
        let standard_data = ci_helpers::read_standard_data(messages);
        let messages = sysex_message::SysexMessages::new(messages);
        Message {
            group: messages.group(),
            source: standard_data.source,
            destination: standard_data.destination,
            authority_level: messages.datum(13).truncate(),
        }
    }

    fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        ci_helpers::validate_sysex(messages, Message::STATUS)?;
        ci_helpers::validate_buffer_size(messages, Message::DATA_SIZE)
    }

    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(&self, messages: &[M]) -> Result<(), Error> {
        ci_helpers::validate_buffer_size(messages, Message::DATA_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::{
            self,
            CiMessage,
        },
        message::system_exclusive_8bit as sysex8,
        message::system_exclusive_7bit as sysex7,
    };
    
    const TEST_GROUP: ux::u4 = ux::u4::new(0xF);
    const TEST_SOURCE: ux::u28 = ux::u28::new(11629215);
    const TEST_DESTINATION: ux::u28 = ux::u28::new(225493351);
    const TEST_AUTHORITY_LEVEL: ux::u7 = ux::u7::new(0x27);
    const TEST_STREAM_ID: u8 = 0x76;
    
    #[test]
    #[rustfmt::skip]
    fn try_to_sysex8() {
        assert_eq!(
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
                .try_to_sysex8(&mut [
                    Default::default(),
                    Default::default(),
                ], TEST_STREAM_ID)
                .unwrap(),
            &[
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x15, // universal sysex sub-id 2: set protocol
                        ci::VERSION,
                        0b00011111, 0b01100101, 0b01000101, 0b00000101, // source
                        0b01100111, 0b00000010, 0b01000011, // destination
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::End)
                    .data(&[
                        0b01101011, // destination
                        0x27, // authority level
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
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x15, // universal sysex sub-id 2: set protocol
                        ci::VERSION,
                        0b00011111, 0b01100101, 0b01000101, 0b00000101, // source
                        0b01100111, 0b00000010, 0b01000011, // destination
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(TEST_GROUP)
                    .stream_id(TEST_STREAM_ID)
                    .status(sysex8::Status::End)
                    .data(&[
                        0b01101011, // destination
                        0x27, // authority level
                    ])
                    .build(),
            ]).unwrap(),
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex7() {
        assert_eq!(
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
                .try_to_sysex7(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ]).unwrap(),
            &[
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID: whole midi port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x15), // universal sysex sub-id 2: set protocol
                        ux::u7::new(ci::VERSION),
                        ux::u7::new(0b00011111), // source
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01100101), ux::u7::new(0b01000101),
                        ux::u7::new(0b00000101), // source
                        ux::u7::new(0b01100111), ux::u7::new(0b00000010),
                        ux::u7::new(0b01000011), // destination
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0b01101011), // destination
                        ux::u7::new(0x27), // authority level
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
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID: whole midi port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x15), // universal sysex sub-id 2: set protocol
                        ux::u7::new(ci::VERSION),
                        ux::u7::new(0b00011111), // source
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01100101), ux::u7::new(0b01000101),
                        ux::u7::new(0b00000101), // source
                        ux::u7::new(0b01100111), ux::u7::new(0b00000010),
                        ux::u7::new(0b01000011), // destination
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(TEST_GROUP)
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0b01101011), // destination
                        ux::u7::new(0x27), // authority level
                    ])
                    .build(),
            ]).unwrap(),
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
        );
    }
}