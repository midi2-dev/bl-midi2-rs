use crate::{helpers::truncate, Packet};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    status: Status,
    data: [ux::u7; 6],
    data_len: usize,
}

impl Message {
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn data(&self) -> &[ux::u7] {
        &self.data[..self.data_len]
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

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            3 => match u8::from(p.nibble(2)) {
                0x0..=0x3 => match u8::from(p.nibble(3)) {
                    0..=5 => {
                        let mut m = Message {
                            status: map_status_bit(p.nibble(2)),
                            data: Default::default(),
                            // see comment: ux missing From usize impls
                            data_len: u8::from(p.nibble(3)).into(),
                        };
                        for (i, val) in p.octets(2, 2 + m.data_len).iter().enumerate() {
                            m.data[i] = truncate(*val);
                        }
                        Ok(m)
                    },
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
        Packet::from_data(&[0x3000_0000])
            .set_nibble(2, truncate(m.status as u8))
            // see comment: ux missing From usize impls
            .set_nibble(3, u8::try_from(m.data_len).unwrap().try_into().unwrap())
            .set_octets(2, m.data.iter().map(|v| v.clone().into()).collect())
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct MessageGroup(Vec<Message>);

impl core::ops::Deref for MessageGroup {
    type Target = [Message];
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl MessageGroup {
    pub fn from_data(data: &[ux::u7]) -> Self {
        let mut ret = MessageGroup(Vec::new());

        for chunk in data.chunks(6) {
            let mut m = Message {
                status: Status::Continue,
                data: Default::default(),
                data_len: chunk.len(),
            };
            m.data[..chunk.len()].clone_from_slice(&chunk);
            ret.0.push(m);
        }

        if ret.len() == 1 {
            ret.0[0].status = Status::Complete;
        } else if ret.len() > 1 {
            let l = ret.len();
            ret.0[0].status = Status::Begin;
            ret.0[l - 1].status = Status::End;
        }

        ret
    }

    pub fn from_messages(messages: &[Message]) -> Result<Self, MessagesDoNotFormGroup> {
        if messages.len() == 0 {
            Ok(MessageGroup(Vec::new()))
        } else if messages.len() == 1 && messages[0].status == Status::Complete {
            Ok(MessageGroup(vec![messages[0].clone()]))
        } else {
            if no_begin(&messages) || no_end(&messages) || incorrect_body(&messages) {
                Err(MessagesDoNotFormGroup)
            } else {
                Ok(MessageGroup(messages.iter().map(|m| m.clone()).collect()))
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct MessagesDoNotFormGroup;

fn no_begin(messages: &[Message]) -> bool {
    messages[0].status != Status::Begin
}

fn no_end(messages: &[Message]) -> bool {
    messages[messages.len() - 1].status != Status::End
}

fn incorrect_body(messages: &[Message]) -> bool {
    let cont = |m: &Message| { m.status == Status::Continue };
    messages.len() > 2 && !messages[1..(messages.len() - 1)].iter().all(cont)
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
            Message::try_from(Packet::from_data(&[0x2000_0000])),
            Err(DeserializeError::IncorrectMessageType(0x2)),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x30A0_0000])),
            Err(DeserializeError::InvalidStatusBit(0xA)),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x3009_0000])),
            Err(DeserializeError::DataOutOfRange(0x9)),
        );
    }

    #[test]
    fn complete_message() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x3003_1234, 
                0x5600_0000,
            ])),
            Ok(Message {
                status: Status::Complete,
                data: [
                    ux::u7::new(0x12), 
                    ux::u7::new(0x34), 
                    ux::u7::new(0x56),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                data_len: 3,
            }),
        );
    }

    #[test]
    fn begin_message() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x3012_5B6D])),
            Ok(Message {
                status: Status::Begin,
                data: [
                    ux::u7::new(0x5B), 
                    ux::u7::new(0x6D),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                data_len: 2,
            }),
        );
    }

    #[test]
    fn continue_status() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x3025_3141, 
                0x1512_6500,
            ])),
            Ok(Message {
                status: Status::Continue,
                data: [
                    ux::u7::new(0x31), 
                    ux::u7::new(0x41), 
                    ux::u7::new(0x15),
                    ux::u7::new(0x12),
                    ux::u7::new(0x65),
                    ux::u7::new(0x0),
                ],
                data_len: 5,
            }),
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x3030_0000])),
            Ok(Message {
                status: Status::End,
                data: Default::default(),
                data_len: 0,
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
                data: [
                    ux::u7::new(0x2B), 
                    ux::u7::new(0x4D),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                data_len: 2,
            }),
            Packet::from_data(&[0x3002_2B4D]),
        );
    }

    #[test]
    fn begin() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Begin,
                data: [
                    ux::u7::new(0x14), 
                    ux::u7::new(0x14), 
                    ux::u7::new(0x21), 
                    ux::u7::new(0x35), 
                    ux::u7::new(0x62), 
                    ux::u7::new(0x37)
                ],
                data_len: 6,
            }),
            Packet::from_data(&[
                0x3016_1414, 
                0x2135_6237,
            ]),
        );
    }

    #[test]
    fn continue_status() {
        assert_eq!(
            Packet::from(Message {
                status: Status::Continue,
                data: [
                    ux::u7::new(0x7F), 
                    ux::u7::new(0x7F), 
                    ux::u7::new(0x7F),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                data_len: 3,
            }),
            Packet::from_data(&[
                0x3023_7F7F, 
                0x7F00_0000,
            ]),
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Packet::from(Message {
                status: Status::End,
                data: Default::default(),
                data_len: 0,
            }),
            Packet::from_data(&[0x3030_0000]),
        );
    }
}

#[cfg(test)]
mod message_group {
    use super::*;

    fn data_vec(begin: u8, end: u8) -> Vec<ux::u7> {
        (begin..end).map(|v| ux::u7::new(v)).collect()
    }

    #[test]
    fn empty_message_group() {
        assert_eq!(
            MessageGroup::from_data(&Vec::<ux::u7>::new()),
            MessageGroup(Vec::new()),
        );
    }

    #[test]
    fn complete_message() {
        assert_eq!(
            MessageGroup::from_data(&data_vec(0, 5)),
            MessageGroup(vec![
                Message {
                    status: Status::Complete,
                    data: [
                        ux::u7::new(0x0),
                        ux::u7::new(0x1),
                        ux::u7::new(0x2),
                        ux::u7::new(0x3),
                        ux::u7::new(0x4),
                        ux::u7::new(0x0),
                    ],
                    data_len: 5,
                },
            ]),
        );
    }

    #[test]
    fn full_complete_message() {
        assert_eq!(
            MessageGroup::from_data(&data_vec(0, 6)),
            MessageGroup(vec![
                Message {
                    status: Status::Complete,
                    data: [
                        ux::u7::new(0x0),
                        ux::u7::new(0x1),
                        ux::u7::new(0x2),
                        ux::u7::new(0x3),
                        ux::u7::new(0x4),
                        ux::u7::new(0x5),
                    ],
                    data_len: 6,
                },
            ]),
        );
    }

    #[test]
    fn continued_message_3_parts() {
        assert_eq!(
            MessageGroup::from_data(&data_vec(0, 15)),
            MessageGroup(vec![
                Message {
                    status: Status::Begin,
                    data: [
                        ux::u7::new(0x0),
                        ux::u7::new(0x1),
                        ux::u7::new(0x2),
                        ux::u7::new(0x3),
                        ux::u7::new(0x4),
                        ux::u7::new(0x5),
                    ],
                    data_len: 6,
                },
                Message {
                    status: Status::Continue,
                    data: [
                        ux::u7::new(0x6),
                        ux::u7::new(0x7),
                        ux::u7::new(0x8),
                        ux::u7::new(0x9),
                        ux::u7::new(0xA),
                        ux::u7::new(0xB),
                    ],
                    data_len: 6,
                },
                Message {
                    status: Status::End,
                    data: [
                        ux::u7::new(0xC),
                        ux::u7::new(0xD),
                        ux::u7::new(0xE),
                        ux::u7::new(0x0),
                        ux::u7::new(0x0),
                        ux::u7::new(0x0),
                    ],
                    data_len: 3,
                },
            ]),
        );
    }
}

// ux missing From usize impls:
//
// the ux crate does note imlement From<usize> on its types :-/
// Should be forthcoming in a future release.
