use crate::{
    error::Error,
    util::truncate, 
    packet::Packet,
};

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    program: ux::u7,
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{
                group: p.nibble(1),
                channel: p.nibble(3),
                program: truncate(p.octet(2)),
            }),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    match super::validate_packet(p) {
        Ok(_) => {
            if p.nibble(2) != ux::u4::new(0b1100) {
                Err(Error::InvalidData)
            } else {
                Ok(())
            }
        },
        Err(e) => Err(e),
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        Packet::new()
            .set_nibble(0, ux::u4::new(0x2))
            .set_nibble(1, m.group)
            .set_nibble(2, ux::u4::new(0b1100))
            .set_nibble(3, m.channel)
            .set_octet(2, m.program.into())
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;
    
    message_traits_test!(Message);

    #[test]
    fn wrong_status() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x2000_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x27C0_6600])),
            Ok(Message{
                group: ux::u4::new(0x7),
                channel: ux::u4::new(0),
                program: ux::u7::new(0x66),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0x8),
                program: ux::u7::new(0x04),
            }),
            Packet::from_data(&[0x20C8_0400]),
        );
    }
}
