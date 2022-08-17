use crate::{
    helpers::mask,
    Packet,
};

#[derive(
    Debug,
    PartialEq,
)]
pub struct Message {
    pub status: Status,
    pub data: Vec<u8>,
}

#[derive(
    Debug,
    PartialEq,
)]
#[repr(u8)]
pub enum Status {
    Complete = 0x0,
    Begin = 0x1,
    Continue = 0x2,
    End = 0x3,
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    InvalidStatusBit(u8),
    DataOutOfRange(u8),
    IncorrectMessageType(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            3 => match u8::from(p.nibble(2)) {
                0x0..=0x3 => match u8::from(p.nibble(3)) {
                    0..=5 => Ok(Message {
                        status: map_status_bit(p.nibble(2)),
                        // see comment: ux missing From usize impls
                        data: p.octets(2, (2 + u8::from(p.nibble(3))).into()),
                    }),
                    overflow_len => Err(DeserializeError::DataOutOfRange(overflow_len)),
                },
                status => Err(DeserializeError::InvalidStatusBit(status)),
            },
            wrong_type => Err(DeserializeError::IncorrectMessageType(wrong_type)),
        }
    }
}

impl std::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        Packet { data: [0x3000_0000, 0x0, 0x0, 0x0 ] }
            .set_nibble(2, mask(m.status as u8))
            // see comment: ux missing From usize impls
            .set_nibble(3, u8::try_from(m.data.len()).unwrap().try_into().unwrap())
            .set_octets(2, m.data)
    }
}

fn map_status_bit(b: ux::u4) -> Status {
    match u8::from(b) {
        0x0 => Status::Complete,
        0x1 => Status::Begin,
        0x2 => Status::Continue,
        0x3 => Status::End,
        _ => panic!(),
    }
}

#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2000_0000,0x0,0x0,0x0]}),
            Err(DeserializeError::IncorrectMessageType(0x2)),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from(Packet{data:[0x30A0_0000,0x0,0x0,0x0]}),
            Err(DeserializeError::InvalidStatusBit(0xA)),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet{data:[0x3009_0000,0x0,0x0,0x0]}),
            Err(DeserializeError::DataOutOfRange(0x9)),
        );
    }

    #[test]
    fn complete_message() {
        assert_eq!(
            Message::try_from(Packet{data:[0x3003_1234,0x5600_0000,0x0,0x0]}),
            Ok(Message {
                status: Status::Complete,
                data: vec![0x12,0x34,0x56],
            }),
        );
    }

    #[test]
    fn begin_message() {
        assert_eq!(
            Message::try_from(Packet{data:[0x3012_ABCD,0x0,0x0,0x0]}),
            Ok(Message {
                status: Status::Begin,
                data: vec![0xAB,0xCD],
            }),
        );
    }

    #[test]
    fn continue_status() {
        assert_eq!(
            Message::try_from(Packet{data:[0x3025_3141,0x1592_6500,0x0,0x0]}),
            Ok(Message {
                status: Status::Continue,
                data: vec![0x31,0x41,0x15,0x92,0x65],
            }),
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Message::try_from(Packet{data:[0x3030_0000,0x0,0x0,0x0]}),
            Ok(Message {
                status: Status::End,
                data: Vec::new(),
            }),
        );
    }
}

#[cfg(test)]
mod serialize {
    use super::*;

    #[test]
    fn complete() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Complete,
                data: vec![0xAB, 0xCD],
            }),
            Packet{data:[0x3002_ABCD, 0x0, 0x0, 0x0]},
        );
    }

    #[test]
    fn begin() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Begin,
                data: vec![0x14, 0x14, 0x21, 0x35, 0x62, 0x37],
            }),
            Packet{data:[0x3016_1414, 0x2135_6237, 0x0, 0x0]},
        );
    }

    #[test]
    fn continue_status() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Continue,
                data: vec![0xFF, 0xFF, 0xFF]
            }),
            Packet{data:[0x3023_FFFF, 0xFF00_0000, 0x0, 0x0]},
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Packet::from(Message {
                status: Status::End,
                data: vec![],
            }),
            Packet{data:[0x3030_0000, 0x0, 0x0, 0x0]},
        );
    }
}

// ux missing From usize impls:
//
// the ux crate does note imlement From 
// for usize on its types :-/
// Should be forthcoming in a future release.
