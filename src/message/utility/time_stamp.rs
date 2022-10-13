use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet, 
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    time_stamp: ux::u20,
    group: ux::u4,
}

impl std::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        Packet::from_data(
            &[u32::from(m.time_stamp) |  0x0020_0000],
        )
            .set_nibble(1, m.group)
            .to_owned()
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message {
                time_stamp: truncate(p[0]),
                group: p.nibble(1),
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
            Message::try_from(Packet::from_data(&[0x0A22_ABCD])),
            Ok(Message {
                time_stamp: ux::u20::new(0x2ABCD),
                group: ux::u4::new(0xA),
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                time_stamp: ux::u20::new(0x2ABCD),
                group: ux::u4::new(0xB),
            }),
            Packet::from_data(&[0x0B22_ABCD]),
        );
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            Message { 
                time_stamp: ux::u20::new(0x314),
                group: Default::default(),
            }.time_stamp,
            ux::u20::new(0x314),
        );
    }
}
