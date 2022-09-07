use super::*;
use crate::extended_system_exclusive as ext_sysex;

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Message {
    Discovery {
        source: muid::Muid,
        device_manufacturer: [u8; 3],
        device_family: [u8; 2],
        device_model_number: [u8; 2],
        software_version: [u8; 4],
        protocol_negotiation_supported: bool,
        profile_configuration_supported: bool,
        property_exchange_supported: bool,
        max_sysex_message_size: [u8; 4],
    },
    DiscoveryReply {
        source: muid::Muid,
        destination: muid::Muid,
        device_manufacturer: [u8; 3],
        device_family: [u8; 2],
        device_model_number: [u8; 2],
        software_version: [u8; 4],
        protocol_negotiation_supported: bool,
        profile_configuration_supported: bool,
        property_exchange_supported: bool,
        max_sysex_message_size: [u8; 4],
    },
    InvalidateMuid {
        source: muid::Muid,
        target: muid::Muid,
    },
    Nak {
        device_id: DeviceId,
        source: muid::Muid,
        destination: muid::Muid,
    },
    InitiateProtocolNegotiation {
        source: muid::Muid,
        destination: muid::Muid,
        authority_level: u8,
        preferred_protocol: Protocol,
        additional_supported_protocols: Vec<Protocol>,
    },
    InitiateProtocolNegotiationReply {
        source: muid::Muid,
        destination: muid::Muid,
        authority_level: u8,
        preferred_protocol: Protocol,
        additional_supported_protocols: Vec<Protocol>,
    },
    SetNewProtocol {
        source: muid::Muid,
        destination: muid::Muid,
        authority_level: u8,
        protocol: Protocol,
    },
    TestNewProtocolInitiatorToResponder {
        source: muid::Muid,
        destination: muid::Muid,
        authority_level: u8,
    },
    TestNewProtocolResponderToInitiator {
        source: muid::Muid,
        destination: muid::Muid,
        authority_level: u8,
    },
    ConfirmNewProtocolEstablished {
        source: muid::Muid,
        destination: muid::Muid,
        authority_level: u8,
    },
    ProfileInquiry,
    ProfileInquiryReply,
    SetProfileOn,
    SetProfileOff,
    ProfileEnabledReport,
    ProfileDisabledReport,
    ProfileSpeificData,
    PropertyExchangeInquiry,
    PropertyExchangeInquiryReply,
    PropertyHasData,
    PropertyHasDataReply,
    PropertyGetData,
    PropertyGetDataReply,
    PropertySetData,
    PropertySetDataReply,
    PropertySubscription,
    PropertySubscriptionReply,
    PropertyNotify,
}

impl Message {
    const VERSION: u8 = 0x01;
}

impl std::convert::From<(Message, u8)> for ext_sysex::MessageGroup {
    fn from((m, stream_id): (Message, u8)) -> Self {
        match m {
            Message::Discovery {
                source,
                device_manufacturer,
                device_family,
                device_model_number,
                software_version,
                protocol_negotiation_supported,
                profile_configuration_supported,
                property_exchange_supported,
                max_sysex_message_size,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x70,
                    source.value().clone(),
                    [0x7F, 0x7F, 0x7F, 0x7F],
                    vec![
                        device_manufacturer[0],
                        device_manufacturer[1],
                        device_manufacturer[2],
                        device_family[0],
                        device_family[1],
                        device_model_number[0],
                        device_model_number[1],
                        software_version[0],
                        software_version[1],
                        software_version[2],
                        software_version[3],
                        bitmap(vec![
                            (0x1, protocol_negotiation_supported),
                            (0x2, profile_configuration_supported),
                            (0x3, property_exchange_supported),
                        ]),
                        max_sysex_message_size[0],
                        max_sysex_message_size[1],
                        max_sysex_message_size[2],
                        max_sysex_message_size[3],
                    ],
                ),
                stream_id,
            ),
            Message::DiscoveryReply {
                source,
                destination,
                device_manufacturer,
                device_family,
                device_model_number,
                software_version,
                protocol_negotiation_supported,
                profile_configuration_supported,
                property_exchange_supported,
                max_sysex_message_size,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x70,
                    source.value().clone(),
                    destination.value().clone(),
                    vec![
                        device_manufacturer[0],
                        device_manufacturer[1],
                        device_manufacturer[2],
                        device_family[0],
                        device_family[1],
                        device_model_number[0],
                        device_model_number[1],
                        software_version[0],
                        software_version[1],
                        software_version[2],
                        software_version[3],
                        bitmap(vec![
                            (0x1, protocol_negotiation_supported),
                            (0x2, profile_configuration_supported),
                            (0x3, property_exchange_supported),
                        ]),
                        max_sysex_message_size[0],
                        max_sysex_message_size[1],
                        max_sysex_message_size[2],
                        max_sysex_message_size[3],
                    ],
                ),
                stream_id,
            ),
            Message::InvalidateMuid{
                source,
                target,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x7E,
                    source.value().clone(),
                    [0x7F, 0x7F, 0x7F, 0x7F],
                    vec![
                        target[muid::Index::Byte1],
                        target[muid::Index::Byte2],
                        target[muid::Index::Byte3],
                        target[muid::Index::Byte4],
                    ],
                ),
                stream_id,
            ),
            Message::Nak {
                source,
                destination,
                device_id,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    device_id,
                    0x7F,
                    source.value().clone(),
                    destination.value().clone(),
                    Vec::new(),
                ),
                stream_id,
            ), 
            Message::InitiateProtocolNegotiation {
                source,
                destination,
                authority_level,
                preferred_protocol,
                additional_supported_protocols,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x0A,
                    source.value().clone(),
                    destination.value().clone(),
                    protocol_negotiation_payload(
                        authority_level,
                        preferred_protocol,
                        additional_supported_protocols,
                    ),
                ),
                stream_id,
            ), 
            Message::InitiateProtocolNegotiationReply {
                source,
                destination,
                authority_level,
                preferred_protocol,
                additional_supported_protocols,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x0B,
                    source.value().clone(),
                    destination.value().clone(),
                    protocol_negotiation_payload(
                        authority_level,
                        preferred_protocol,
                        additional_supported_protocols,
                    ),
                ),
                stream_id,
            ), 
            Message::SetNewProtocol {
                source,
                destination,
                authority_level,
                protocol,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x0C,
                    source.value().clone(),
                    destination.value().clone(),
                    set_protocol_payload(
                        authority_level,
                        protocol,
                    ),
                ),
                stream_id,
            ), 
            Message::TestNewProtocolInitiatorToResponder {
                source,
                destination,
                authority_level,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x0D,
                    source.value().clone(),
                    destination.value().clone(),
                    test_protocol_payload(authority_level),
                ),
                stream_id,
            ), 
            Message::TestNewProtocolResponderToInitiator {
                source,
                destination,
                authority_level,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x0E,
                    source.value().clone(),
                    destination.value().clone(),
                    test_protocol_payload(authority_level),
                ),
                stream_id,
            ), 
            Message::ConfirmNewProtocolEstablished {
                source,
                destination,
                authority_level,
            } => ext_sysex::MessageGroup::from_data(
                &ci_data(
                    DeviceId::MidiPort,
                    0x0F,
                    source.value().clone(),
                    destination.value().clone(),
                    vec![authority_level],
                ),
                stream_id,
            ), 
            _ => todo!(),
        }
    }
}

fn append_protocol(p: Protocol, data: &mut Vec<u8>) {
    data.push(match p {
        Protocol::Midi1{..} => 0x01,
        Protocol::Midi2{..} => 0x02,
    });
    data.push(match p {
        Protocol::Midi1{..} => Protocol::MIDI_1_VERSION,
        Protocol::Midi2{..} => Protocol::MIDI_2_VERSION,
    });
    data.push(match p {
        Protocol::Midi1 {
            size_of_packet_extension,
            jitter_reduction_extension,
        } => bitmap(vec![
            (0, jitter_reduction_extension),
            (1, size_of_packet_extension),
        ]),
        Protocol::Midi2 {
            jitter_reduction_extension,
        } => bitmap(vec![
            (0, jitter_reduction_extension),
        ]),
    });
    data.push(0x0);
    data.push(0x0);
}

fn test_protocol_payload(auth_level: u8) -> Vec<u8> {
    let mut payload = vec![auth_level];
    payload.append(&mut (0x00..0x30).collect());
    payload
}

fn protocol_negotiation_payload(
    authority_level: u8,
    preferred_protocol: Protocol,
    additional_supported_protocols: Vec<Protocol>,
) -> Vec<u8> {
    let mut payload = vec![
        authority_level,
        (1 + additional_supported_protocols.len()).try_into().unwrap(),
    ];

    append_protocol(preferred_protocol, &mut payload);
    for p in additional_supported_protocols {
        append_protocol(p, &mut payload);
    }

    payload
}

fn set_protocol_payload(authority_level: u8, protocol: Protocol) -> Vec<u8> {
    let mut payload = vec![
        authority_level,
    ];
    append_protocol(protocol, &mut payload);
    payload
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Protocol {
    Midi1 {
        size_of_packet_extension: bool,
        jitter_reduction_extension: bool,
    },
    Midi2 {
        jitter_reduction_extension: bool,
    },
}

impl Protocol {
    const MIDI_1_VERSION: u8 = 0x0;
    const MIDI_2_VERSION: u8 = 0x0;
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum ProtocolId {
    Midi1,
    Midi2,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}

fn bitmap(data: Vec<(u32, bool)>) -> u8 {
    let mut ret: u8 = 0x0;
    for (v, on) in data {
        if on {
            ret |= 2_u8.pow(v);
        }
    }
    ret
}

fn ci_data(
    device_id: DeviceId,
    category: u8,
    source: [u8; 4],
    destination: [u8; 4],
    mut payload: Vec<u8>
) -> Vec<u8> {
    let mut ret = Vec::with_capacity(16 + payload.len());
    ret.append(&mut vec![
        0x7E,
        match device_id {
            DeviceId::MidiPort => 0x7F,
            DeviceId::Channel(v) => v.into(),
        },
        0x0D,
        category,
        Message::VERSION,
        source[0],
        source[1],
        source[2],
        source[3],
        destination[0],
        destination[1],
        destination[2],
        destination[3],
    ]);
    ret.append(&mut payload);
    ret
}

#[cfg(test)]
mod to_extended_sysex {
    use super::*;

    #[test]
    fn discovery() {
        let source = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::Discovery {
                        source: source.clone(),
                        device_manufacturer: [0x1, 0x2, 0x3],
                        device_family: [0x4, 0x5],
                        device_model_number: [0x5, 0x6],
                        software_version: [0x7, 0x8, 0x9, 0xA],
                        protocol_negotiation_supported: true,
                        profile_configuration_supported: true,
                        property_exchange_supported: true,
                        max_sysex_message_size: [0xB, 0xC, 0xD, 0xE],
                    },
                    0xB,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E,
                    0x7F,
                    0x0D,
                    0x70,
                    0x01,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    0x7F, 0x7F, 0x7F, 0x7F, 
                    0x1, 0x2, 0x3, // device manufacturer
                    0x4, 0x5, // device family
                    0x5, 0x6, // device model
                    0x7, 0x8, 0x9, 0xA, // software version
                    0b0000_1110, // ci support flags
                    0xB, 0xC, 0xD, 0xE, // max message size
                ],
                0xB,
            ),
        );
    }

    #[test]
    fn discovery_reply() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::DiscoveryReply {
                        source: source.clone(),
                        destination: destination.clone(),
                        device_manufacturer: [0x1, 0x2, 0x3],
                        device_family: [0x4, 0x5],
                        device_model_number: [0x5, 0x6],
                        software_version: [0x7, 0x8, 0x9, 0xA],
                        protocol_negotiation_supported: true,
                        profile_configuration_supported: false,
                        property_exchange_supported: true,
                        max_sysex_message_size: [0xB, 0xC, 0xD, 0xE],
                    },
                    0x1,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E,
                    0x7F,
                    0x0D,
                    0x70,
                    0x01,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x1, 0x2, 0x3, // device manufacturer
                    0x4, 0x5, // device family
                    0x5, 0x6, // device model
                    0x7, 0x8, 0x9, 0xA, // software version
                    0b0000_1010, // ci support flags
                    0xB, 0xC, 0xD, 0xE, // max message size
                ],
                0x1,
            )
        );
    }

    #[test]
    fn invalidate_muid() {
        let source = muid::Muid::new();
        let target = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::InvalidateMuid {
                        source: source.clone(),
                        target: target.clone(),
                    },
                    0x4,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E,
                    0x7F,
                    0x0D,
                    0x7E,
                    0x01,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    0x7F, 0x7F, 0x7F, 0x7F, // destination
                    target[muid::Index::Byte1], target[muid::Index::Byte2], 
                    target[muid::Index::Byte3], target[muid::Index::Byte4],
                ],
                0x4,
            )
        );
    }

    #[test]
    fn nak() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::Nak {
                        source: source.clone(),
                        destination: destination.clone(),
                        device_id: DeviceId::Channel(ux::u4::new(0xA)),
                    },
                    0x3,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E,
                    0x0A,
                    0x0D,
                    0x7F,
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                ],
                0x3,
            ),
        );
    }

    #[test]
    fn initiate_protocol_negotiation() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::InitiateProtocolNegotiation {
                        source: source.clone(),
                        destination: destination.clone(),
                        authority_level: 0x2,
                        preferred_protocol: Protocol::Midi1 {
                            size_of_packet_extension: false,
                            jitter_reduction_extension: true,
                        },
                        additional_supported_protocols: vec![
                            Protocol::Midi2 {
                                jitter_reduction_extension: true,
                            },
                        ],
                    },
                    0x3,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E, // universal sysex
                    0x7F, // to midi port
                    0x0D, // midi ci
                    0x0A, // init protocol negotiation
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x2, // auth level
                    2, // number supported protocols
                    0x1,  // midi 1 protocol
                    0x0, // version
                    0b0000_0001, // extension flags
                    0x0, // reserved
                    0x0, // reserved
                    0x2,  // midi 1 protocol
                    0x0, // version
                    0b0000_0001, // extension flags
                    0x0, // reserved
                    0x0, // reserved
                ],
                0x3,
            ),
        );
    }

    #[test]
    fn initiate_protocol_negotiation_reply() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::InitiateProtocolNegotiationReply {
                        source: source.clone(),
                        destination: destination.clone(),
                        authority_level: 0x5,
                        preferred_protocol: Protocol::Midi1 {
                            size_of_packet_extension: true,
                            jitter_reduction_extension: false,
                        },
                        additional_supported_protocols: vec![
                            Protocol::Midi2 {
                                jitter_reduction_extension: false,
                            },
                        ],
                    },
                    0x1,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E, // universal sysex
                    0x7F, // to midi port
                    0x0D, // midi ci
                    0x0B, // init protocol negotiation reply
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x5, // auth level
                    2, // number supported protocols
                    0x1,  // midi 1 protocol
                    0x0, // version
                    0b0000_0010, // extension flags
                    0x0, // reserved
                    0x0, // reserved
                    0x2,  // midi 1 protocol
                    0x0, // version
                    0b0000_0000, // extension flags
                    0x0, // reserved
                    0x0, // reserved
                ],
                0x1,
            ),
        );
    }

    #[test]
    fn set_new_protocol() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::SetNewProtocol {
                        source: source.clone(),
                        destination: destination.clone(),
                        authority_level: 0x3,
                        protocol: Protocol::Midi2 {
                            jitter_reduction_extension: true,
                        },
                    },
                    0x2,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E, // universal sysex
                    0x7F, // to midi port
                    0x0D, // midi ci
                    0x0C, // set protocol
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x3, // auth level
                    0x2,  // midi 2 protocol
                    0x0, // version
                    0b0000_0001, // extension flags
                    0x0, // reserved
                    0x0, // reserved
                ],
                0x2,
            ),
        );
    }

    #[test]
    fn test_new_protocol_initiator_to_responder() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::TestNewProtocolInitiatorToResponder {
                        source: source.clone(),
                        destination: destination.clone(),
                        authority_level: 0x1,
                    },
                    0xA,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E, // universal sysex
                    0x7F, // to midi port
                    0x0D, // midi ci
                    0x0D, // test protocol initiator to responder
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x1, // auth level
                    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
                    0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 
                    0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 
                    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 
                    0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, // test data
                ],
                0xA,
            ),
        );
    }

    #[test]
    fn test_new_protocol_responder_to_initiator() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::TestNewProtocolResponderToInitiator {
                        source: source.clone(),
                        destination: destination.clone(),
                        authority_level: 0x2,
                    },
                    0xC,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E, // universal sysex
                    0x7F, // to midi port
                    0x0D, // midi ci
                    0x0E, // test protocol responder to initiator
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x2, // auth level
                    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
                    0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 
                    0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 
                    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 
                    0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, // test data
                ],
                0xC,
            ),
        );
    }

    #[test]
    fn confirm_new_protocol() {
        let source = muid::Muid::new();
        let destination = muid::Muid::new();
        assert_eq!(
            ext_sysex::MessageGroup::from(
                (
                    Message::ConfirmNewProtocolEstablished {
                        source: source.clone(),
                        destination: destination.clone(),
                        authority_level: 0x6,
                    },
                    0xD,
                ),
            ),
            ext_sysex::MessageGroup::from_data(
                &vec![
                    0x7E, // universal sysex
                    0x7F, // to midi port
                    0x0D, // midi ci
                    0x0F, // confirm new protocol
                    Message::VERSION,
                    source[muid::Index::Byte1], source[muid::Index::Byte2], 
                    source[muid::Index::Byte3], source[muid::Index::Byte4],
                    destination[muid::Index::Byte1], destination[muid::Index::Byte2], 
                    destination[muid::Index::Byte3], destination[muid::Index::Byte4],
                    0x6, // auth level
                ],
                0xD,
            ),
        );
    }
}
