use crate::{error, sysex7};

#[derive(Eq, PartialEq, Clone, midi2_proc::Debug)]
pub struct Packet(pub(crate) [u32; 2]);

impl crate::traits::BufferAccess<[u32; 2]> for Packet {
    fn buffer_access(&self) -> &[u32; 2] {
        &self.0
    }
    fn buffer_access_mut(&mut self) -> &mut [u32; 2]
    where
        [u32; 2]: crate::buffer::BufferMut,
    {
        &mut self.0
    }
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Packet {
    type Error = error::InvalidData;
    fn try_from(data: &'a [u32]) -> Result<Self, Self::Error> {
        if data.len() < 2 {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT,
            ));
        }

        use crate::detail::BitOps;
        if u8::from(data[0].nibble(0)) != sysex7::UMP_MESSAGE_TYPE {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE,
            ));
        }

        status_from_data(data)?;

        Ok(Packet({
            let mut buffer = [0x0; 2];
            let sz = 2.min(data.len());
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

impl crate::Grouped<[u32; 2]> for Packet {
    fn group(&self) -> crate::ux::u4 {
        use crate::detail::BitOps;
        self.0[0].nibble(1)
    }
    fn set_group(&mut self, group: crate::ux::u4)
    where
        [u32; 2]: crate::buffer::BufferMut,
    {
        use crate::detail::BitOps;
        self.0[0].set_nibble(1, group);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Complete,
    Start,
    Continue,
    End,
}

impl Packet {
    pub fn status(&self) -> Status {
        status_from_data(&self.0[..]).unwrap()
    }
}

fn status_from_data(data: &[u32]) -> Result<Status, error::InvalidData> {
    use crate::detail::BitOps;
    use Status::*;
    match u8::from(data[0].nibble(2)) {
        0x0 => Ok(Complete),
        0x1 => Ok(Start),
        0x2 => Ok(Continue),
        0x3 => Ok(End),
        _ => Err(error::InvalidData("Invalid SysEx7 status byte")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        assert_eq!(
            &*Packet::try_from(&[0x3000_0000, 0x0000_0000][..]).unwrap(),
            &[0x3000_0000, 0x0000_0000][..],
        );
    }

    #[test]
    fn construction_long_slice() {
        assert_eq!(
            &*Packet::try_from(&[0x3000_0000, 0x0, 0x0, 0x0][..]).unwrap(),
            &[0x3000_0000, 0x0000_0000][..],
        );
    }

    #[test]
    fn construction_short_slice() {
        assert_eq!(
            Packet::try_from(&[0x3000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT
            )),
        );
    }

    #[test]
    fn construction_wrong_ump_type() {
        assert_eq!(
            Packet::try_from(&[0x0000_0000, 0x0000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            )),
        );
    }

    #[test]
    fn construction_complete() {
        assert_eq!(
            Packet::try_from(&[0x3000_0000, 0x0000_0000][..])
                .unwrap()
                .status(),
            Status::Complete,
        );
    }

    #[test]
    fn construction_start() {
        assert_eq!(
            Packet::try_from(&[0x3010_0000, 0x0000_0000][..])
                .unwrap()
                .status(),
            Status::Start,
        );
    }

    #[test]
    fn construction_continue() {
        assert_eq!(
            Packet::try_from(&[0x3020_0000, 0x0000_0000][..])
                .unwrap()
                .status(),
            Status::Continue,
        );
    }

    #[test]
    fn construction_end() {
        assert_eq!(
            Packet::try_from(&[0x3030_0000, 0x0000_0000][..])
                .unwrap()
                .status(),
            Status::End,
        );
    }
}
