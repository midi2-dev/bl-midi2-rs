use crate::{
    error::Error,
    packet::Packet,
};

pub mod note_on;
pub mod note_off;

mod attribute;
pub use attribute::Attribute;

fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x4) {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}