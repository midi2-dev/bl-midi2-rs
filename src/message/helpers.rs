use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
    util::Truncate,
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
    super::write_group_to_packet(group, p);
    p
        .set_nibble(2, op_code)
        .set_nibble(3, channel);
}

pub fn group_from_packet(p: &Packet) -> ux::u4 {
    p.nibble(1)
}

pub fn channel_from_packet(p: &Packet) -> ux::u4 {
    p.nibble(3)
}

pub fn concatenate(lsb: ux::u7, msb: ux::u7) -> ux::u14 {
    (ux::u14::from(msb) << 7) | ux::u14::from(lsb)
}

pub fn most_significant_bit(word_14: ux::u14) -> ux::u7 {
    (word_14 >> 7).truncate()
}

pub fn least_significant_bit(word_14: ux::u14) -> ux::u7 {
    (word_14 & ux::u14::new(0b0000000_0111111)).truncate()
}