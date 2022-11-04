use crate::{error::Error, message::helpers as message_helpers};

pub fn validate_packet(p: &[u32], type_code: ux::u4, op_code: ux::u4) -> Result<(), Error> {
    if p.len() < 2 {
        Err(Error::BufferOverflow)
    } else {
        message_helpers::validate_packet(p, type_code, op_code)
    }
}
