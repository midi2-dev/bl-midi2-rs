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
        super::validate_packet(&p, Message::OP_CODE)?;
        Ok(Message {
            group: super::group_from_packet(&p),
            channel: super::channel_from_packet(&p),
            note: truncate(p.octet(2)),
            data: p[1],
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        super::write_data_to_packet(m.group, Message::OP_CODE, m.channel, &mut p);
        p.set_octet(2, m.note.into());
        p[1] = m.data;
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
    }

    #[test]
    fn serialize() {
    }
}