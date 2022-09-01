use crate::{std_error_impl, helpers::mask, Packet};

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub enum Message {
    NoteOff {
        channel: ux::u4,
        note: ux::u7,
        velocity: ux::u7,
    },
    NoteOn {
        channel: ux::u4,
        note: ux::u7,
        velocity: ux::u7,
    },
    KeyPressure {
        channel: ux::u4,
        note: ux::u7,
        value: ux::u7,
    },
    ControlChange {
        channel: ux::u4,
        controller: ux::u7,
        value: ux::u7,
    },
    ProgramChange {
        channel: ux::u4,
        program: ux::u7,
    },
    ChannelPressure {
        channel: ux::u4,
        value: ux::u7,
    },
    PitchBend {
        channel: ux::u4,
        data: [ux::u7; 2],
    },
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    UnsupportedStatus(u8),
    IncorrectMessageType(u8),
}
std_error_impl!(DeserializeError);

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            2 => {
                let channel = p.nibble(3);
                match u8::from(p.nibble(2)) {
                    0x8 => Ok(Message::NoteOff {
                        channel,
                        note: mask(p.octet(2)),
                        velocity: mask(p.octet(3)),
                    }),
                    0x9 => Ok(Message::NoteOn {
                        channel,
                        note: mask(p.octet(2)),
                        velocity: mask(p.octet(3)),
                    }),
                    0xA => Ok(Message::KeyPressure {
                        channel,
                        note: mask(p.octet(2)),
                        value: mask(p.octet(3)),
                    }),
                    0xB => Ok(Message::ControlChange {
                        channel,
                        controller: mask(p.octet(2)),
                        value: mask(p.octet(3)),
                    }),
                    0xC => Ok(Message::ProgramChange {
                        channel,
                        program: mask(p.octet(2)),
                    }),
                    0xD => Ok(Message::ChannelPressure {
                        channel,
                        value: mask(p.octet(2)),
                    }),
                    0xE => Ok(Message::PitchBend {
                        channel,
                        data: [
                            mask(p.octet(2)),
                            mask(p.octet(3)),
                        ],
                    }),
                    status => Err(DeserializeError::UnsupportedStatus(status)),
                }
            }
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
            } => message_packet(0x8, channel, note.into(), Some(velocity)),
            Message::NoteOn {
                channel,
                note,
                velocity,
            } => message_packet(0x9, channel, note.into(), Some(velocity)),
            Message::KeyPressure {
                channel,
                note,
                value,
            } => message_packet(0xA, channel, note.into(), Some(value)),
            Message::ControlChange {
                channel,
                controller,
                value,
            } => message_packet(0xB, channel, controller, Some(value)),
            Message::ProgramChange { channel, program } => {
                message_packet(0xC, channel, program, None)
            }
            Message::ChannelPressure { channel, value } => {
                message_packet(0xD, channel, value, None)
            }
            Message::PitchBend {
                channel,
                data: [lsb, msb],
            } => message_packet(0xE, channel, lsb, Some(msb)),
        }
    }
}

fn message_packet(status: u8, channel: ux::u4, bit1: ux::u7, bit2: Option<ux::u7>) -> Packet {
    let mut p = Packet {
        data: [0x2000_0000, 0x0, 0x0, 0x0],
    }
    .set_nibble(2, mask(status))
    .set_nibble(3, channel)
    .set_octet(2, bit1.into());

    if let Some(b) = bit2 {
        p = p.set_octet(3, b.into());
    }

    p
}

#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn wrong_type() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x1000_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::IncorrectMessageType(0x1)),
        );
    }

    #[test]
    fn note_off() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x2A80_3C58, 0x0, 0x0, 0x0]
            }),
            Ok(Message::NoteOff {
                channel: ux::u4::new(0),
                note: ux::u7::new(60),
                velocity: ux::u7::new(88),
            })
        );
    }

    #[test]
    fn note_on() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x2C9D_5020, 0x0, 0x0, 0x0]
            }),
            Ok(Message::NoteOn {
                channel: ux::u4::new(13),
                note: ux::u7::new(80),
                velocity: ux::u7::new(32),
            })
        );
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x22A2_7F5D, 0x0, 0x0, 0x0]
            }),
            Ok(Message::KeyPressure {
                channel: ux::u4::new(2),
                note: ux::u7::new(0x7F),
                value: ux::u7::new(0x5D),
            })
        );
    }

    #[test]
    fn control_change() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x21BF_010A, 0x0, 0x0, 0x0]
            }),
            Ok(Message::ControlChange {
                channel: ux::u4::new(15),
                controller: ux::u7::new(1),
                value: ux::u7::new(10),
            })
        );
    }

    #[test]
    fn program_change() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x27C0_6600, 0x0, 0x0, 0x0]
            }),
            Ok(Message::ProgramChange {
                channel: ux::u4::new(0),
                program: ux::u7::new(0x66),
            })
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x24D4_5300, 0x0, 0x0, 0x0]
            }),
            Ok(Message::ChannelPressure {
                channel: ux::u4::new(4),
                value: ux::u7::new(83),
            })
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x2BE0_533C, 0x0, 0x0, 0x0]
            }),
            Ok(Message::PitchBend {
                channel: ux::u4::new(0),
                data: [
                    ux::u7::new(0x53),
                    ux::u7::new(0x3C),
                ],
            })
        );
    }
}

#[cfg(test)]
mod serialize {
    use super::*;

    #[test]
    fn note_off() {
        assert_eq!(
            Packet::from(Message::NoteOff {
                channel: ux::u4::new(0xA),
                note: ux::u7::new(0x66),
                velocity: ux::u7::new(0x5A),
            }),
            Packet {
                data: [0x208A_665A, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn note_on() {
        assert_eq!(
            Packet::from(Message::NoteOn {
                channel: ux::u4::new(0x3),
                note: ux::u7::new(0x39),
                velocity: ux::u7::new(0x40),
            }),
            Packet {
                data: [0x2093_3940, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            Packet::from(Message::KeyPressure {
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x7F),
                value: ux::u7::new(0x40),
            }),
            Packet {
                data: [0x20A5_7F40, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn control_change() {
        assert_eq!(
            Packet::from(Message::ControlChange {
                channel: ux::u4::new(0x0),
                controller: ux::u7::new(0x30),
                value: ux::u7::new(0x32),
            }),
            Packet {
                data: [0x20B0_3032, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn program_change() {
        assert_eq!(
            Packet::from(Message::ProgramChange {
                channel: ux::u4::new(0x8),
                program: ux::u7::new(0x04),
            }),
            Packet {
                data: [0x20C8_0400, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            Packet::from(Message::ChannelPressure {
                channel: ux::u4::new(0xF),
                value: ux::u7::new(0x02),
            }),
            Packet {
                data: [0x20DF_0200, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            Packet::from(Message::PitchBend {
                channel: ux::u4::new(0x0),
                data: [ 
                    ux::u7::new(0x5F),
                    ux::u7::new(0x77),
                ],
            }),
            Packet {
                data: [0x20E0_5F77, 0x0, 0x0, 0x0]
            },
        );
    }
}
