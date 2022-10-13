use crate::{
    error::Error,
    util::Truncate, 
    packet::{Packet, PacketMethods},
};
use super::super::channel_voice_helpers;

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
    velocity: ux::u7,
}

impl<const OP: u8> Message<OP> {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(OP);
}

impl<const OP: u8> std::convert::TryFrom<Packet> for Message<OP> {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        channel_voice_helpers::validate_packet(
            &p,
            Message::<OP>::TYPE_CODE,
            Message::<OP>::OP_CODE,
        )?;
        Ok(Message{
            group: p.nibble(1),
            channel: p.nibble(3),
            note: p.octet(2).truncate(),
            velocity: p.octet(3).truncate(),
        })
    }
}

impl<const OP: u8> From<Message<OP>> for Packet {
    fn from(m: Message<OP>) -> Self {
        let mut p = Packet::new();
        channel_voice_helpers::write_data_to_packet(
            Message::<OP>::TYPE_CODE,
            m.group,
            Message::<OP>::OP_CODE,
            m.channel,
            &mut p,
        );
        p
            .set_octet(2, m.note.into())
            .set_octet(3, m.velocity.into())
            .to_owned()
    }
}