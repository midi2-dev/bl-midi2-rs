use crate::{
    ExtendedSysExDeserializeError, ExtendedSysExMessage, Group, Midi1ChannelVoiceDeserializeError,
    Midi1ChannelVoiceMessage, Midi2ChannelVoiceDeserializeError, Midi2ChannelVoiceMessage, Packet,
    SysCommonDeserializeError, SysCommonMessage, SysExDeserializeError, SysExMessage,
    UtilityDeserializeError, UtilityMessage,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
enum MessageType {
    Utility(UtilityMessage),
    System(SysCommonMessage),
    Midi1ChannelVoice(Midi1ChannelVoiceMessage),
    SystemExclusive(SysExMessage),
    Midi2ChannelVoice(Midi2ChannelVoiceMessage),
    ExtendedSystemExclusive(ExtendedSysExMessage),
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    group: Group,
    message_type: MessageType,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    ExtendedSystemExclusive(ExtendedSysExDeserializeError),
    InvalidMessageType(u32),
    Midi1ChannelVoice(Midi1ChannelVoiceDeserializeError),
    Midi2ChannelVoice(Midi2ChannelVoiceDeserializeError),
    SystemCommon(SysCommonDeserializeError),
    SystemExclusive(SysExDeserializeError),
    Utility(UtilityDeserializeError),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        let group = p.group();
        return match p.data[0] >> 28 {
            0x0 => match UtilityMessage::try_from(p) {
                Ok(message) => Ok(Message {
                    group,
                    message_type: MessageType::Utility(message),
                }),
                Err(e) => Err(DeserializeError::Utility(e)),
            },
            0x1 => match SysCommonMessage::try_from(p) {
                Ok(message) => Ok(Message {
                    group,
                    message_type: MessageType::System(message),
                }),
                Err(e) => Err(DeserializeError::SystemCommon(e)),
            },
            0x2 => match Midi1ChannelVoiceMessage::try_from(p) {
                Ok(message) => Ok(Message {
                    group,
                    message_type: MessageType::Midi1ChannelVoice(message),
                }),
                Err(e) => Err(DeserializeError::Midi1ChannelVoice(e)),
            },
            0x3 => match SysExMessage::try_from(p) {
                Ok(message) => Ok(Message {
                    group,
                    message_type: MessageType::SystemExclusive(message),
                }),
                Err(e) => Err(DeserializeError::SystemExclusive(e)),
            },
            0x4 => match Midi2ChannelVoiceMessage::try_from(p) {
                Ok(message) => Ok(Message {
                    group,
                    message_type: MessageType::Midi2ChannelVoice(message),
                }),
                Err(e) => Err(DeserializeError::Midi2ChannelVoice(e)),
            },
            0x5 => match ExtendedSysExMessage::try_from(p) {
                Ok(message) => Ok(Message {
                    group,
                    message_type: MessageType::ExtendedSystemExclusive(message),
                }),
                Err(e) => Err(DeserializeError::ExtendedSystemExclusive(e)),
            },
            invalid_type => Err(DeserializeError::InvalidMessageType(invalid_type)),
        };
    }
}

impl std::convert::From<Message> for Packet {
    fn from(_m: Message) -> Packet {
        todo!()
    }
}

#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn invalid_message_type() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x6000_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::InvalidMessageType(0x6)),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x2A80_3C58, 0x0, 0x0, 0x0]
            })
            .unwrap()
            .group,
            10,
        );
    }
}
