use crate::Packet;
use super::controllers::Controller;

#[derive(
    Debug,
    PartialEq,
)]
pub enum Message {
    NoteOff {
        channel: u8,
        note: u8,
        velocity: u16,
        attribute: Option<Attribute>,
    },
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u16,
        attribute: Option<Attribute>,
    },
    KeyPressure {
        channel: u8,
        note: u8,
        data: u32,
    },
    RegisteredPerNoteController {
        channel: u8,
        note: u8,
        controller: Controller,
        data: u32,
    },
    AssignablePerNoteController {
        channel: u8,
        note: u8,
        controller: u8,
        data: u32,
    },
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum Attribute {
    ManufacturerSpecific(u16),
    ProfileSpecific(u16),
    Pitch7_9(pitch_7_9::Value),
}

pub mod pitch_7_9 {
    #[derive(
        Debug,
        PartialEq,
    )]
    pub struct Value(pub u16);

    impl Value {
        pub fn note(&self) -> u16 {
            self.0 >> 9
        }
        pub fn pitch_up(&self) -> u16 {
            self.0 & 0b0000_0001_1111_1111
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn note() {
            assert_eq!(
                Value(0b0010_0100_0011_1111).note(),
                0b0010010,
            );
        }

        #[test]
        fn pitch_up() {
            assert_eq!(
                Value(0b0010_0100_0011_1111).pitch_up(),
                0b000111111,
            );
        }
    }
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    IncorrectMessageType(u8),
    InvalidAttributeType(u8),
    InvalidControllerCode(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match p.nibble(0) {
            0x4 => match p.nibble(2) {
                0x8 => Ok(Message::NoteOff {
                    channel: p.nibble(3),
                    note: p.octet(2),
                    velocity: p.word(2),
                    attribute: attribute(p.octet(3), p.word(3))?,
                }),
                0x9 => Ok(Message::NoteOn {
                    channel: p.nibble(3),
                    note: p.octet(2),
                    velocity: p.word(2),
                    attribute: attribute(p.octet(3), p.word(3))?,
                }),
                0xA => Ok(Message::KeyPressure {
                    channel: p.nibble(3),
                    note: p.octet(2),
                    data: p.data[1],
                }),
                0x0 => Ok(Message::RegisteredPerNoteController {
                    channel: p.nibble(3),
                    note: p.octet(2),
                    controller: controller(p.octet(3))?,
                    data: p.data[1],
                }),
                0x1 => Ok(Message::AssignablePerNoteController {
                    channel: p.nibble(3),
                    note: p.octet(2),
                    controller: p.octet(3),
                    data: p.data[1],
                }),
                _ => panic!(),
            }
            t => Err(DeserializeError::IncorrectMessageType(t)),
        }
    }
}

fn attribute(t: u8, data: u16) -> Result<Option<Attribute>, DeserializeError> {
    match t {
        0x0 => Ok(None),
        0x1 => Ok(Some(Attribute::ManufacturerSpecific(data))),
        0x2 => Ok(Some(Attribute::ProfileSpecific(data))),
        0x3 => Ok(Some(Attribute::Pitch7_9(pitch_7_9::Value(data)))),
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
            Message::try_from(Packet{data:[0x0,0x0,0x0,0x0]}),
            Err(DeserializeError::IncorrectMessageType(0)),
        );
    }

    #[test]
    fn note_off() {
        assert_eq!(
            Message::try_from(Packet{data:[0x408A_D300,0xABCD_0000,0x0,0x0]}),
            Ok(Message::NoteOff {
                channel: 0xA,
                note: 0xD3,
                velocity: 0xABCD,
                attribute: None,
            })
        );
    }
    
    #[test]
    fn note_on() {
        assert_eq!(
            Message::try_from(Packet{data:[0x4093_5000,0x6666_0000,0x0,0x0]}),
            Ok(Message::NoteOn {
                channel: 0x3,
                note: 0x50,
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
            (0x3, Attribute::Pitch7_9(pitch_7_9::Value(0xABCD))),
        ];
        for d in data {
            assert_eq!(
                Message::try_from(
                    Packet{
                        data:[0x4097_F000,0x1415_ABCD,0x0,0x0]
                    }.set_octet(3, d.0)
                ),
                Ok(Message::NoteOn {
                    channel: 0x7,
                    note: 0xF0,
                    velocity: 0x1415,
                    attribute: Some(d.1),
                }),
            );
        }
    }

    #[test]
    fn key_pressure() {
        assert_eq!(
            Message::try_from(Packet{data:[0x40A5_EA00,0xABCD_EF01,0x0,0x0]}),
            Ok(Message::KeyPressure {
                channel: 0x5,
                note: 0xEA,
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
                Message::try_from(Packet {
                    data:[0x4001_8100,0xABCD_EF01,0x0,0x0]
                }.set_octet(3, d.0)),
                Ok(Message::RegisteredPerNoteController {
                    channel: 0x1,
                    note: 0x81,
                    controller: d.1,
                    data: 0xABCD_EF01,
                })
            );
        }
    }

    #[test]
    fn assignable_per_note_controller() {
        assert_eq!(
            Message::try_from(Packet{data:[0x4010_78BE,0x3141_5926,0x0,0x0]}),
            Ok(Message::AssignablePerNoteController {
                channel: 0x0,
                note: 0x78,
                controller: 0xBE,
                data: 0x3141_5926,
            }),
        );
    }
}
