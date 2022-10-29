use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
};

pub mod no_op;
pub mod time_stamp;

pub use no_op::Message as NoOpMessage;
pub use no_op::Builder as NoOpBuilder;
pub use time_stamp::Message as TimeStampMessage;
pub use time_stamp::Builder as TimeStampBuilder;

pub fn validate_packet(p: &Packet, op_code: ux::u4) -> Result<(), Error> {
    if p.nibble(0) != ux::u4::new(0x0) || p.nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
