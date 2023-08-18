use crate::{error::Error, message::helpers as message_helpers, util::{BitOps, Truncate}, result::Result};

pub fn validate_packet(p: &[u32], type_code: ux::u4, op_code: ux::u4) -> Result<()> {
    if p.len() < 2 {
        Err(Error::BufferOverflow)
    } else {
        message_helpers::validate_packet(p, type_code, op_code)
    }
}

pub fn controller_bank_from_packet(p: &[u32]) -> ux::u7 {
    p[0].octet(2).truncate()
}

pub fn controller_index_from_packet(p: &[u32]) -> ux::u7 {
    p[0].octet(3).truncate()
}

pub fn controller_data_from_packet(p: &[u32]) -> u32 {
    p[1]
}

pub fn write_controller_bank_to_packet(v: ux::u7, p: &mut [u32]) -> &mut [u32] {
    p[0].set_octet(2, v.into());
    p
}

pub fn write_controller_index_to_packet(v: ux::u7, p: &mut [u32]) -> &mut [u32]{
    p[0].set_octet(3, v.into());
    p
}

pub fn write_controller_data_to_packet(v: u32, p: &mut [u32]) -> &mut [u32]{
    p[1] = v;
    p
}

pub fn validate_controller_message_buffer_size(p: &[u32]) -> Result<()> {
    match p.len() {
        0 | 1 => Err(Error::BufferOverflow),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_bank_from_packet() {
        assert_eq!(controller_bank_from_packet(&[0x472A_5B3C]), ux::u7::new(0x5B));
    }

    #[test]
    fn test_controller_index_from_packet() {
        assert_eq!(controller_index_from_packet(&[0x472A_5B3C]), ux::u7::new(0x3C));
    }

    #[test]
    fn test_controller_data_from_packet() {
        assert_eq!(controller_data_from_packet(&[0x472A_5B3C, 0x7B96D981]), 0x7B96D981);
    }

    #[test]
    fn test_write_controller_bank_to_packet() {
        assert_eq!(write_controller_bank_to_packet(ux::u7::new(0x3A), &mut [0x0]), &[0x0000_3A00]);
    }

    #[test]
    fn test_write_controller_index_to_packet() {
        assert_eq!(write_controller_index_to_packet(ux::u7::new(0x55), &mut [0x0]), &[0x0000_0055]);
    }

    #[test]
    fn test_write_controller_data_to_packet() {
        assert_eq!(write_controller_data_to_packet(0xE9C40FF4, &mut [0x0, 0x0]), &[0x0, 0xE9C40FF4]);
    }

    #[test]
    fn test_valid_controller_message_buffer_size() {
        assert_eq!(validate_controller_message_buffer_size(&[]), Err(Error::BufferOverflow));
        assert_eq!(validate_controller_message_buffer_size(&[0x0]), Err(Error::BufferOverflow));
        assert_eq!(validate_controller_message_buffer_size(&[0x0, 0x0]), Ok(()));
    }
}
