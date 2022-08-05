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

impl From<ChannelVoiceMessage> for Packet {
    fn from(m: ChannelVoiceMessage) -> Self {
        match m {
            ChannelVoiceMessage::NoteOff {
                channel,
                note,
                velocity,
            } => channel_voice_packet(0x8, channel, note, Some(velocity)),
            ChannelVoiceMessage::NoteOn {
                channel,
                note,
                velocity,
            } => channel_voice_packet(0x9, channel, note, Some(velocity)),
            ChannelVoiceMessage::KeyPressure {
                channel,
                note,
                value,
            } => channel_voice_packet(0xA, channel, note, Some(value)),
            ChannelVoiceMessage::ControlChange {
                channel,
                controller,
                value,
            } => channel_voice_packet(0xB, channel, controller, Some(value)),
            ChannelVoiceMessage::ProgramChange {
                channel,
                program,
            } => channel_voice_packet(0xC, channel, program, None),
            ChannelVoiceMessage::ChannelPressure {
                channel,
                value,
            } => channel_voice_packet(0xD, channel, value, None),
            ChannelVoiceMessage::PitchBend {
                channel,
                least_significant_bit,
                most_significant_bit,
            } => channel_voice_packet(
                0xE, 
                channel, 
                least_significant_bit, 
                Some(most_significant_bit),
            ),
        }
    }
}

fn channel_voice_packet(
    status: u8,
    channel: u8, 
    bit1: u8, 
    bit2: Option<u8>
) -> Packet {
    let mut p = Packet {
        data: [
            0x2000_0000,
            0x0,
            0x0,
            0x0,
        ],
    }
    .set_nibble(2, status)
    .set_nibble(3, channel)
    .set_octet(2, bit1);

    if let Some(b) = bit2 {
        p = p.set_octet(3, b);
    }

    p
}

#[cfg(test)]
mod message_from_packet {
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

#[cfg(test)]
mod packet_from_message {
    use super::*;

    #[test]
    fn note_off() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::NoteOff {
                channel: 0xA,
                note: 0x66,
                velocity: 0x88,
            }),
            Packet{ data: [ 0x208A_6688, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn note_on() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::NoteOn {
                channel: 0x3,
                note: 0x39,
                velocity: 0x90,
            }),
            Packet{ data: [ 0x2093_3990, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::KeyPressure {
                channel: 0x5,
                note: 0xF2,
                value: 0x40,
            }),
            Packet{ data: [ 0x20A5_F240, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn control_change() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::ControlChange {
                channel: 0x0,
                controller: 0x30,
                value: 0xD3,
            }),
            Packet{ data: [ 0x20B0_30D3, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn program_change() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::ProgramChange {
                channel: 0x8,
                program: 0xEE,
            }),
            Packet{ data: [ 0x20C8_EE00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::ChannelPressure {
                channel: 0xF,
                value: 0x02,
            }),
            Packet{ data: [ 0x20DF_0200, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            Packet::from(ChannelVoiceMessage::PitchBend {
                channel: 0x0,
                least_significant_bit: 0x88,
                most_significant_bit: 0x77,

            }),
            Packet{ data: [ 0x20E0_8877, 0x0, 0x0, 0x0 ] },
        );
    }
}
