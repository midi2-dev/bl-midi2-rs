use crate::{
    error::Error,
    packet::Packet,
};

pub mod time_stamp;
pub mod no_op;

pub fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x0) {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
