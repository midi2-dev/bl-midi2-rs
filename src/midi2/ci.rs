use super::*;
use crate::extended_system_exclusive as ext_sysex;

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
    InitiateProtocolNegotiation,
    InitiateProtocolNegotiationReply,
    SetProtocolNegotiation,
    TestNewProtocolInitiatorToResponder,
    TestNewProtocolResponderToInitiator,
    ConfirmNewProtocolEstablished,
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
            _ => todo!(),
        }
    }
}

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
}
