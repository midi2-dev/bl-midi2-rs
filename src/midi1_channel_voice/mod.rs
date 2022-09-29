use crate::{
    error::Error,
    packet::Packet,
};

pub mod note_off;
pub mod note_on;

fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x2) {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
