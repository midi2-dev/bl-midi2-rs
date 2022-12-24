use crate::{
    ci::{
        helpers as ci_helpers,
        CiMessageDetail,
        DeviceId,
    },
    error::Error,
    util::{
        builder, 
        getter,
        sysex_message::{self, SysexMessage},
        Encode7Bit,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    source: ux::u28,
    target: ux::u28,
}

builder::builder!(
    group: ux::u4,
    source: ux::u28,
    target: ux::u28
);

impl Message {
    const STATUS: u8 = 0x7E;
    const DATA_SIZE: usize = 17;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
    getter::getter!(target, ux::u28);
}

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        ci_helpers::write_ci_data(
            self.group,
            DeviceId::MidiPort,
            0x7E,
            self.source,
            ux::u28::new(0xFFF_FFFF),
            &self.target.to_u7s(),
            messages,
        )
    }
    fn from_sysex<M: SysexMessage>(messages: &[M]) -> Self {
        let standard_data = ci_helpers::read_standard_data(messages);
        let messages = sysex_message::SysexMessages(messages);
        Message {
            group: messages.group(),
            source: standard_data.source,
            target: ux::u28::from_u7s(&[
                messages.datum(13),
                messages.datum(14),
                messages.datum(15),
                messages.datum(16),
            ]),
        }
    }
    fn validate_sysex<M: SysexMessage>(messages: &[M]) -> Result<(), Error> {
        ci_helpers::validate_sysex(messages, Message::STATUS)?;
        ci_helpers::validate_buffer_size(messages, Message::DATA_SIZE)
    }
    fn validate_to_sysex_buffer<M: SysexMessage>(messages: &[M]) -> Result<(), Error> {
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
                group: ux::u4::new(0x5),
                source: ux::u28::new(0xFABF0D5),
                target: ux::u28::new(0xBA55B05),
            }.try_to_sysex8(&mut [
                    Default::default(),
                    Default::default(),
                ], 
                0x33,
            ).unwrap(),
            &[
                sysex8::Message::builder()
                    .stream_id(0x33)
                    .group(ux::u4::new(0x5))
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x7E, // universal sysex sub-id 2: invalidate muid
                        VERSION,
                        0b01010101, 0b01100001, 0b00101111, 0b01111101, // source muid
                        0x7F, 0x7F, 0x7F,  // destination muid
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x33)
                    .group(ux::u4::new(0x5))
                    .status(sysex8::Status::End)
                    .data(&[
                        0x7F, // destination muid
                        0b0000101, 0b0110110, 0b0010101, 0b1011101, // target muid
                    ])
                    .build(),
            ],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex8() {
        assert_eq!(
            Message::try_from_sysex8(
                &[
                    sysex8::Message::builder()
                        .stream_id(0x33)
                        .group(ux::u4::new(0x5))
                        .status(sysex8::Status::Begin)
                        .data(&[
                            0x7E, // universal sysex
                            0x7F, // Device ID
                            0x0D, // universal sysex sub-id 1: midi ci
                            0x7E, // universal sysex sub-id 2: invalidate muid
                            VERSION,
                            0b01010101, 0b01100001, 0b00101111, 0b01111101, // source muid
                            0x7F, 0x7F, 0x7F,  // destination muid
                        ])
                        .build(),
                    sysex8::Message::builder()
                        .stream_id(0x33)
                        .group(ux::u4::new(0x5))
                        .status(sysex8::Status::End)
                        .data(&[
                            0x7F, // destination muid
                            0b0000101, 0b0110110, 0b0010101, 0b1011101, // target muid
                        ])
                        .build(),
                ],
            ),
            Ok(Message {
                group: ux::u4::new(0x5),
                source: ux::u28::new(0xFABF0D5),
                target: ux::u28::new(0xBA55B05),
            })
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex7() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x5),
                source: ux::u28::new(0xFABF0D5),
                target: ux::u28::new(0xBA55B05),
            }.try_to_sysex7(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
            ]).unwrap(),
            &[
                sysex7::Message::builder()
                    .group(ux::u4::new(0x5))
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x7E), // universal sysex sub-id 2: invalidate muid
                        ux::u7::new(VERSION),
                        ux::u7::new(0b01010101), // source muid
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x5))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01100001),
                        ux::u7::new(0b00101111), ux::u7::new(0b01111101), // source muid
                        ux::u7::new(0x7F), ux::u7::new(0x7F), ux::u7::new(0x7F),  // destination muid
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x5))
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0x7F), // destination muid
                        ux::u7::new(0b0000101), ux::u7::new(0b0110110),
                        ux::u7::new(0b0010101), ux::u7::new(0b1011101), // target muid
                    ])
                    .build(),
            ],
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_sysex7() {
        assert_eq!(
            Message::try_from_sysex7(
                &[
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x5))
                        .status(sysex7::Status::Begin)
                        .data(&[
                            ux::u7::new(0x7E), // universal sysex
                            ux::u7::new(0x7F), // Device ID
                            ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                            ux::u7::new(0x7E), // universal sysex sub-id 2: invalidate muid
                            ux::u7::new(VERSION),
                            ux::u7::new(0b01010101), // source muid
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x5))
                        .status(sysex7::Status::Continue)
                        .data(&[
                            ux::u7::new(0b01100001),
                            ux::u7::new(0b00101111), ux::u7::new(0b01111101), // source muid
                            ux::u7::new(0x7F), ux::u7::new(0x7F), ux::u7::new(0x7F),  // destination muid
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x5))
                        .status(sysex7::Status::End)
                        .data(&[
                            ux::u7::new(0x7F), // destination muid
                            ux::u7::new(0b0000101), ux::u7::new(0b0110110),
                            ux::u7::new(0b0010101), ux::u7::new(0b1011101), // target muid
                        ])
                        .build(),
                ],
            ),
            Ok(Message {
                group: ux::u4::new(0x5),
                source: ux::u28::new(0xFABF0D5),
                target: ux::u28::new(0xBA55B05),
            })
        );
    }
}