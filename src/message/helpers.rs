use crate::{
    error::Error,
    util::{BitOps, Truncate},
};

pub fn validate_packet(p: &[u32], type_code: ux::u4, op_code: ux::u4) -> Result<(), Error> {
    if p.is_empty() || p[0].nibble(0) != type_code || p[0].nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn write_type_to_packet(t: ux::u4, p: &mut [u32]) {
    p[0].set_nibble(0, t);
}

pub fn write_group_to_packet(g: ux::u4, p: &mut [u32]) {
    p[0].set_nibble(1, g);
}

pub fn write_data(
    type_code: ux::u4,
    group: ux::u4,
    op_code: ux::u4,
    channel: ux::u4,
    p: &mut [u32],
) {
    write_type_to_packet(type_code, p);
    write_group_to_packet(group, p);
    p[0].set_nibble(2, op_code).set_nibble(3, channel);
}

pub fn group_from_packet(p: &[u32]) -> ux::u4 {
    p[0].nibble(1)
}

pub fn channel_from_packet(p: &[u32]) -> ux::u4 {
    p[0].nibble(3)
}

pub fn concatenate(lsb: ux::u7, msb: ux::u7) -> ux::u14 {
    (ux::u14::from(msb) << 7) | ux::u14::from(lsb)
}

pub fn most_significant_bit(word_14: ux::u14) -> ux::u7 {
    (word_14 >> 7).truncate()
}

pub fn least_significant_bit(word_14: ux::u14) -> ux::u7 {
    (word_14 & ux::u14::new(0b00_0000_0011_1111)).truncate()
}
