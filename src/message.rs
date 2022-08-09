use crate::{
    midi1_channel_voice, 
    midi2_channel_voice, 
    system_common,
    system_exclusive,
    Group,
    Packet,
};

#[derive(
    Debug,
    PartialEq,
)]
enum MessageType {
    Utility,
    System(system_common::Message),
    Midi1ChannelVoice(midi1_channel_voice::Message),
    SystemExclusive(system_exclusive::Message),
    Midi2ChannelVoice(midi2_channel_voice::Message),
    ExtendedSystemExclusive,
}

#[derive(
    Debug,
    PartialEq,
)]
struct Message {
    group: Group,
    message_type: MessageType,
}

#[derive(
    Debug,
    PartialEq,
)]
enum DeserializeError {
    InvalidMessageType(u32),
    Midi1ChannelVoice(midi1_channel_voice::DeserializeError),
    Midi2ChannelVoice(midi2_channel_voice::DeserializeError),
    SystemCommon(system_common::DeserializeError),
    SystemExclusive(system_exclusive::DeserializeError),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        let group = p.group();
        return match p.data[0] >> 28 {
            0x0 => {
                Ok(Message {
                    group,
                    message_type: MessageType::Utility,
                })
            },
            0x1 => {
                match system_common::Message::try_from(p) {
                    Ok(message) => Ok(Message {
                        group,
                        message_type: MessageType::System(message),
                    }),
                    Err(e) => Err(DeserializeError::SystemCommon(e))
                }
            },
            0x2 => {
                match midi1_channel_voice::Message::try_from(p) {
                    Ok(message) => Ok(Message{
                        group,
                        message_type: MessageType::Midi1ChannelVoice(message)
                    }),
                    Err(e) => Err(DeserializeError::Midi1ChannelVoice(e))
                }
            },
            0x3 => {
                match system_exclusive::Message::try_from(p) {
                    Ok(message) => Ok(Message{
                        group,
                        message_type: MessageType::SystemExclusive(message)
                    }),
                    Err(e) => Err(DeserializeError::SystemExclusive(e)),
                }
            },
            0x4 => {
                match midi2_channel_voice::Message::try_from(p) {
                    Ok(message) => { 
                        Ok(Message{
                            group,
                            message_type: MessageType::Midi2ChannelVoice(message)
                        }) 
                    },
                    Err(e) => Err(DeserializeError::Midi2ChannelVoice(e))
                }
            },
            0x5 => {
                Ok(Message {
                    group,
                    message_type: MessageType::ExtendedSystemExclusive,
                })
            },
            invalid_type => Err(DeserializeError::InvalidMessageType(invalid_type)),
        }
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
            Message::try_from(Packet{data:[0x6000_0000,0x0,0x0,0x0]}),
            Err(DeserializeError::InvalidMessageType(0x6)),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2A80_3C58,0x0,0x0,0x0]})
                .unwrap()
                .group,
            10,
        );
    }
}
