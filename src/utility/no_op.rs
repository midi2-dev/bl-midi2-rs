use crate::{
    error::Error, 
    packet::Packet
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message;

impl std::convert::From<Message> for Packet {
    fn from(_m: Message) -> Self {
        Default::default()
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{}),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    match super::validate_packet(&p) {
        Ok(_) => {
            if p.nibble(2) != ux::u4::new(0) {
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
            Message::try_from(<Packet as Default>::default()),
            Ok(Message),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message),
            <Packet as Default>::default(),
        );
    }
}
