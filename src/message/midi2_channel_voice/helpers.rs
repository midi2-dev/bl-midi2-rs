use crate::{
    error::Error,
    message::helpers as message_helpers,
    result::Result,
    util::{BitOps, Truncate},
};

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

pub fn note_from_packet(p: &[u32]) -> ux::u7 {
    p[0].octet(2).truncate()
}

pub fn note_velocity_from_packet(p: &[u32]) -> u16 {
    p[1].word(0)
}

pub fn write_controller_bank_to_packet(v: ux::u7, p: &mut [u32]) -> &mut [u32] {
    p[0].set_octet(2, v.into());
    p
}

pub fn write_controller_index_to_packet(v: ux::u7, p: &mut [u32]) -> &mut [u32] {
    p[0].set_octet(3, v.into());
    p
}

pub fn write_controller_data_to_packet(v: u32, p: &mut [u32]) -> &mut [u32] {
    p[1] = v;
    p
}

pub fn write_note_to_packet(note: ux::u7, p: &mut [u32]) -> &mut [u32] {
    p[0].set_octet(2, note.into());
    p
}

pub fn write_note_velocity_to_packet(velocity: u16, p: &mut [u32]) -> &mut [u32] {
    p[1].set_word(0, velocity);
    p
}

pub fn validate_buffer_size(p: &[u32], sz: usize) -> Result<()> {
    if p.len() < sz {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_bank_from_packet() {
        assert_eq!(
            controller_bank_from_packet(&[0x472A_5B3C]),
            ux::u7::new(0x5B)
        );
    }

    #[test]
    fn test_controller_index_from_packet() {
        assert_eq!(
            controller_index_from_packet(&[0x472A_5B3C]),
            ux::u7::new(0x3C)
        );
    }

    #[test]
    fn test_controller_data_from_packet() {
        assert_eq!(
            controller_data_from_packet(&[0x472A_5B3C, 0x7B96D981]),
            0x7B96D981
        );
    }

    #[test]
    fn test_note_from_packet() {
        assert_eq!(note_from_packet(&[0x0000_3200]), ux::u7::new(0x32));
    }

    #[test]
    fn test_note_velocity_from_packet() {
        assert_eq!(
            note_velocity_from_packet(&[0x0000_0000, 0x328A_0000]),
            0x328A
        );
    }

    #[test]
    fn test_write_controller_bank_to_packet() {
        assert_eq!(
            write_controller_bank_to_packet(ux::u7::new(0x3A), &mut [0x0]),
            &[0x0000_3A00]
        );
    }

    #[test]
    fn test_write_controller_index_to_packet() {
        assert_eq!(
            write_controller_index_to_packet(ux::u7::new(0x55), &mut [0x0]),
            &[0x0000_0055]
        );
    }

    #[test]
    fn test_write_controller_data_to_packet() {
        assert_eq!(
            write_controller_data_to_packet(0xE9C40FF4, &mut [0x0, 0x0]),
            &[0x0, 0xE9C40FF4]
        );
    }

    #[test]
    fn test_write_note_to_packet() {
        assert_eq!(
            write_note_to_packet(ux::u7::new(0x73), &mut [0x0]),
            &[0x0000_7300]
        );
    }

    #[test]
    fn test_write_note_velocity_to_packet() {
        assert_eq!(
            write_note_velocity_to_packet(0x1E02, &mut [0x0, 0x0]),
            &[0x0, 0x1E02_0000]
        );
    }

    #[test]
    fn test_valid_buffer_size() {
        assert_eq!(validate_buffer_size(&[], 2), Err(Error::BufferOverflow));
        assert_eq!(validate_buffer_size(&[0x0], 2), Err(Error::BufferOverflow));
        assert_eq!(validate_buffer_size(&[0x0, 0x0], 2), Ok(()));
    }
}
