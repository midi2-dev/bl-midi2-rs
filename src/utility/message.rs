use crate::{
    error::Error,
    packet::Packet,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Message {
    NoOp(super::no_op::Message),
    TimeStamp(super::time_stamp::Message),
}

impl std::convert::From<Message> for Packet {
    fn from(m: Message) -> Packet {
        match m {
            Message::NoOp(no) => no.into(),
            Message::TimeStamp(ts) => ts.into(),
        }
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(2)) {
            0x0 => match super::no_op::Message::try_from(p) {
                Ok(m) => Ok(Message::NoOp(m)),
                Err(e) => Err(e),
            },
            0x1 => match super::time_stamp::Message::try_from(p) {
                Ok(m) => Ok(Message::TimeStamp(m)),
                Err(e) => Err(e),
            },
            _ => Err(Error::InvalidData),
        }
    }
}

pub fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x0) {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
