use crate::{
    error::Error,
    util::Truncate,
    packet::{Packet, PacketMethods},
};
use super::super::channel_voice_helpers;

pub mod key_pressure;
pub mod pitch_bend;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message<const OP: u8> {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    data: u32,
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
        Ok(Message::<OP> {
            group: channel_voice_helpers::group_from_packet(&p),
            channel: channel_voice_helpers::channel_from_packet(&p),
            note: p.octet(2).truncate(),
            data: p[1],
        })
    }
}

impl<const OP: u8> From<Message::<OP>> for Packet {
    fn from(m: Message<OP>) -> Self {
        let mut p = Packet::new();
        channel_voice_helpers::write_data_to_packet(
            Message::<OP>::TYPE_CODE, 
            m.group, 
            Message::<OP>::OP_CODE, 
            m.channel, 
            &mut p
        );
        p.set_octet(2, m.note.into());
        p[1] = m.data;
        p
    }
}