use crate::Packet;

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    status: Status,
    stream_id: u8,
    data: Vec<u8>,
}

impl Message {
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn stream_id(&self) -> u8 {
        self.stream_id
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[derive(Clone)]
pub struct Builder {
    data: Vec<u8>,
    stream_id: u8,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            data: Vec::new(),
            stream_id: 0,
        }
    }

    pub fn build(self) -> Vec<Message> {
        if self.data.len() <= 13 {
            vec![
                Message {
                    status: Status::Complete,
                    stream_id: self.stream_id,
                    data: self.data,
                }
            ]
        } else {
            let mut ret = Vec::new();

            for chunk in self.data.chunks(13) {
                ret.push(Message{
                    status: Status::Continue,
                    stream_id: self.stream_id,
                    data: chunk
                        .iter()
                        .map(|v| v.clone())
                        .collect(),
                });
            }

            let l = ret.len();
            ret[0].status = Status::Begin;
            ret[l - 1].status = Status::End(true);

            ret
        }
    }

    pub fn data(mut self, d: Vec<u8>) -> Self {
        self.data = d;
        self
    }

    pub fn stream_id(mut self, id: u8) -> Self {
        self.stream_id = id;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    /// Value indicates whether the data sent is
    /// previously is valid.
    End(bool),
}

#[derive(
    Clone,
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
        match u8::from(p.nibble(0)) {
            5 => match u8::from(p.nibble(2)) {
                status if status <= 0x3 => match u8::from(p.nibble(3)) {
                    0 => Err(DeserializeError::ExpectedStreamId),
                    l if l < 0xF => Ok(Message {
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
                },
                status => Err(DeserializeError::InvalidStatusBit(status)),
            },
            t => Err(DeserializeError::IncorrectMessageType(t)),
        }
    }
}

impl std::convert::From<Message> for Packet {
    fn from(
        Message {
            status,
            stream_id,
            data,
        }: Message,
    ) -> Self {
        Packet {
            data: [0x5000_0000, 0x0, 0x0, 0x0],
        }
        .set_nibble(2, map_from_status(status))
        .set_nibble(3, (data.len() + 1).try_into().unwrap())
        .set_octet(2, stream_id)
        .set_octets(3, data)
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

fn map_from_status(status: Status) -> ux::u4 {
    match status {
        Status::Complete => ux::u4::new(0x0),
        Status::Begin => ux::u4::new(0x1),
        Status::Continue => ux::u4::new(0x2),
        Status::End(_) => ux::u4::new(0x3),
    }
}

#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x0, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::IncorrectMessageType(0x0)),
        );
    }

    #[test]
    fn invalid_status() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x5040_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::InvalidStatusBit(0x4)),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x500F_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::DataOutOfRange(0xF)),
        );
    }

    #[test]
    fn expected_stream_id() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x5000_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::ExpectedStreamId),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x5001_BE00, 0x0, 0x0, 0x0]
            })
            .unwrap()
            .stream_id,
            0xBE,
        );
    }

    #[test]
    fn status() {
        let data = [
            (ux::u4::new(0x0), Status::Complete),
            (ux::u4::new(0x1), Status::Begin),
            (ux::u4::new(0x2), Status::Continue),
            (ux::u4::new(0x3), Status::End(true)),
        ];
        for d in data {
            assert_eq!(
                Message::try_from(
                    Packet {
                        data: [0x5001_A000, 0x0, 0x0, 0x0]
                    }
                    .set_nibble(2, d.0)
                ),
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
            Message::try_from(Packet {
                data: [0x503F_A000, 0x0, 0x0, 0x0]
            }),
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
                data: [0x500E_A123, 0x4567_890A, 0xBCDE_F123, 0x4567_890A,]
            }),
            Ok(Message {
                status: Status::Complete,
                stream_id: 0xA1,
                data: vec![
                    0x23, 0x45, 0x67, 0x89, 0x0A, 0xBC, 0xDE, 0xF1, 0x23, 0x45, 0x67, 0x89, 0x0A,
                ],
            }),
        );
        assert_eq!(
            Message::try_from(Packet {
                data: [0x5003_A1FF, 0xFF00_0000, 0x0, 0x0,]
            }),
            Ok(Message {
                status: Status::Complete,
                stream_id: 0xA1,
                data: vec![0xFF, 0xFF],
            }),
        );
    }
}

#[cfg(test)]
mod serialize {
    use super::*;

    #[test]
    fn status() {
        let data = [
            (Status::Complete, ux::u4::new(0x0)),
            (Status::Begin, ux::u4::new(0x1)),
            (Status::Continue, ux::u4::new(0x2)),
            (Status::End(true), ux::u4::new(0x3)),
        ];
        for d in data {
            assert_eq!(
                Packet::from(Message {
                    status: d.0,
                    stream_id: 0x0,
                    data: Vec::new(),
                }),
                Packet {
                    data: [0x5001_0000, 0x0, 0x0, 0x0]
                }
                .set_nibble(2, d.1),
            );
        }
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Complete,
                stream_id: 0x0A,
                data: Vec::new(),
            }),
            Packet {
                data: [0x5001_0A00, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn data() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Complete,
                stream_id: 0x0,
                data: vec![
                    0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,
                ],
            }),
            Packet {
                data: [0x500E_0012, 0x3456_7890, 0xABCD_EF12, 0x3456_7890]
            },
        );
        assert_eq!(
            Packet::from(Message {
                status: Status::Complete,
                stream_id: 0x0,
                data: vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF,],
            }),
            Packet {
                data: [0x5006_00FF, 0xFFFF_FFFF, 0x0, 0x0,]
            },
        );
    }
}

#[cfg(test)]
mod builder {
    use super::*;

    #[test]
    fn no_data() {
        assert_eq!(
            Builder::new().build(),
            vec![Message{ 
                status: Status::Complete,
                stream_id: 0, 
                data: Vec::new(), 
            }],
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Builder::new().stream_id(0xB3).build(),
            vec![Message{ 
                status: Status::Complete,
                stream_id: 0xB3, 
                data: Vec::new(), 
            }],
        );
    }

    #[test]
    fn complete_message() {
        assert_eq!(
            Builder::new()
                .data(std::iter::repeat(0x0).take(10).collect())
                .build(),
            vec![Message{ 
                status: Status::Complete,
                stream_id: 0x0, 
                data: std::iter::repeat(0x0).take(10).collect(),
            }],
        );
    }

    #[test]
    fn full_complete_message() {
        assert_eq!(
            Builder::new()
                .data(std::iter::repeat(0x0).take(13).collect())
                .build(),
            vec![Message{ 
                status: Status::Complete,
                stream_id: 0x0, 
                data: std::iter::repeat(0x0).take(13).collect(),
            }],
        );
    }

    #[test]
    fn continued_message_2_parts() {
        assert_eq!(
            Builder::new()
                .data((0..).take(20).collect())
                .build(),
            vec![
                Message{ 
                    status: Status::Begin,
                    stream_id: 0x0, 
                    data: (0..13).collect(),
                },
                Message{ 
                    status: Status::End(true),
                    stream_id: 0x0, 
                    data: (13..20).collect(),
                },
            ],
        );
    }

    #[test]
    fn continued_message_3_parts() {
        assert_eq!(
            Builder::new()
                .data((0..).take(30).collect())
                .build(),
            vec![
                Message{ 
                    status: Status::Begin,
                    stream_id: 0x0, 
                    data: (0..13).collect(),
                },
                Message{ 
                    status: Status::Continue,
                    stream_id: 0x0, 
                    data: (13..26).collect(),
                },
                Message{ 
                    status: Status::End(true),
                    stream_id: 0x0, 
                    data: (26..30).collect(),
                },
            ],
        );
    }
}
