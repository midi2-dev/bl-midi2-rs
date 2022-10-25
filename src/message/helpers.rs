use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
};

pub fn validate_packet(
    p: &Packet, 
    type_code: ux::u4,
    op_code: ux::u4,
) -> Result<(), Error> {
    if p.nibble(0) != type_code {
        Err(Error::InvalidData)
    } else if p.nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn write_data_to_packet(
    type_code: ux::u4,
    group: ux::u4,
    op_code: ux::u4,
    channel: ux::u4,
    p: &mut Packet,
) {
    super::write_type_to_packet(type_code, p);
    p
        .set_nibble(1, group)
        .set_nibble(2, op_code)
        .set_nibble(3, channel);
}

pub fn group_from_packet(p: &Packet) -> ux::u4 {
    p.nibble(1)
}

pub fn channel_from_packet(p: &Packet) -> ux::u4 {
    p.nibble(3)
}