use crate::{
    error::Error,
    util::truncate, 
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
        super::validate_packet(&p, Message::<OP>::OP_CODE)?;
        Ok(Message{
            group: super::group_from_packet(&p),
            channel: super::channel_from_packet(&p),
            note: truncate(p.octet(2)),
            velocity: p.word(2),
            attribute: attribute::from_packet(&p)?,
        })
    }
}

impl<const OP: u8> From<Message<OP>> for Packet {
    fn from(m: Message<OP>) -> Self {
        let mut p = Packet::new();
        super::write_data_to_packet(m.group, Message::<OP>::OP_CODE, m.channel, &mut p);
        p
            .set_octet(2, m.note.into())
            .set_word(2, m.velocity);
        attribute::write_attribute(&mut p, m.attribute);
        p
    }
}