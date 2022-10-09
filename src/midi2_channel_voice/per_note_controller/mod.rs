use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet,
};
use super::attribute;

pub mod note_on;
pub mod note_off;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message<const OP: u8> {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    velocity: u16,
    attribute: Option<attribute::Attribute>,
}

impl<const OP: u8> Message<OP> {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(OP);
}

impl<const OP: u8> std::convert::TryFrom<Packet> for Message<OP> {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        validate_packet::<OP>(&p)?;
        Ok(Message{
            group: p.nibble(1),
            channel: p.nibble(3),
            note: truncate(p.octet(2)),
            velocity: p.word(2),
            attribute: attribute::from_packet(&p)?,
        })
    }
}

fn validate_packet<const OP: u8>(p: &Packet) -> Result<(), Error> {
    super::validate_packet(p)?;
    if p.nibble(2) != Message::<OP>::OP_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

impl<const OP: u8> From<Message<OP>> for Packet {
    fn from(m: Message<OP>) -> Self {
        let mut p = Packet::new()
            .set_nibble(0, Message::<OP>::TYPE_CODE)
            .set_nibble(1, m.group)
            .set_nibble(2, Message::<OP>::OP_CODE)
            .set_nibble(3, m.channel)
            .set_octet(2, m.note.into())
            .set_word(2, m.velocity)
            .to_owned();
        attribute::write_attribute(&mut p, m.attribute);
        p
    }
}