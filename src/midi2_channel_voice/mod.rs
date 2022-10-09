use crate::{
    error::Error,
    packet::Packet,
};

pub mod note_on;
pub mod note_off;
pub mod key_pressure;

mod attribute;
pub use attribute::Attribute;

const TYPE_CODE: ux::u4 = ux::u4::new(0x4);

fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != TYPE_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}