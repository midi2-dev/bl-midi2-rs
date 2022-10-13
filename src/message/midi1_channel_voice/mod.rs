use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
};

pub mod channel_pressure;
pub mod control_change;
pub mod key_pressure;
pub mod note_off;
pub mod note_on;
pub mod pitch_bend;
pub mod program_change;

fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x2) {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
