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
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    data: u32,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(0b1010);
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(&p)?;
        if p.nibble(2) == Message::OP_CODE {
            Ok(Message {
                group: p.nibble(1),
                channel: p.nibble(3),
                note: truncate(p.octet(2)),
                data: p[1],
            })
        } else {
            Err(Error::InvalidData)
        }
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new()
            .set_nibble(0, Message::TYPE_CODE)
            .set_nibble(1, m.group)
            .set_nibble(2, Message::OP_CODE)
            .set_nibble(3, m.channel)
            .set_octet(2, m.note.into())
            .to_owned();
        p[1] = m.data;
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_status() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x4090_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4CA5_3A00, 
                0xABCD_EF01,
            ])),
            Ok(Message {
                group: ux::u4::new(0xC),
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x3A),
                data: 0xABCD_EF01,
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0xF),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x38),
                data: 0x2468_1012,
            }),
            Packet::from_data(&[
                0x4FA2_3800,
                0x2468_1012,
            ])
        )
    }
}