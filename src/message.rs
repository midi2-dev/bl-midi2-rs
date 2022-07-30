use crate::{
    midi1, 
    midi2,
    Group,
    Packet,
};

#[derive(
    Debug,
    PartialEq,
)]
enum MessageType {
    Utility,
    System,
    Midi1ChannelVoice(midi1::ChannelVoiceMessage),
    Data,
    Midi2ChannelVoice(midi2::ChannelVoiceMessage),
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
    Midi1ChannelVoiceParseError(midi1::ChannelVoiceMessageParseError),
    Midi2ChannelVoiceParseError(midi2::ChannelVoiceMessageParseError),
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
                Ok(Message {
                    group,
                    message_type: MessageType::System,
                })
            },
            0x2 => {
                match midi1::ChannelVoiceMessage::try_from(p) {
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
                    message_type: MessageType::Data,
                })
            },
            0x4 => {
                match midi2::ChannelVoiceMessage::try_from(p) {
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

#[cfg(test)]
mod from_packet_tests {
    use super::*;

    #[test]
    fn invalid_message_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x6000_0000,0x0,0x0,0x0]}),
            Err(MessageParseError::InvalidMessageType(0x6)),
        );
    }

    #[test]
    fn midi1_note_off() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2A80_3C58,0x0,0x0,0x0]}),
            Ok(Message {
                group: 10,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::NoteOff {
                        channel: 0,
                        note: 60,
                        velocity: 88,
                    }
                )
            })
        );
    }

    #[test]
    fn midi1_note_on() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2C9D_5020,0x0,0x0,0x0]}),
            Ok(Message {
                group: 12,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::NoteOn {
                        channel: 13,
                        note: 80,
                        velocity: 32,
                    }
                )
            })
        );
    }

    #[test]
    fn midi1_key_pressure() {
        assert_eq!(
            Message::try_from(Packet{data:[0x22A2_3EA0,0x0,0x0,0x0]}),
            Ok(Message {
                group: 2,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::KeyPressure {
                        channel: 2,
                        note: 62,
                        value: 160,
                    }
                )
            })
        );
    }

    #[test]
    fn midi1_control_change() {
        assert_eq!(
            Message::try_from(Packet{data:[0x21BF_010A,0x0,0x0,0x0]}),
            Ok(Message {
                group: 1,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::ControlChange {
                        channel: 15,
                        controller: 1,
                        value: 10,
                    }
                )
            })
        );
    }

    #[test]
    fn midi1_program_change() {
        assert_eq!(
            Message::try_from(Packet{data:[0x27C0_A400,0x0,0x0,0x0]}),
            Ok(Message {
                group: 7,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::ProgramChange {
                        channel: 0,
                        program: 164,
                    }
                )
            })
        );
    }

    #[test]
    fn midi1_channel_pressure() {
        assert_eq!(
            Message::try_from(Packet{data:[0x24D4_5300,0x0,0x0,0x0]}),
            Ok(Message {
                group: 4,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::ChannelPressure {
                        channel: 4,
                        value: 83,
                    }
                )
            })
        );
    }

    #[test]
    fn midi1_pitch_bend() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2BE0_5381,0x0,0x0,0x0]}),
            Ok(Message {
                group: 11,
                message_type: MessageType::Midi1ChannelVoice(
                    midi1::ChannelVoiceMessage::PitchBend {
                        channel: 0,
                        least_significant_bit: 83,
                        most_significant_bit: 129,
                    }
                )
            })
        );
    }
}
