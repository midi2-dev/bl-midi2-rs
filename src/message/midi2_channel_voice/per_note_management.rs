use crate::{
    error::Error,
    packet::Packet,
    util::Numeric,
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
    detach: bool,
    reset: bool,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(0b1111);
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(&p, Message::OP_CODE)?;
        Ok(Message {
            group: super::group_from_packet(&p),
            channel: super::channel_from_packet(&p),
            note: p.octet(2).truncate(),
            detach: 0b0000_0010 & p.octet(3) == 0b0000_0010,
            reset: 0b0000_0001 & p.octet(3) == 0b0000_0001,
        })
    }
}

impl core::convert::From<Message> for Packet {
    fn from(m: Message) -> Packet {
        let mut p = Packet::new();
        super::write_data_to_packet(m.group, Message::OP_CODE, m.channel, &mut p);
        p.set_octet(2, m.note.into());
        let mut flags = 0x0_u8;
        if m.detach {
            flags |= 0b0000_0010;
        }
        if m.reset {
            flags |= 0b0000_0001;
        }
        p.set_octet(3, flags);
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;
    
    message_traits_test!(Message);
    
    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x41FF_4003])),
            Ok(Message {
                group: ux::u4::new(0x1),
                channel: ux::u4::new(0xF),
                note: ux::u7::new(0x40),
                detach: true,
                reset: true,
            }),
        );
    }
    
    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x6),
                channel: ux::u4::new(0x6),
                note: ux::u7::new(0x4F),
                detach: true,
                reset: true,
            }),
            Packet::from_data(&[0x46F6_4F03]),
        );
    }

}