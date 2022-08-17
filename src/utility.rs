use crate::{
    helpers::mask,
    Packet,
};

#[derive(
    Debug,
    PartialEq,
)]
pub enum Message {
    NoOp,
    TimeStamp {
        time_stamp: ux::u20,
    },
}

impl std::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        match m {
            Message::NoOp => Packet {
                data: [0x0, 0x0, 0x0, 0x0],
            },
            Message::TimeStamp{ time_stamp } => Packet {
                data: [
                    u32::from(time_stamp) | 0x0020_0000,
                    0x0, 0x0, 0x0
                ],
            }
        }
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            0 => match u8::from(p.nibble(2)) {
                0 => Ok(Message::NoOp),
                2 => Ok(Message::TimeStamp{time_stamp: mask(p.data[0])}),
                s => Err(DeserializeError::InvalidStatusBit(s))
            },
            t => Err(DeserializeError::IncorrectMessageType(t))
        }
    }
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    InvalidStatusBit(u8),
    IncorrectMessageType(u8),
}

#[cfg(test)]
mod deserialize {
    use super::*;
    
    #[test]
    fn invalid_type() {
        assert_eq!(
            Message::try_from(Packet{data:[0x1000_0000, 0x0, 0x0, 0x0]}),
            Err(DeserializeError::IncorrectMessageType(1)),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from(Packet{data:[0x0030_0000, 0x0, 0x0, 0x0]}),
            Err(DeserializeError::InvalidStatusBit(3)),
        );
    }

    #[test]
    fn noop() {
        assert_eq!(
            Message::try_from(Packet{data:[0x0,0x0,0x0,0x0]}),
            Ok(Message::NoOp),
        );
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            Message::try_from(Packet{data:[0x0022_ABCD,0x0,0x0,0x0]}),
            Ok(Message::TimeStamp{ time_stamp: ux::u20::new(0x2ABCD) }),
        );
    }
}

#[cfg(test)]
mod serialize {
    use super::*;

    #[test]
    fn noop() {
        assert_eq!(
            Packet::from(Message::NoOp),
            Packet{data:[0x0,0x0,0x0,0x0]},
        );
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            Packet::from(Message::TimeStamp{ time_stamp: ux::u20::new(0x2ABCD) }),
            Packet{data:[0x0022_ABCD,0x0,0x0,0x0]},
        );
    }
}
