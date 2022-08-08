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
pub enum Message {
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
pub enum DeserializeError {
    UnsupportedStatus(u8),
    IncorrectMessageType(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match p.nibble(0) {
            2 => {
                let channel = p.nibble(3);
                match p.nibble(2) {
                    0x8 => Ok(Message::NoteOff {
                        channel,
                        note: p.octet(2),
                        velocity: p.octet(3),
                    }),
                    0x9 => Ok(Message::NoteOn {
                        channel,
                        note: p.octet(2),
                        velocity: p.octet(3),
                    }),
                    0xA => Ok(Message::KeyPressure {
                        channel,
                        note: p.octet(2),
                        value: p.octet(3),
                    }),
                    0xB => Ok(Message::ControlChange {
                        channel,
                        controller: p.octet(2),
                        value: p.octet(3),
                    }),
                    0xC => Ok(Message::ProgramChange {
                        channel,
                        program: p.octet(2),
                    }),
                    0xD => Ok(Message::ChannelPressure {
                        channel,
                        value: p.octet(2),
                    }),
                    0xE => Ok(Message::PitchBend {
                        channel,
                        least_significant_bit: p.octet(2),
                        most_significant_bit: p.octet(3),
                    }),
                    status => Err(DeserializeError::UnsupportedStatus(status)),
                }
            },
            wrong_type => Err(DeserializeError::IncorrectMessageType(wrong_type)),
        }
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        match m {
            Message::NoteOff {
                channel,
                note,
                velocity,
            } => message_packet(0x8, channel, note, Some(velocity)),
            Message::NoteOn {
                channel,
                note,
                velocity,
            } => message_packet(0x9, channel, note, Some(velocity)),
            Message::KeyPressure {
                channel,
                note,
                value,
            } => message_packet(0xA, channel, note, Some(value)),
            Message::ControlChange {
                channel,
                controller,
                value,
            } => message_packet(0xB, channel, controller, Some(value)),
            Message::ProgramChange {
                channel,
                program,
            } => message_packet(0xC, channel, program, None),
            Message::ChannelPressure {
                channel,
                value,
            } => message_packet(0xD, channel, value, None),
            Message::PitchBend {
                channel,
                least_significant_bit,
                most_significant_bit,
            } => message_packet(
                0xE, 
                channel, 
                least_significant_bit, 
                Some(most_significant_bit),
            ),
        }
    }
}

fn message_packet(
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
    fn wrong_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x1000_0000,0x0,0x0,0x0]}),
            Err(DeserializeError::IncorrectMessageType(0x1)),
        );
    }

    #[test]
    fn note_off() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2A80_3C58,0x0,0x0,0x0]}),
            Ok(Message::NoteOff {
                channel: 0,
                note: 60,
                velocity: 88,
            })
        );
    }

    #[test]
    fn note_on() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2C9D_5020,0x0,0x0,0x0]}),
            Ok(Message::NoteOn {
                channel: 13,
                note: 80,
                velocity: 32,
            })
        );
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            Message::try_from(Packet{data:[0x22A2_3EA0,0x0,0x0,0x0]}),
            Ok(Message::KeyPressure {
                channel: 2,
                note: 62,
                value: 160,
            })
        );
    }

    #[test]
    fn control_change() {
        assert_eq!(
            Message::try_from(Packet{data:[0x21BF_010A,0x0,0x0,0x0]}),
            Ok(Message::ControlChange {
                channel: 15,
                controller: 1,
                value: 10,
            })
        );
    }

    #[test]
    fn program_change() {
        assert_eq!(
            Message::try_from(Packet{data:[0x27C0_A400,0x0,0x0,0x0]}),
            Ok(Message::ProgramChange {
                channel: 0,
                program: 164,
            })
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            Message::try_from(Packet{data:[0x24D4_5300,0x0,0x0,0x0]}),
            Ok(Message::ChannelPressure {
                channel: 4,
                value: 83,
            })
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2BE0_5381,0x0,0x0,0x0]}),
            Ok(Message::PitchBend {
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
            Packet::from(Message::NoteOff {
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
            Packet::from(Message::NoteOn {
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
            Packet::from(Message::KeyPressure {
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
            Packet::from(Message::ControlChange {
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
            Packet::from(Message::ProgramChange {
                channel: 0x8,
                program: 0xEE,
            }),
            Packet{ data: [ 0x20C8_EE00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            Packet::from(Message::ChannelPressure {
                channel: 0xF,
                value: 0x02,
            }),
            Packet{ data: [ 0x20DF_0200, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            Packet::from(Message::PitchBend {
                channel: 0x0,
                least_significant_bit: 0x88,
                most_significant_bit: 0x77,

            }),
            Packet{ data: [ 0x20E0_8877, 0x0, 0x0, 0x0 ] },
        );
    }
}
