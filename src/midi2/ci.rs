use super::muid::Muid;
use crate::extended_system_exclusive as ext_sysex;

enum Message {
    Discovery {
        source: Muid,
        device_manufacturer: [u8; 3],
        device_family: [u8; 2],
        model_number: [u8; 2],
        software_version: [u8; 4],
        protocol_negotiation_supported: bool,
        profile_configuration_supported: bool,
        property_exchange_supported: bool,
        max_sysex_message_size: [u8; 4],
    },
    DiscoveryReply {
        source: Muid,
        destination: Muid,
        device_manufacturer: [u8; 3],
        device_family: [u8; 2],
        model_number: [u8; 2],
        software_version: [u8; 4],
        protocol_negotiation_supported: bool,
        profile_configuration_supported: bool,
        property_exchange_supported: bool,
        max_sysex_message_size: [u8; 4],
    },
}

impl Message {
    fn version() -> u8 {
        0x01
    }
}

impl std::convert::From<Message> for Vec<ext_sysex::Message> {
    fn from(m: Message) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod to_extended_sysex {
    use super::*;

    #[test]
    fn discovery() {
        let source = Muid::new();
        assert_eq!(
            Vec::<ext_sysex::Message>::from(Message::Discovery {
                source: source.clone(),
                device_manufacturer: [0x0, 0x0, 0x0],
                device_family: [0x0, 0x0],
                model_number: [0x0, 0x0],
                software_version: [0x0, 0x0, 0x0, 0x0],
                protocol_negotiation_supported: true,
                profile_configuration_supported: true,
                property_exchange_supported: true,
                max_sysex_message_size: [0x0, 0x0, 0x0, 0x0]
            }),
            ext_sysex::Builder::new().data(vec![
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                0x01,
                source.value()[0], source.value()[1], 
                source.value()[2], source.value()[3],
                0x7F, 0x7F, 0x7F, 0x7F, 
                0x0, 0x0, 0x0, // device manufacturer
                0x0, 0x0, // device family
                0x0, 0x0, // device model
                0x0, 0x0, 0x0, 0x0, // software version
                0x0, // ci support flags
                0x0, 0x0, 0x0, 0x0, // max message size
                0xF7,
            ]).build(),
        );
    }
}
