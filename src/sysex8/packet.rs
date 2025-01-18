use crate::{error, sysex8};

#[derive(Eq, PartialEq, Clone, midi2_proc::Debug)]
pub struct Packet(pub(crate) [u32; 4]);

impl crate::traits::BufferAccess<[u32; 4]> for Packet {
    fn buffer_access(&self) -> &[u32; 4] {
        &self.0
    }
    fn buffer_access_mut(&mut self) -> &mut [u32; 4]
    where
        [u32; 4]: crate::buffer::BufferMut,
    {
        &mut self.0
    }
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Packet {
    type Error = error::InvalidData;
    fn try_from(data: &'a [u32]) -> Result<Self, Self::Error> {
        if data.len() < 4 {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT,
            ));
        }

        use crate::detail::BitOps;
        if u8::from(data[0].nibble(0)) != sysex8::UMP_MESSAGE_TYPE {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE,
            ));
        }

        status_from_data(data)?;

        Ok(Packet({
            let mut buffer = [0x0; 4];
            let sz = 4.min(data.len());
            buffer[..sz].copy_from_slice(&data[..sz]);
            buffer
        }))
    }
}

impl core::ops::Deref for Packet {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl crate::Grouped<[u32; 4]> for Packet {
    fn group(&self) -> crate::ux::u4 {
        use crate::detail::BitOps;
        self.0[0].nibble(1)
    }
    fn set_group(&mut self, group: crate::ux::u4)
    where
        [u32; 4]: crate::buffer::BufferMut,
    {
        use crate::detail::BitOps;
        self.0[0].set_nibble(1, group);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PreviousDataValidity {
    Valid,
    Invalid,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Complete,
    Start,
    Continue,
    End,
    UnexpectedEnd(PreviousDataValidity),
}

fn status_from_data(data: &[u32]) -> Result<Status, error::InvalidData> {
    use crate::detail::BitOps;
    use PreviousDataValidity::*;
    use Status::*;
    match u8::from(data[0].nibble(2)) {
        0x0 => Ok(Complete),
        0x1 => Ok(Start),
        0x2 => Ok(Continue),
        0x3 => {
            if data[0].octet(3) == 0 && data[1..4].iter().all(|b| *b == 0x0) {
                // unexpected end
                match u8::from(data[0].nibble(3)) {
                    0x1 => Ok(UnexpectedEnd(Valid)),
                    0xF => Ok(UnexpectedEnd(Invalid)),
                    _ => Ok(End),
                }
            } else {
                Ok(End)
            }
        }
        _ => Err(error::InvalidData("Invalid SysEx8 status byte")),
    }
}

impl Packet {
    pub fn status(&self) -> Status {
        status_from_data(&self.0[..]).unwrap()
    }
    pub fn stream_id(&self) -> u8 {
        sysex8::stream_id_from_packet(&self.0[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        assert!(
            Packet::try_from(&[0x5001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..]).is_ok()
        );
    }

    #[test]
    fn construction_long_slice() {
        assert!(
            Packet::try_from(&[0x5001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0][..])
                .is_ok()
        );
    }

    #[test]
    fn construction_short_slice() {
        assert_eq!(
            Packet::try_from(&[0x5001_0000, 0x0000_0000, 0x0000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT
            )),
        );
    }

    #[test]
    fn construction_incorrect_ump_type() {
        assert_eq!(
            Packet::try_from(&[0x0001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            )),
        );
    }

    #[test]
    fn complete() {
        assert_eq!(
            Packet::try_from(&[0x5001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .status(),
            Status::Complete,
        );
    }

    #[test]
    fn start() {
        assert_eq!(
            Packet::try_from(&[0x5011_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .status(),
            Status::Start,
        );
    }

    #[test]
    fn cont() {
        assert_eq!(
            Packet::try_from(&[0x5021_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .status(),
            Status::Continue,
        );
    }

    #[test]
    fn end() {
        assert_eq!(
            Packet::try_from(&[0x5032_0001, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .status(),
            Status::End,
        );
    }

    #[test]
    fn unexpected_end_valid_data() {
        assert_eq!(
            Packet::try_from(&[0x5031_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .status(),
            Status::UnexpectedEnd(PreviousDataValidity::Valid),
        );
    }

    #[test]
    fn unexpected_end_invalid_data() {
        assert_eq!(
            Packet::try_from(&[0x503F_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .status(),
            Status::UnexpectedEnd(PreviousDataValidity::Invalid),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Packet::try_from(&[0x5001_0100, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .stream_id(),
            0x1,
        );
    }

    #[test]
    fn group() {
        use crate::Grouped;

        assert_eq!(
            Packet::try_from(&[0x5A01_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,][..])
                .unwrap()
                .group(),
            crate::num::u4::new(0xA),
        );
    }
}
