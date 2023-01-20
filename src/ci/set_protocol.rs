use crate::{
    util::{builder, getter, sysex_message, Truncate},
    ci::{helpers as ci_helpers, protocol::*, CiMessageDetail, DeviceId},
    error::Error,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
    protocol: Protocol,
}

impl Message {
    const STATUS: u8 = 0x12;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
    getter::getter!(destination, ux::u28);
    getter::getter!(authority_level, ux::u7);
    getter::getter!(protocol, Protocol); // todo get by ref
}

builder::builder!(
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
    protocol: Protocol
);

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        let mut protocol_data_buffer = [ux::u7::default(); 5];
        ci_helpers::write_ci_data(
            self.group,
            DeviceId::MidiPort,
            Message::STATUS,
            self.source,
            self.destination,
            &[
                &[self.authority_level],
                ci_helpers::protocol_data(&self.protocol, &mut protocol_data_buffer),
            ]
            .concat(),
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
            protocol: ci_helpers::read_protocol(&messages, 14),
        }
    }

    fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        ci_helpers::validate_sysex(messages, Message::STATUS)?;
        ci_helpers::validate_buffer_size(messages, 19)?;
        let messages = sysex_message::SysexMessages::new(messages);
        ci_helpers::validate_protocol_data(&[
            messages.datum(14),
            messages.datum(15),
            messages.datum(16),
            messages.datum(17),
            messages.datum(18),
        ])
    }

    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(&self, messages: &[M]) -> Result<(), Error> {
        ci_helpers::validate_buffer_size(messages, 19)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::{
            protocol::Protocol,
            CiMessage,
            self,
        },
        util::Truncate,
        message::system_exclusive_8bit as sysex8,
        message::system_exclusive_7bit as sysex7,
    };
    
    #[test]
    #[rustfmt::skip]
    fn try_to_sysex8() {
        assert_eq!(
            Message::builder()
                .group(ux::u4::new(0x8))
                .source(ux::u28::new(229884696))
                .destination(ux::u28::new(36457347))
                .authority_level(ux::u7::new(0x53))
                .protocol(Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: Protocol::MIDI_2_VERSION
                })
                .build()
                .try_to_sysex8(&mut [
                    Default::default(),
                    Default::default(),
                ], 0x29)
                .unwrap(),
            &[
                sysex8::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .stream_id(0x29)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x12, // universal sysex sub-id 2: set protocol
                        ci::VERSION,
                        0b00011000, 0b00000110, 0b01001111, 0b01101101, // source
                        0b00000011, 0b00010111, 0b00110001, // destination muid
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .stream_id(0x29)
                    .status(sysex8::Status::End)
                    .data(&[
                        0b00010001, // destination muid
                        0x53, // authority level
                        // protocol
                        0x02, // midi2
                        Protocol::MIDI_2_VERSION.into(),
                        0x00000001, // flags
                        0x0, 0x0 // protected
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
                    .group(ux::u4::new(0x8))
                    .stream_id(0x29)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x12, // universal sysex sub-id 2: set protocol
                        ci::VERSION,
                        0b00011000, 0b00000110, 0b01001111, 0b01101101, // source
                        0b00000011, 0b00010111, 0b00110001, // destination muid
                    ])
                    .build(),
                sysex8::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .stream_id(0x29)
                    .status(sysex8::Status::End)
                    .data(&[
                        0b00010001, // destination muid
                        0x53, // authority level
                        // protocol
                        0x02, // midi2
                        Protocol::MIDI_2_VERSION.into(),
                        0x00000001, // flags
                        0x0, 0x0 // protected
                    ])
                    .build(),
            ]),
            Ok(Message::builder()
                .group(ux::u4::new(0x8))
                .source(ux::u28::new(229884696))
                .destination(ux::u28::new(36457347))
                .authority_level(ux::u7::new(0x53))
                .protocol(Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: Protocol::MIDI_2_VERSION
                })
               .build()
            ),
        )
    }

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex7() {
        assert_eq!(
            Message::builder()
                .group(ux::u4::new(0x8))
                .source(ux::u28::new(229884696))
                .destination(ux::u28::new(36457347))
                .authority_level(ux::u7::new(0x53))
                .protocol(Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: Protocol::MIDI_2_VERSION
                })
                .build()
                .try_to_sysex7(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ])
                .unwrap(),
            &[
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID: whole midi port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x12), // universal sysex sub-id 2: set protocol
                        ci::VERSION.truncate(),
                        ux::u7::new(0b00011000), // source
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b00000110), ux::u7::new(0b01001111), ux::u7::new(0b01101101), // source
                        ux::u7::new(0b00000011), ux::u7::new(0b00010111), ux::u7::new(0b00110001), // destination muid
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b00010001), // destination muid
                        ux::u7::new(0x53), // authority level
                        // protocol
                        ux::u7::new(0x02), // midi2
                        Protocol::MIDI_2_VERSION, // version
                        ux::u7::new(0x00000001), // flags
                        ux::u7::new(0x0), // protected
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0x0) // protected
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
                        ux::u7::new(0x7F), // Device ID: whole midi port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x12), // universal sysex sub-id 2: set protocol
                        ci::VERSION.truncate(),
                        ux::u7::new(0b00011000), // source
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b00000110), ux::u7::new(0b01001111), ux::u7::new(0b01101101), // source
                        ux::u7::new(0b00000011), ux::u7::new(0b00010111), ux::u7::new(0b00110001), // destination muid
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b00010001), // destination muid
                        ux::u7::new(0x53), // authority level
                        // protocol
                        ux::u7::new(0x02), // midi2
                        Protocol::MIDI_2_VERSION, // version
                        ux::u7::new(0x00000001), // flags
                        ux::u7::new(0x0), // protected
                    ])
                    .build(),
                sysex7::Message::builder()        
                    .group(ux::u4::new(0x8))
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0x0) // protected
                    ])
                    .build(),
            ]),
            Ok(Message::builder()
                .group(ux::u4::new(0x8))
                .source(ux::u28::new(229884696))
                .destination(ux::u28::new(36457347))
                .authority_level(ux::u7::new(0x53))
                .protocol(Protocol::Midi2 {
                    jitter_reduction_extension: true,
                    version: Protocol::MIDI_2_VERSION
                })
                .build()
            ),
        )
    }
}