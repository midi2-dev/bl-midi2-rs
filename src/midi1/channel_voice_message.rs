use crate::{
    Channel,
    Controller,
    Note,
    Value,
    Velocity,
    Packet,
    Program,
};

#[derive(
    Debug,
    PartialEq,
)]
pub enum ChannelVoiceMessage {
    NoteOff {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    NoteOn {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    KeyPressure {
        channel: Channel,
        note: Note,
        value: Value,
    },
    ControlChange {
        channel: Channel,
        controller: Controller,
        value: Value,
    },
    ProgramChange {
        channel: Channel,
        program: Program,
    },
    ChannelPressure {
        channel: Channel,
        value: Value,    
    },
    PitchBend {
        channel: Channel,
        least_significant_bit: Value,
        most_significant_bit: Value,
    },
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum ChannelVoiceMessageParseError {
    UnsupportedStatus(u8),
}

impl std::convert::TryFrom<Packet> for ChannelVoiceMessage {
    type Error = ChannelVoiceMessageParseError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        let channel = p.nibble(3);
        match p.nibble(2) {
            0x8 => Ok(ChannelVoiceMessage::NoteOff {
                channel,
                note: p.octet(2),
                velocity: p.octet(3),
            }),
            0x9 => Ok(ChannelVoiceMessage::NoteOn {
                channel,
                note: p.octet(2),
                velocity: p.octet(3),
            }),
            0xA => Ok(ChannelVoiceMessage::KeyPressure {
                channel,
                note: p.octet(2),
                value: p.octet(3),
            }),
            0xB => Ok(ChannelVoiceMessage::ControlChange {
                channel,
                controller: p.octet(2),
                value: p.octet(3),
            }),
            0xC => Ok(ChannelVoiceMessage::ProgramChange {
                channel,
                program: p.octet(2),
            }),
            0xD => Ok(ChannelVoiceMessage::ChannelPressure {
                channel,
                value: p.octet(2),
            }),
            0xE => Ok(ChannelVoiceMessage::PitchBend {
                channel,
                least_significant_bit: p.octet(2),
                most_significant_bit: p.octet(3),
            }),
            status => Err(ChannelVoiceMessageParseError::UnsupportedStatus(status)),
        }
    }
}

#[cfg(test)]
mod from_packet_tests {
    use super::*;

    #[test]
    fn note_off() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x2A80_3C58,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::NoteOff {
                channel: 0,
                note: 60,
                velocity: 88,
            })
        );
    }

    #[test]
    fn note_on() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x2C9D_5020,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::NoteOn {
                channel: 13,
                note: 80,
                velocity: 32,
            })
        );
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x22A2_3EA0,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::KeyPressure {
                channel: 2,
                note: 62,
                value: 160,
            })
        );
    }

    #[test]
    fn control_change() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x21BF_010A,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::ControlChange {
                channel: 15,
                controller: 1,
                value: 10,
            })
        );
    }

    #[test]
    fn program_change() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x27C0_A400,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::ProgramChange {
                channel: 0,
                program: 164,
            })
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x24D4_5300,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::ChannelPressure {
                channel: 4,
                value: 83,
            })
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            ChannelVoiceMessage::try_from(Packet{data:[0x2BE0_5381,0x0,0x0,0x0]}),
            Ok(ChannelVoiceMessage::PitchBend {
                channel: 0,
                least_significant_bit: 83,
                most_significant_bit: 129,
            })
        );
    }
}
