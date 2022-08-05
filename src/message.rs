use crate::{
    midi1_channel_voice, 
    midi2_channel_voice, 
    system,
    Group,
    Packet,
};

#[derive(
    Debug,
    PartialEq,
)]
enum MessageType {
    Utility,
    System(system::Message),
    Midi1ChannelVoice(midi1_channel_voice::Message),
    SystemExclusive,
    Midi2ChannelVoice(midi2_channel_voice::Message),
    ExtendedData,
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
enum MessageParseError {
    InvalidMessageType(u32),
    Midi1ChannelVoiceParseError(midi1_channel_voice::MessageParseError),
    Midi2ChannelVoiceParseError(midi2_channel_voice::MessageParseError),
    SystemParseError(system::MessageParseError),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = MessageParseError;
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
                match system::Message::try_from(p) {
                    Ok(message) => {
                        Ok(Message {
                            group,
                            message_type: MessageType::System(message),
                        })
                    },
                    Err(e) => { Err(MessageParseError::SystemParseError(e)) }
                }
            },
            0x2 => {
                match midi1_channel_voice::Message::try_from(p) {
                    Ok(message) => { 
                        Ok(Message{
                            group,
                            message_type: MessageType::Midi1ChannelVoice(message)
                        }) 
                    },
                    Err(e) => { Err(MessageParseError::Midi1ChannelVoiceParseError(e)) }
                }
            },
            0x3 => {
                Ok(Message {
                    group,
                    message_type: MessageType::SystemExclusive,
                })
            },
            0x4 => {
                match midi2_channel_voice::Message::try_from(p) {
                    Ok(message) => { 
                        Ok(Message{
                            group,
                            message_type: MessageType::Midi2ChannelVoice(message)
                        }) 
                    },
                    Err(e) => { Err(MessageParseError::Midi2ChannelVoiceParseError(e)) }
                }
            },
            0x5 => {
                Ok(Message {
                    group,
                    message_type: MessageType::ExtendedData,
                })
            },
            invalid_type => {
                Err(MessageParseError::InvalidMessageType(invalid_type))
            },
        }
    }
}

impl std::convert::From<Message> for Packet {
    fn from(_m: Message) -> Packet {
        todo!()
    }
}

#[cfg(test)]
mod message_from_packet {
    use super::*;

    #[test]
    fn invalid_message_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x6000_0000,0x0,0x0,0x0]}),
            Err(MessageParseError::InvalidMessageType(0x6)),
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
