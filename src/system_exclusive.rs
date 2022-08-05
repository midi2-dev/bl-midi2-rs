use crate::Packet;

#[derive(
    Debug,
    PartialEq,
)]
pub struct Message {
    status: Status,
    data: Vec<u8>,
}

#[derive(
    Debug,
    PartialEq,
)]
#[repr(u8)]
enum Status {
    Complete = 0x0,
    Begin = 0x1,
    Continue = 0x2,
    End = 0x3,
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum MessageParseError {
    InvalidStatusBit(u8),
    DataOutOfRange(u8),
    IncorrectMessageType(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = MessageParseError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match p.nibble(0) {
            3 => {
                match p.nibble(2) {
                    0x0..=0x3 => {
                        match p.nibble(3) {
                            len if len <= 5 => Ok(Message {
                                status: map_status_bit(p.nibble(2)),
                                data: {
                                    let mut d = Vec::new();
                                    for i in 0..p.nibble(3) { 
                                        d.push(p.octet((i + 2).into()));
                                    }
                                    d
                                },
                            }),
                            overflow_len => Err(MessageParseError::DataOutOfRange(overflow_len)),
                        }
                    },
                    status => Err(MessageParseError::InvalidStatusBit(status)),
                }
            },
            wrong_type => Err(MessageParseError::IncorrectMessageType(wrong_type)),
        }
    }
}

fn map_status_bit(b: u8) -> Status {
    match b {
        0x0 => Status::Complete,
        0x1 => Status::Begin,
        0x2 => Status::Continue,
        0x3 => Status::End,
        _ => panic!(),
    }
}

#[cfg(test)]
mod message_from_packet {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x2000_0000,0x0,0x0,0x0]}),
            Err(MessageParseError::IncorrectMessageType(0x2)),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from(Packet{data:[0x30A0_0000,0x0,0x0,0x0]}),
            Err(MessageParseError::InvalidStatusBit(0xA)),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet{data:[0x3009_0000,0x0,0x0,0x0]}),
            Err(MessageParseError::DataOutOfRange(0x9)),
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
