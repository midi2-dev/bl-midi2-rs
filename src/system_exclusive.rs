use crate::{std_error_impl, helpers::mask, Packet};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    status: Status,
    data: Vec<ux::u7>,
}

impl Message {
    pub fn from_data(data: Vec<ux::u7>) -> Vec<Self> {
        if data.len() <= 6 {
            vec![
                Message {
                    status: Status::Complete,
                    data,
                }
            ]
        } else {
            let mut ret = Vec::new();

            for chunk in data.chunks(6) {
                ret.push(Message{
                    status: Status::Continue,
                    data: chunk
                        .iter()
                        .map(|v| v.clone())
                        .collect(),
                });
            }

            let l = ret.len();
            ret[0].status = Status::Begin;
            ret[l - 1].status = Status::End;

            ret
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn data(&self) -> &Vec<ux::u7> {
        &self.data
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Status {
    Complete = 0x0,
    Begin = 0x1,
    Continue = 0x2,
    End = 0x3,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    InvalidStatusBit(u8),
    DataOutOfRange(u8),
    IncorrectMessageType(u8),
}
std_error_impl!(DeserializeError); 

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            3 => match u8::from(p.nibble(2)) {
                0x0..=0x3 => match u8::from(p.nibble(3)) {
                    0..=5 => Ok(Message {
                        status: map_status_bit(p.nibble(2)),
                        // see comment: ux missing From usize impls
                        data: p.octets(2, (2 + u8::from(p.nibble(3))).into())
                            .iter()
                            .map(|v| mask(v.clone()))
                            .collect(),
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
        Packet {
            data: [0x3000_0000, 0x0, 0x0, 0x0],
        }
        .set_nibble(2, mask(m.status as u8))
        // see comment: ux missing From usize impls
        .set_nibble(3, u8::try_from(m.data.len()).unwrap().try_into().unwrap())
        .set_octets(2, m.data.iter().map(|v| v.clone().into()).collect())
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
            Message::try_from(Packet {
                data: [0x2000_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::IncorrectMessageType(0x2)),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x30A0_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::InvalidStatusBit(0xA)),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x3009_0000, 0x0, 0x0, 0x0]
            }),
            Err(DeserializeError::DataOutOfRange(0x9)),
        );
    }

    #[test]
    fn complete_message() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x3003_1234, 0x5600_0000, 0x0, 0x0]
            }),
            Ok(Message {
                status: Status::Complete,
                data: vec![ux::u7::new(0x12), ux::u7::new(0x34), ux::u7::new(0x56)],
            }),
        );
    }

    #[test]
    fn begin_message() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x3012_5B6D, 0x0, 0x0, 0x0]
            }),
            Ok(Message {
                status: Status::Begin,
                data: vec![ux::u7::new(0x5B), ux::u7::new(0x6D)],
            }),
        );
    }

    #[test]
    fn continue_status() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x3025_3141, 0x1512_6500, 0x0, 0x0]
            }),
            Ok(Message {
                status: Status::Continue,
                data: vec![
                    ux::u7::new(0x31), 
                    ux::u7::new(0x41), 
                    ux::u7::new(0x15),
                    ux::u7::new(0x12),
                    ux::u7::new(0x65)
                ],
            }),
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Message::try_from(Packet {
                data: [0x3030_0000, 0x0, 0x0, 0x0]
            }),
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
                data: vec![ux::u7::new(0x2B), ux::u7::new(0x4D)],
            }),
            Packet {
                data: [0x3002_2B4D, 0x0, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn begin() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Begin,
                data: vec![
                    ux::u7::new(0x14), 
                    ux::u7::new(0x14), 
                    ux::u7::new(0x21), 
                    ux::u7::new(0x35), 
                    ux::u7::new(0x62), 
                    ux::u7::new(0x37)
                ],
            }),
            Packet {
                data: [0x3016_1414, 0x2135_6237, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn continue_status() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Continue,
                data: vec![ux::u7::new(0x7F), ux::u7::new(0x7F), ux::u7::new(0x7F)]
            }),
            Packet {
                data: [0x3023_7F7F, 0x7F00_0000, 0x0, 0x0]
            },
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Packet::from(Message {
                status: Status::End,
                data: vec![],
            }),
            Packet {
                data: [0x3030_0000, 0x0, 0x0, 0x0]
            },
        );
    }
}

#[cfg(test)]
mod from_data {
    use super::*;

    #[test]
    fn empty_message() {
        assert_eq!(
            Message::from_data(Vec::new()),
            vec![
                Message {
                    status: Status::Complete,
                    data: Vec::new(),
                },
            ],
        );
    }

    #[test]
    fn complete_message() {
        assert_eq!(
            Message::from_data((0..5).map(|v| ux::u7::new(v.clone())).collect()),
            vec![
                Message {
                    status: Status::Complete,
                    data: (0..5).map(|v| ux::u7::new(v.clone())).collect(),
                },
            ],
        );
    }

    #[test]
    fn full_complete_message() {
        assert_eq!(
            Message::from_data((0..6).map(|v| ux::u7::new(v.clone())).collect()),
            vec![
                Message {
                    status: Status::Complete,
                    data: (0..6).map(|v| ux::u7::new(v.clone())).collect(),
                },
            ],
        );
    }

    #[test]
    fn continued_message_2_parts() {
        assert_eq!(
            Message::from_data((0..10).map(|v| ux::u7::new(v.clone())).collect()),
            vec![
                Message {
                    status: Status::Begin,
                    data: (0..6).map(|v| ux::u7::new(v.clone())).collect(),
                },
                Message {
                    status: Status::End,
                    data: (6..10).map(|v| ux::u7::new(v.clone())).collect(),
                },
            ],
        );
    }

    #[test]
    fn continued_message_3_parts() {
        assert_eq!(
            Message::from_data((0..15).map(|v| ux::u7::new(v.clone())).collect()),
            vec![
                Message {
                    status: Status::Begin,
                    data: (0..6).map(|v| ux::u7::new(v.clone())).collect(),
                },
                Message {
                    status: Status::Continue,
                    data: (6..12).map(|v| ux::u7::new(v.clone())).collect(),
                },
                Message {
                    status: Status::End,
                    data: (12..15).map(|v| ux::u7::new(v.clone())).collect(),
                },
            ],
        );
    }
}

// ux missing From usize impls:
//
// the ux crate does note imlement From<usize> on its types :-/
// Should be forthcoming in a future release.
