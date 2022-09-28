use crate::{
    error::Error, 
    packet::Packet,
    message_trait,
};

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
}

impl std::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        Packet::new().set_nibble(1, m.group)
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{ group: p.nibble(1) }),
            Err(e) => Err(e),
        }
    }
}

impl message_trait::Message for Message {
    fn group(&self) -> ux::u4 {
        self.group
    }

    fn set_group(self, group: ux::u4) -> Self {
        Self {
            group,
            ..self
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
            Message::try_from(Packet::from_data(&[0x0700_0000])),
            Ok(Message { group: ux::u4::new(0x7) }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message { group: ux::u4::new(0x2) }),
            Packet::from_data(&[0x0200_0000]),
        );
    }
}
