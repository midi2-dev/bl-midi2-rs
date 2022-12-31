use crate::{
    ci::{
        helpers as ci_helpers,
        CiMessageDetail,
        DeviceId,
    },
    error::Error,
    util::{builder, getter, sysex_message},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    device_id: DeviceId,
    source: ux::u28,
    destination: ux::u28,
}

builder::builder!(
    group: ux::u4,
    device_id: DeviceId,
    source: ux::u28,
    destination: ux::u28
);

impl Message {
    const STATUS: u8 = 0x7F;
    const DATA_SIZE: usize = 13;
    getter::getter!(group, ux::u4);
    getter::getter!(device_id, DeviceId);
    getter::getter!(source, ux::u28);
    getter::getter!(destination, ux::u28);
}

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        ci_helpers::write_ci_data(
            self.group,
            self.device_id,
            Message::STATUS,
            self.source,
            self.destination,
            &[],
            messages,
        )
    }
    fn from_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Self {
        let standard_data = ci_helpers::read_standard_data(messages);
        let messages = sysex_message::SysexMessages(messages);
        Message {
            group: messages.group(),
            device_id: standard_data.device_id,
            source: standard_data.source,
            destination: standard_data.destination,
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
        ci::{VERSION, CiMessage},
        message::system_exclusive_8bit as sysex8,
        message::system_exclusive_7bit as sysex7,
    };
    
    #[test]
    #[rustfmt::skip]
    fn try_to_sysex8() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x9),
                device_id: DeviceId::Channel(ux::u4::new(0xC)),
                source: ux::u28::new(126343486),
                destination: ux::u28::new(69631782),
            }.try_to_sysex8(
                &mut [
                    Default::default(),
                    Default::default(),
                ],
                0xF2
            ).unwrap(),
            &[
                sysex8::Message::builder()
                    .stream_id(0xF2)
                    .group(ux::u4::new(0x9))
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0xC, // Device ID
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x7F, // universal sysex sub-id 2: nak
                        VERSION,
                        0b00111110, 0b00110010, 0b00011111, 0b00111100, 
                        0b00100110, 0b01111110, 0b00011001, // destination muid
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0xF2)
                    .group(ux::u4::new(0x9))
                    .status(sysex8::Status::End)
                    .data(&[
                        0b00100001, // destination muid
                    ])
                    .build(),
            ]
        )
    }
    
    #[test]
    #[rustfmt::skip]
    fn try_from_sysex8() {
        assert_eq!(
            Message::try_from_sysex8(
                &[
                    sysex8::Message::builder()
                        .stream_id(0xF2)
                        .group(ux::u4::new(0x9))
                        .status(sysex8::Status::Begin)
                        .data(&[
                            0x7E, // universal sysex
                            0xC, // Device ID
                            0x0D, // universal sysex sub-id 1: midi ci
                            0x7F, // universal sysex sub-id 2: nak
                            VERSION,
                            0b00111110, 0b00110010, 0b00011111, 0b00111100, // source muid
                            0b00100110, 0b01111110, 0b00011001, // destination muid
                        ])
                        .build(),
                    sysex8::Message::builder()
                        .stream_id(0xF2)
                        .group(ux::u4::new(0x9))
                        .status(sysex8::Status::End)
                        .data(&[
                            0b00100001, // destination muid
                        ])
                        .build(),
                ],
            ),
            Ok(Message {
                group: ux::u4::new(0x9),
                device_id: DeviceId::Channel(ux::u4::new(0xC)),
                source: ux::u28::new(126343486),
                destination: ux::u28::new(69631782),
            })
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex7() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x9),
                device_id: DeviceId::Channel(ux::u4::new(0xC)),
                source: ux::u28::new(126343486),
                destination: ux::u28::new(69631782),
            }.try_to_sysex7(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
            ]).unwrap(),
            &[
                sysex7::Message::builder()
                    .group(ux::u4::new(0x9))
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0xC), // Device ID
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x7F), // universal sysex sub-id 2: nak
                        ux::u7::new(VERSION),
                        ux::u7::new(0b00111110), // source muid
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x9))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b00110010),
                        ux::u7::new(0b00011111), ux::u7::new(0b00111100), // source muid
                        ux::u7::new(0b00100110), ux::u7::new(0b01111110),
                        ux::u7::new(0b00011001), // destination muid
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x9))
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0b00100001), // destination muid
                    ])
                    .build(),
            ]
        )
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex7() {
        assert_eq!(
            Message::try_from_sysex7(
                &[
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x9))
                        .status(sysex7::Status::Begin)
                        .data(&[
                            ux::u7::new(0x7E), // universal sysex
                            ux::u7::new(0xC), // Device ID
                            ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                            ux::u7::new(0x7F), // universal sysex sub-id 2: nak
                            ux::u7::new(VERSION),
                            ux::u7::new(0b00111110), // source muid
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x9))
                        .status(sysex7::Status::Continue)
                        .data(&[
                            ux::u7::new(0b00110010),
                            ux::u7::new(0b00011111), ux::u7::new(0b00111100), // source muid
                            ux::u7::new(0b00100110), ux::u7::new(0b01111110),
                            ux::u7::new(0b00011001), // destination muid
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x9))
                        .status(sysex7::Status::End)
                        .data(&[
                            ux::u7::new(0b00100001), // destination muid
                        ])
                        .build(),
                ],
            ),
            Ok(Message {
                group: ux::u4::new(0x9),
                device_id: DeviceId::Channel(ux::u4::new(0xC)),
                source: ux::u28::new(126343486),
                destination: ux::u28::new(69631782),
            })
        );
    }
}