use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet,
};
use builder_derive::Builder;
use getters_derive::Getters;

#[derive(
    Clone,
    Debug, 
    PartialEq,
    Builder,
    Getters,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    bend: ux::u14,
}

impl Message {
    pub fn bend_lsb(&self) -> ux::u7 {
        truncate(self.bend)
    }

    pub fn bend_msb(&self) -> ux::u7 {
        truncate(self.bend >> 7)
    }
}

impl Builder {
    pub fn bend_lsb(&mut self, bend_lsb: ux::u7) -> &mut Self {
        match &mut self.bend {
            Some(b) => {
                *b &= ux::u14::new(0b1111111_0000000);
                *b |= ux::u14::from(bend_lsb);
            },
            None => {
                self.bend = Some(ux::u14::from(bend_lsb));
            },
        }
        self
    }

    pub fn bend_msb(&mut self, bend_msb: ux::u7) -> &mut Self {
        match &mut self.bend {
            Some(b) => {
                *b &= ux::u14::new(0b0000000_1111111);
                *b |= ux::u14::from(bend_msb << 7);
            },
            None => {
                self.bend = Some(ux::u14::from(bend_msb << 7));
            },
        }
        self
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{
                group: p.nibble(1),
                channel: p.nibble(3),
                bend: (ux::u14::from(p.octet(3)) << 7) | ux::u14::from(p.octet(2)),
            }),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    match super::validate_packet(p) {
        Ok(_) => {
            if p.nibble(2) != ux::u4::new(0b1110) {
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
            .set_nibble(2, ux::u4::new(0b1110))
            .set_nibble(3, m.channel)
            .set_octet(2, truncate(m.bend & ux::u14::new(0b0000000_0111111)))
            .set_octet(3, truncate(m.bend >> 7))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Message::try_from(Packet::from_data(&[0b0010_1011_1110_0000_01101001_00110011])),
            Ok(Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0),
                bend: ux::u14::new(0b0110011_1101001),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x0),
                bend: ux::u14::new(0b0011011_0111001),
            }),
            Packet::from_data(&[0b0010_0101_1110_0000_00111001_00011011]),
        );
    }
}
