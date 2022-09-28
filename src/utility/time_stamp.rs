use crate::{
    helpers::mask, 
    packet::Packet, 
    error::Error,
};

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
)]
pub struct Message {
    time_stamp: ux::u20,
}

impl Message {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn time_stamp(&self) -> ux::u20 {
        self.time_stamp
    }

    pub fn set_time_stamp(self, time_stamp: ux::u20) -> Self {
        Self {
            time_stamp
        }
    }
}

impl std::convert::From<Message> for Packet {
    fn from(Message { time_stamp } : Message) -> Self {
        Packet::from_data(
            &[u32::from(time_stamp) | 0x0020_0000],
        )
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message {
                time_stamp: mask(p[0]),
            }),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error>  {
    match super::validate_packet(&p) {
        Ok(()) => {
            if p.nibble(2) != ux::u4::new(0x1) && p.nibble(2) != ux::u4::new(0x2) {
                Err(Error::InvalidData)
            } else {
                Ok(())
            }
        },
        err => err,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x0022_ABCD])),
            Ok(Message {
                time_stamp: ux::u20::new(0x2ABCD)
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                time_stamp: ux::u20::new(0x2ABCD)
            }),
            Packet::from_data(&[0x0022_ABCD]),
        );
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            Message { time_stamp: ux::u20::new(0x314) }.time_stamp(),
            ux::u20::new(0x314),
        );
    }

    #[test]
    fn set_time_stamp() {
        assert_eq!(
            Message::new().set_time_stamp(ux::u20::new(42)),
            Message { time_stamp: ux::u20::new(42) },
        );
    }
}
