use crate::{error, system_common};

#[derive(Eq, PartialEq, Clone, midi2_proc::Debug)]
pub struct Packet(pub(crate) [u32; 1]);

impl crate::traits::BufferAccess<[u32; 1]> for Packet {
    fn buffer_access(&self) -> &[u32; 1] {
        &self.0
    }
    fn buffer_access_mut(&mut self) -> &mut [u32; 1]
    where
        [u32; 1]: crate::buffer::BufferMut,
    {
        &mut self.0
    }
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Packet {
    type Error = error::InvalidData;
    fn try_from(data: &'a [u32]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT,
            ));
        }

        use crate::detail::BitOps;
        if u8::from(data[0].nibble(0)) != system_common::UMP_MESSAGE_TYPE {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE,
            ));
        }

        Ok(Packet({
            let mut buffer = [0x0; 1];
            buffer[0] = data[0];
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

impl crate::Grouped<[u32; 1]> for Packet {
    fn group(&self) -> crate::ux::u4 {
        use crate::detail::BitOps;
        self.0[0].nibble(1)
    }
    fn set_group(&mut self, group: crate::ux::u4)
    where
        [u32; 1]: crate::buffer::BufferMut,
    {
        use crate::detail::BitOps;
        self.0[0].set_nibble(1, group);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn construction() {
        assert!(Packet::try_from(&[0x1000_0000][..]).is_ok());
    }

    #[test]
    fn construction_incorrect_ump_message_type() {
        assert_eq!(
            Packet::try_from(&[0x0000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            )),
        );
    }

    #[test]
    fn construction_short_slice() {
        assert_eq!(
            Packet::try_from(&[][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT
            )),
        );
    }

    #[test]
    fn group() {
        use crate::Grouped;
        assert_eq!(
            Packet::try_from(&[0x1A00_0000][..]).unwrap().group(),
            crate::num::u4::new(0xA),
        );
    }
}
