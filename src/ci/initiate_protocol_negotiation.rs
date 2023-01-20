macro_rules! initiate_protocol_negotiation_message {
    ($op_code:expr) => {
        use crate::{
            ci::{helpers as ci_helpers, protocol::*, CiMessageDetail, DeviceId},
            error::Error,
            util::{builder, getter, sysex_message, SliceData, Truncate},
        };

        type Protocols = SliceData<Option<Protocol>, 2>;
        
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            source: ux::u28,
            destination: ux::u28,
            authority_level: ux::u7,
            protocols: Protocols,
        }

        impl Message {
            const STATUS: u8 = 0x10;

            getter::getter!(group, ux::u4);
            getter::getter!(source, ux::u28);
            getter::getter!(destination, ux::u28);
            getter::getter!(authority_level, ux::u7);

            pub fn protocol(&self, index: usize) -> Option<&Protocol> {
                match index {
                    0 | 1 => self.protocols[index].as_ref(),
                    _ => None,
                }
            }

            pub fn preferred_protocol(&self) -> &Protocol {
                self.protocols[0].as_ref().unwrap()
            }

            fn protocol_data<'a>(&self, buff: &'a mut [ux::u7]) -> &'a [ux::u7] {
                for (i, protocol) in self.protocols.iter()
                        .filter_map(|v| v.as_ref())
                        .enumerate() {
                    ci_helpers::protocol_data(protocol, &mut buff[5*i..5*i + 5]);
                }
                &buff[0..5 * self.protocols.len()]
            }

            pub fn builder() -> Builder {
                Builder {
                    group: None,
                    source: None,
                    destination: None,
                    authority_level: None,
                    protocols: Default::default(),
                }
            }
        }

        pub struct Builder {
            group: Option<ux::u4>,
            source: Option<ux::u28>,
            destination: Option<ux::u28>,
            authority_level: Option<ux::u7>,
            protocols: Protocols,
        }

        impl Builder {
            builder::builder_setter!(group: ux::u4);
            builder::builder_setter!(source: ux::u28);
            builder::builder_setter!(destination: ux::u28);
            builder::builder_setter!(authority_level: ux::u7);

            /// Append an additional protocol to the list of
            /// supported protocols.
            ///
            /// **Warning**: only two supported protocols can be added.
            /// This function will panic if called more than
            /// twice on the same builder.
            pub fn protocol(&mut self, p: Protocol) -> &mut Self {
                self.protocols.push(Some(p));
                self
            }

            pub fn build(&self) -> Message {
                Message {
                    group: self.group.unwrap_or_else(|| panic!("Missing fields")),
                    source: self.source.unwrap_or_else(|| panic!("Missing fields")),
                    destination: self.destination.unwrap_or_else(|| panic!("Missing fields")),
                    authority_level: self
                        .authority_level
                        .unwrap_or_else(|| panic!("Missing fields")),
                    protocols: {
                        if self.protocols.is_empty() {
                            panic!("Missing fields");
                        }
                        self.protocols.clone()
                    },
                }
            }
        }

        impl CiMessageDetail for Message {
            fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
                let mut protocol_data_buffer = [ux::u7::default(); 5 * Protocols::LEN];
                ci_helpers::write_ci_data(
                    self.group,
                    DeviceId::MidiPort,
                    Message::STATUS,
                    self.source,
                    self.destination,
                    &[
                        &[
                            self.authority_level,
                            match self.protocols.len() {
                                1 => ux::u7::new(0x1),
                                2 => ux::u7::new(0x2),
                                _ => unreachable!(),
                            },
                        ],
                        self.protocol_data(&mut protocol_data_buffer),
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
                    protocols: read_protocols(&messages),
                }
            }

            fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
                let messages_wrapper = sysex_message::SysexMessages::new(messages);

                if messages_wrapper.len() < 15 {
                    return Err(Error::InvalidData);
                }
                let protocol_count = messages_wrapper.datum(14) as usize;

                // do we need to support more than two
                // protocols at this point?
                let protocol_count_supported = [1_usize, 2_usize].iter().any(|&v| v == protocol_count);
                if !protocol_count_supported {
                    // todo
                    // maybe better not to fail at this point
                    // could just pick the first two supported protocols?
                    return Err(Error::InvalidData);
                }

                ci_helpers::validate_buffer_size(messages, 15 + 5 * protocol_count)?;

                for i in 0..protocol_count {
                    ci_helpers::validate_protocol_data(&[
                        messages_wrapper.datum(15 + i * 5),
                        messages_wrapper.datum(16 + i * 5),
                        messages_wrapper.datum(17 + i * 5),
                        messages_wrapper.datum(18 + i * 5),
                        messages_wrapper.datum(19 + i * 5),
                    ])?;
                }

                Ok(())
            }

            fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(
                &self,
                messages: &[M],
            ) -> Result<(), Error> {
                ci_helpers::validate_buffer_size(messages, 15 + 5 * self.protocols.len())
            }
        }

        fn read_protocols<M>(messages: &sysex_message::SysexMessages<M>) -> Protocols
        where
            M: sysex_message::SysexMessage,
        {
            let mut protocols = Protocols::default();
            for i in 0..(messages.datum(14) as usize) {
                protocols.push(Some(ci_helpers::read_protocol(messages, 15 + i*5)));
            }
            protocols
        }
    }
}

pub mod query {
    initiate_protocol_negotiation_message!(0x10);
}

pub mod reply {
    initiate_protocol_negotiation_message!(0x11);
}

#[cfg(test)]
mod tests {
    use crate::{
        ci::{CiMessage, VERSION},
        message::system_exclusive_7bit as sysex7,
        message::system_exclusive_8bit as sysex8,
    };
    
    use super::query::*;
    use crate::ci::protocol::*;

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex8() {
        assert_eq!(
            Message::builder()
                .group(ux::u4::new(0x3))
                .source(ux::u28::new(14973326))
                .destination(ux::u28::new(89757246))
                .authority_level(ux::u7::new(0x2D))
                .protocol(Protocol::Midi1 {
                        size_of_packet_extension: true,
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_1_VERSION,
                })
                .protocol(Protocol::Midi2 {
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_2_VERSION,
                })
                .build()
                .try_to_sysex8(
                    &mut [
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    ],
                    0x5C,
                ).unwrap(),
            &[
                sysex8::Message::builder()
                    .stream_id(0x5C)
                    .group(ux::u4::new(0x3))
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: to MIDI Port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x10, // universal sysex sub-id 2: init protocol negotiation
                        VERSION,
                        0b00001110, 0b01110011, 0b00010001, 0b00000111, // source
                        0b00111110, 0b00101100, 0b01100110, // destination
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x5C)
                    .group(ux::u4::new(0x3))
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0b00101010, // destination muid
                        0x2D, // authority level
                        2, // number of supported protocols
                        // first protocol
                        0x01, // midi1
                        Protocol::MIDI_1_VERSION.into(),
                        0b0000_0010, // extension bit mask
                        0x0, 0x0, // reserved
                        // second protocol
                        0x02, // midi1
                        Protocol::MIDI_2_VERSION.into(),
                        0b0000_0000, // extension bit mask
                        0x0, // reserved
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(0x5C)
                    .group(ux::u4::new(0x3))
                    .status(sysex8::Status::End)
                    .data(&[
                        0x0, // reserved
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
                        .stream_id(0x5C)
                        .group(ux::u4::new(0x3))
                        .status(sysex8::Status::Begin)
                        .data(&[
                            0x7E, // universal sysex
                            0x7F, // Device ID: to MIDI Port
                            0x0D, // universal sysex sub-id 1: midi ci
                            0x10, // universal sysex sub-id 2: init protocol negotiation
                            VERSION,
                            0b00001110, 0b01110011, 0b00010001, 0b00000111, // source
                            0b00111110, 0b00101100, 0b01100110, // destination
                        ])
                        .build(),
                    sysex8::Message::builder()
                        .stream_id(0x5C)
                        .group(ux::u4::new(0x3))
                        .status(sysex8::Status::Continue)
                        .data(&[
                            0b00101010, // destination muid
                            0x2D, // authority level
                            2, // number of supported protocols
                            // first protocol
                            0x01, // midi1
                            Protocol::MIDI_1_VERSION.into(),
                            0b0000_0010, // extension bit mask
                            0x0, 0x0, // reserved
                            // second protocol
                            0x02, // midi1
                            Protocol::MIDI_2_VERSION.into(),
                            0b0000_0000, // extension bit mask
                            0x0, // reserved
                        ])
                        .build(),
                    sysex8::Message::builder()
                        .stream_id(0x5C)
                        .group(ux::u4::new(0x3))
                        .status(sysex8::Status::End)
                        .data(&[
                            0x0, // reserved
                        ])
                        .build(),
                ],
            ),
            Ok(Message::builder()
                .group(ux::u4::new(0x3))
                .source(ux::u28::new(14973326))
                .destination(ux::u28::new(89757246))
                .authority_level(ux::u7::new(0x2D))
                .protocol(Protocol::Midi1 {
                        size_of_packet_extension: true,
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_1_VERSION,
                })
                .protocol(Protocol::Midi2 {
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_2_VERSION,
                })
                .build()
            ),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn try_to_sysex7() {
        assert_eq!(
            Message::builder()
                .group(ux::u4::new(0x3))
                .source(ux::u28::new(14973326))
                .destination(ux::u28::new(89757246))
                .authority_level(ux::u7::new(0x2D))
                .protocol(Protocol::Midi1 {
                        size_of_packet_extension: true,
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_1_VERSION,
                })
                .protocol(Protocol::Midi2 {
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_2_VERSION,
                })
                .build()
                .try_to_sysex7(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ]).unwrap(),
            &[
                sysex7::Message::builder()
                    .group(ux::u4::new(0x3))
                    .status(sysex7::Status::Begin)
                    .data(&[
                        ux::u7::new(0x7E), // universal sysex
                        ux::u7::new(0x7F), // Device ID: to MIDI Port
                        ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                        ux::u7::new(0x10), // universal sysex sub-id 2: init protocol negotiation
                        ux::u7::new(VERSION),
                        ux::u7::new(0b00001110), // source
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x3))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b01110011), ux::u7::new(0b00010001), ux::u7::new(0b00000111), // source
                        ux::u7::new(0b00111110), ux::u7::new(0b00101100), ux::u7::new(0b01100110), // destination
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x3))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0b00101010), // destination muid
                        ux::u7::new(0x2D), // authority level
                        ux::u7::new(2), // number of supported protocols
                        // first protocol
                        ux::u7::new(0x01), // midi1
                        Protocol::MIDI_1_VERSION,
                        ux::u7::new(0b0000_0010), // extension bit mask
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x3))
                    .status(sysex7::Status::Continue)
                    .data(&[
                        ux::u7::new(0x0), ux::u7::new(0x0), // reserved
                        // second protocol
                        ux::u7::new(0x02), // midi1
                        Protocol::MIDI_2_VERSION,
                        ux::u7::new(0b0000_0000), // extension bit mask
                        ux::u7::new(0x0), // reserved
                    ])
                    .build(),
                sysex7::Message::builder()
                    .group(ux::u4::new(0x3))
                    .status(sysex7::Status::End)
                    .data(&[
                        ux::u7::new(0x0), // reserved
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
                        .group(ux::u4::new(0x3))
                        .status(sysex7::Status::Begin)
                        .data(&[
                            ux::u7::new(0x7E), // universal sysex
                            ux::u7::new(0x7F), // Device ID: to MIDI Port
                            ux::u7::new(0x0D), // universal sysex sub-id 1: midi ci
                            ux::u7::new(0x10), // universal sysex sub-id 2: init protocol negotiation
                            ux::u7::new(VERSION),
                            ux::u7::new(0b00001110), // source
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x3))
                        .status(sysex7::Status::Continue)
                        .data(&[
                            ux::u7::new(0b01110011), ux::u7::new(0b00010001), ux::u7::new(0b00000111), // source
                            ux::u7::new(0b00111110), ux::u7::new(0b00101100), ux::u7::new(0b01100110), // destination
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x3))
                        .status(sysex7::Status::Continue)
                        .data(&[
                            ux::u7::new(0b00101010), // destination muid
                            ux::u7::new(0x2D), // authority level
                            ux::u7::new(2), // number of supported protocols
                            // first protocol
                            ux::u7::new(0x01), // midi1
                            Protocol::MIDI_1_VERSION,
                            ux::u7::new(0b0000_0010), // extension bit mask
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x3))
                        .status(sysex7::Status::Continue)
                        .data(&[
                            ux::u7::new(0x0), ux::u7::new(0x0), // reserved
                            // second protocol
                            ux::u7::new(0x02), // midi1
                            Protocol::MIDI_2_VERSION,
                            ux::u7::new(0b0000_0000), // extension bit mask
                            ux::u7::new(0x0), // reserved
                        ])
                        .build(),
                    sysex7::Message::builder()
                        .group(ux::u4::new(0x3))
                        .status(sysex7::Status::End)
                        .data(&[
                            ux::u7::new(0x0), // reserved
                        ])
                        .build(),
                ],
            ),
            Ok(Message::builder()
                .group(ux::u4::new(0x3))
                .source(ux::u28::new(14973326))
                .destination(ux::u28::new(89757246))
                .authority_level(ux::u7::new(0x2D))
                .protocol(Protocol::Midi1 {
                        size_of_packet_extension: true,
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_1_VERSION,
                })
                .protocol(Protocol::Midi2 {
                        jitter_reduction_extension: false,
                        version: Protocol::MIDI_2_VERSION,
                })
                .build()
            ),
        );
    }
}