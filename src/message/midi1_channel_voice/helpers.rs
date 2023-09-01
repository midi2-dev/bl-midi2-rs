use crate::{
    util::{BitOps, Truncate},
    *,
};

pub fn note_velocity_from_packet(p: &[u32]) -> u7 {
    p[0].octet(3).truncate()
}

pub fn write_note_velocity_to_packet(velocity: u7, p: &mut [u32]) -> &mut [u32] {
    p[0].set_octet(3, velocity.into());
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_note_velocity_from_packet() {
        assert_eq!(note_velocity_from_packet(&[0x0000_0062]), u7::new(0x62));
    }

    #[test]
    fn test_write_note_velocity_to_packet() {
        assert_eq!(
            write_note_velocity_to_packet(u7::new(0x1A), &mut [0x0]),
            &[0x0000_001A]
        );
    }
}
