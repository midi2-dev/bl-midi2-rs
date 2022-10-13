use super::controllers::Controller;
use crate::{util::truncate, Packet};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Message {
    NoteOff {
        channel: ux::u4,
        note: ux::u7,
        velocity: u16,
        attribute: Option<Attribute>,
    },
    NoteOn {
        channel: ux::u4,
        note: ux::u7,
        velocity: u16,
        attribute: Option<Attribute>,
    },
    KeyPressure {
        channel: ux::u4,
        note: ux::u7,
        data: u32,
    },
    RegisteredPerNoteController {
        channel: ux::u4,
        note: ux::u7,
        controller: Controller,
        data: u32,
    },
    AssignablePerNoteController {
        channel: ux::u4,
        note: ux::u7,
        controller: u8,
        data: u32,
    },
    PerNoteManagement {
        channel: ux::u4,
        note: ux::u7,
        detach: bool,
        reset: bool,
    },
    ControlChange {
        channel: ux::u4,
        index: u8,
        data: u32,
    },
    RegisteredController {
        channel: ux::u4,
        bank: ux::u7,
        index: ux::u7,
        data: u32,
    },
    AssignableController {
        channel: ux::u4,
        bank: ux::u7,
        index: ux::u7,
        data: u32,
    },
    RelativeRegisteredController {
        channel: ux::u4,
        bank: ux::u7,
        index: ux::u7,
        data: u32,
    },
    RelativeAssignableController {
        channel: ux::u4,
        bank: ux::u7,
        index: ux::u7,
        data: u32,
    },
    ProgramChange {
        channel: ux::u4,
        program: ux::u7,
        bank: Option<[ux::u7; 2]>,
    },
    ChannelPressure {
        channel: ux::u4,
        data: u32,
    },
    PitchBend {
        channel: ux::u4,
        data: u32,
    },
    PerNotePitchBend {
        channel: ux::u4,
        note: ux::u7,
        data: u32,
    },
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    IncorrectMessageType(u8),
    InvalidAttributeType(u8),
    InvalidControllerCode(u8),
    InvalidStatusByte(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            0x4 => match u8::from(p.nibble(2)) {
                0x8 => Ok(Message::NoteOff {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    velocity: p.word(2),
                    attribute: attribute(p.octet(3), p.word(3))?,
                }),
                0x9 => Ok(Message::NoteOn {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    velocity: p.word(2),
                    attribute: attribute(p.octet(3), p.word(3))?,
                }),
                0xA => Ok(Message::KeyPressure {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    data: p[1],
                }),
                0x0 => Ok(Message::RegisteredPerNoteController {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    controller: controller(p.octet(3))?,
                    data: p[1],
                }),
                0x1 => Ok(Message::AssignablePerNoteController {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    controller: p.octet(3),
                    data: p[1],
                }),
                0xF => Ok(Message::PerNoteManagement {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    detach: p.bit(30),
                    reset: p.bit(31),
                }),
                0xB => Ok(Message::ControlChange {
                    channel: p.nibble(3),
                    index: truncate(p.octet(2)),
                    data: p[1],
                }),
                0x2 => Ok(Message::RegisteredController {
                    channel: p.nibble(3),
                    bank: truncate(p.octet(2)),
                    index: truncate(p.octet(3)),
                    data: p[1],
                }),
                0x3 => Ok(Message::AssignableController {
                    channel: p.nibble(3),
                    bank: truncate(p.octet(2)),
                    index: truncate(p.octet(3)),
                    data: p[1],
                }),
                0x4 => Ok(Message::RelativeRegisteredController {
                    channel: p.nibble(3),
                    bank: truncate(p.octet(2)),
                    index: truncate(p.octet(3)),
                    data: p[1],
                }),
                0x5 => Ok(Message::RelativeAssignableController {
                    channel: p.nibble(3),
                    bank: truncate(p.octet(2)),
                    index: truncate(p.octet(3)),
                    data: p[1],
                }),
                0xC => Ok(Message::ProgramChange {
                    channel: p.nibble(3),
                    program: truncate(p.octet(4)),
                    bank: if p.bit(31) {
                        Some([
                            truncate(p.octet(6)),
                            truncate(p.octet(7)),
                        ])
                    } else {
                        None
                    },
                }),
                0xD => Ok(Message::ChannelPressure {
                    channel: p.nibble(3),
                    data: p[1],
                }),
                0xE => Ok(Message::PitchBend {
                    channel: p.nibble(3),
                    data: p[1],
                }),
                0x6 => Ok(Message::PerNotePitchBend {
                    channel: p.nibble(3),
                    note: truncate(p.octet(2)),
                    data: p[1],
                }),
                s => Err(DeserializeError::InvalidStatusByte(s)),
            },
            t => Err(DeserializeError::IncorrectMessageType(t)),
        }
    }
}

fn attribute(t: u8, data: u16) -> Result<Option<Attribute>, DeserializeError> {
    match t {
        0x0 => Ok(None),
        0x1 => Ok(Some(Attribute::ManufacturerSpecific(data))),
        0x2 => Ok(Some(Attribute::ProfileSpecific(data))),
        0x3 => Ok(Some(Attribute::Pitch7_9 {
            note: truncate(data >> 9),
            pitch_up: truncate(data),
        })),
        t => Err(DeserializeError::InvalidAttributeType(t)),
    }
}

fn controller(code: u8) -> Result<Controller, DeserializeError> {
    match code {
        1 => Ok(Controller::Modulation),
        2 => Ok(Controller::Breath),
        3 => Ok(Controller::Pitch7_25),
        7 => Ok(Controller::Volume),
        8 => Ok(Controller::Balance),
        10 => Ok(Controller::Pan),
        11 => Ok(Controller::Expression),
        70 => Ok(Controller::SoundController(1)),
        71 => Ok(Controller::SoundController(2)),
        72 => Ok(Controller::SoundController(3)),
        73 => Ok(Controller::SoundController(4)),
        74 => Ok(Controller::SoundController(5)),
        75 => Ok(Controller::SoundController(6)),
        76 => Ok(Controller::SoundController(7)),
        77 => Ok(Controller::SoundController(8)),
        78 => Ok(Controller::SoundController(9)),
        79 => Ok(Controller::SoundController(10)),
        91 => Ok(Controller::EffectDepth(1)),
        92 => Ok(Controller::EffectDepth(2)),
        93 => Ok(Controller::EffectDepth(3)),
        94 => Ok(Controller::EffectDepth(4)),
        95 => Ok(Controller::EffectDepth(5)),
        c => Err(DeserializeError::InvalidControllerCode(c)),
    }
}

#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn incorrect_type() {
        assert_eq!(
            Message::try_from(<Packet as Default>::default()),
            Err(DeserializeError::IncorrectMessageType(0)),
        );
    }

    #[test]
    fn note_off() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x408A_6300, 
                0xABCD_0000,
            ])),
            Ok(Message::NoteOff {
                channel: ux::u4::new(0xA),
                note: ux::u7::new(0x63),
                velocity: 0xABCD,
                attribute: None,
            })
        );
    }

    #[test]
    fn note_on() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4093_5000, 
                0x6666_0000,
            ])),
            Ok(Message::NoteOn {
                channel: ux::u4::new(0x3),
                note: ux::u7::new(0x50),
                velocity: 0x6666,
                attribute: None,
            })
        );
    }

    #[test]
    fn note_on_off_attribute() {
        let data = [
            (0x1, Attribute::ManufacturerSpecific(0xABCD)),
            (0x2, Attribute::ProfileSpecific(0xABCD)),
            (
                0x3,
                Attribute::Pitch7_9 {
                    note: ux::u7::new(0b1010101),
                    pitch_up: ux::u9::new(0b111001101),
                },
            ),
        ];
        for d in data {
            assert_eq!(
                Message::try_from(
                    Packet::from_data(&[
                        0x4097_6E00, 
                        0x1415_ABCD,
                    ]).set_octet(3, d.0)
                ),
                Ok(Message::NoteOn {
                    channel: ux::u4::new(0x7),
                    note: ux::u7::new(0x6E),
                    velocity: 0x1415,
                    attribute: Some(d.1),
                }),
            );
        }
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x40A5_3A00, 
                0xABCD_EF01,
            ])),
            Ok(Message::KeyPressure {
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x3A),
                data: 0xABCD_EF01,
            })
        );
    }

    #[test]
    fn registered_per_note_controller() {
        let data = [
            (1, Controller::Modulation),
            (2, Controller::Breath),
            (3, Controller::Pitch7_25),
            (7, Controller::Volume),
            (8, Controller::Balance),
            (10, Controller::Pan),
            (11, Controller::Expression),
            (70, Controller::SoundController(1)),
            (71, Controller::SoundController(2)),
            (72, Controller::SoundController(3)),
            (73, Controller::SoundController(4)),
            (74, Controller::SoundController(5)),
            (75, Controller::SoundController(6)),
            (76, Controller::SoundController(7)),
            (77, Controller::SoundController(8)),
            (78, Controller::SoundController(9)),
            (79, Controller::SoundController(10)),
            (91, Controller::EffectDepth(1)),
            (92, Controller::EffectDepth(2)),
            (93, Controller::EffectDepth(3)),
            (94, Controller::EffectDepth(4)),
            (95, Controller::EffectDepth(5)),
        ];
        for d in data {
            assert_eq!(
                Message::try_from(
                    Packet::from_data(&[
                        0x4001_4100, 
                        0xABCD_EF01,
                    ]).set_octet(3, d.0)
                ),
                Ok(Message::RegisteredPerNoteController {
                    channel: ux::u4::new(0x1),
                    note: ux::u7::new(0x41),
                    controller: d.1,
                    data: 0xABCD_EF01,
                })
            );
        }
    }

    #[test]
    fn assignable_per_note_controller() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4010_78BE, 
                0x3141_5926,
            ])),
            Ok(Message::AssignablePerNoteController {
                channel: ux::u4::new(0x0),
                note: ux::u7::new(0x78),
                controller: 0xBE,
                data: 0x3141_5926,
            }),
        );
    }

    #[test]
    fn per_note_management() {
        let data = [(false, false), (true, false), (false, true), (true, true)];
        for d in data {
            assert_eq!(
                Message::try_from(
                    Packet::from_data(&[0x40F9_3B00])
                        .set_bit(30, d.0)
                        .set_bit(31, d.1)
                ),
                Ok(Message::PerNoteManagement {
                    channel: ux::u4::new(0x9),
                    note: ux::u7::new(0x3B),
                    detach: d.0,
                    reset: d.1,
                }),
            );
        }
    }

    #[test]
    fn control_change() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x40BC_F100, 
                0x1234_5678,
            ])),
            Ok(Message::ControlChange {
                channel: ux::u4::new(0xC),
                index: 0xF1,
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn registered_controller() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4022_3A40, 
                0x1234_5678,
            ])),
            Ok(Message::RegisteredController {
                channel: ux::u4::new(0x2),
                bank: ux::u7::new(0x3A),
                index: ux::u7::new(0x40),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn assignable_controller() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x403A_3141, 
                0x1234_5678,
            ])),
            Ok(Message::AssignableController {
                channel: ux::u4::new(0xA),
                bank: ux::u7::new(0x31),
                index: ux::u7::new(0x41),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn relative_registered_controller() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4041_5A2E, 
                0x1234_5678,
            ])),
            Ok(Message::RelativeRegisteredController {
                channel: ux::u4::new(0x1),
                bank: ux::u7::new(0x5A),
                index: ux::u7::new(0x2E),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn relative_assignable_controller() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x405C_4E30, 
                0x1234_5678,
            ])),
            Ok(Message::RelativeAssignableController {
                channel: ux::u4::new(0xC),
                bank: ux::u7::new(0x4E),
                index: ux::u7::new(0x30),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn program_change() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x40C3_0000, 
                0x0100_0000,
            ])),
            Ok(Message::ProgramChange {
                channel: ux::u4::new(0x3),
                program: ux::u7::new(0x01),
                bank: None,
            }),
        );
    }

    #[test]
    fn program_change_with_bank() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x40C3_0001, 
                0x1E00_2A55,
            ])),
            Ok(Message::ProgramChange {
                channel: ux::u4::new(0x3),
                program: ux::u7::new(0x1E),
                bank: Some([
                    ux::u7::new(0x2A),
                    ux::u7::new(0x55),
                ]),
            }),
        );
    }

    #[test]
    fn channel_pressure() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x40D5_0000, 
                0x1234_5678,
            ])),
            Ok(Message::ChannelPressure {
                channel: ux::u4::new(0x5),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn pitch_bend() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x40E8_0000, 
                0x1234_5678,
            ])),
            Ok(Message::PitchBend {
                channel: ux::u4::new(0x8),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn per_note_pitch_bend() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4066_3C00, 
                0x1234_5678,
            ])),
            Ok(Message::PerNotePitchBend {
                channel: ux::u4::new(0x6),
                note: ux::u7::new(0x3C),
                data: 0x1234_5678,
            }),
        );
    }
}
