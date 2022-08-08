use crate::Packet;

#[derive(
    Debug,
    PartialEq,
)]
pub struct Message {
    pub status: Status,
    pub stream_id: u8,
    pub data: Vec<u8>,
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    /// Value indicates whether the data sent is
    /// previously is valid.
    End(bool),
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    ExpectedStreamId,
    InvalidStatusBit(u8),
    DataOutOfRange(u8),
    IncorrectMessageType(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match p.nibble(0) {
            5 => match p.nibble(2) {
                status if status <= 0x3 => match p.nibble(3) {
                    0 => Err(DeserializeError::ExpectedStreamId),
                    l if l >= 0 && l <= 14 => Ok(Message {
                        status: map_to_status(status),
                        stream_id: p.octet(2),
                        data: p.octets(3, (2 + l).into()),
                    }),
                    l if l == 0xF && status == 0x3 => Ok(Message {
                        status: Status::End(false),
                        stream_id: p.octet(2),
                        data: Vec::new(),
                    }),
                    l => Err(DeserializeError::DataOutOfRange(l)),
                }
                status => Err(DeserializeError::InvalidStatusBit(status)),
            },
            t => Err(DeserializeError::IncorrectMessageType(t)),
        }
    }
}

fn map_to_status(val: u8) -> Status {
    match val {
        0x0 => Status::Complete,
        0x1 => Status::Begin,
        0x2 => Status::Continue,
        0x3 => Status::End(true),
        _ => panic!(),
    }
}

#[cfg(test)]
mod message_from_packet {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x0,0x0,0x0,0x0]}),
            Err(DeserializeError::IncorrectMessageType(0x0)),
        );
    }

    #[test]
    fn invalid_status() {
        assert_eq!(
            Message::try_from(Packet{data:[0x5040_0000, 0x0, 0x0, 0x0]}),
            Err(DeserializeError::InvalidStatusBit(0x4)),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet{data:[0x500F_0000, 0x0, 0x0, 0x0]}),
            Err(DeserializeError::DataOutOfRange(0xF)),
        );
    }

    #[test]
    fn expected_stream_id() {
        assert_eq!(
            Message::try_from(Packet{data:[0x5000_0000, 0x0, 0x0, 0x0]}),
            Err(DeserializeError::ExpectedStreamId),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Message::try_from(Packet{data:[0x5001_BE00, 0x0, 0x0, 0x0]}).unwrap().stream_id,
            0xBE,
        );
    }

    #[test]
    fn status() {
        let data = [
            (0x0, Status::Complete),
            (0x1, Status::Begin),
            (0x2, Status::Continue),
            (0x3, Status::End(true)),
        ];
        for d in data {
            assert_eq!(
                Message::try_from(Packet{data:[0x5001_A000, 0x0, 0x0, 0x0]}.set_nibble(2, d.0)),
                Ok(Message {
                    status: d.1,
                    stream_id: 0xA0,
                    data: Vec::new(),
                })
            );
        }
    }

    #[test]
    fn end_invalid() {
        assert_eq!(
            Message::try_from(Packet{data:[0x503F_A000, 0x0, 0x0, 0x0]}),
            Ok(Message {
                status: Status::End(false),
                stream_id: 0xA0,
                data: Vec::new(),
            }),
        );
    }

    #[test]
    fn data() {
        assert_eq!(
            Message::try_from(Packet {
                data: [
                    0x500E_A123,
                    0x4567_890A,
                    0xBCDE_F123,
                    0x4567_890A,
                ]
            }),
            Ok(Message {
                status: Status::Complete,
                stream_id: 0xA1,
                data: vec![
                    0x23, 0x45, 0x67, 0x89,
                    0x0A, 0xBC, 0xDE, 0xF1,
                    0x23, 0x45, 0x67, 0x89,
                    0x0A,
                ],
            }),
        );
        assert_eq!(
            Message::try_from(Packet {
                data: [
                    0x5003_A1FF,
                    0xFF00_0000,
                    0x0, 0x0,
                ]
            }),
            Ok(Message {
                status: Status::Complete,
                stream_id: 0xA1,
                data: vec![0xFF, 0xFF],
            }),
        );
    }
}
