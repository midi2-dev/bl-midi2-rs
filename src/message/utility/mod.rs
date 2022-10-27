use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
};

pub mod time_stamp;
pub mod no_op;

pub fn validate_packet(p: &Packet, op_code: ux::u4) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x0) {
        Err(Error::InvalidData)
    } else if p.nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
