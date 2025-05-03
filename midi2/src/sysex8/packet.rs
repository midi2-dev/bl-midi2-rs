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

pub struct PayloadIterator<'a> {
    data: &'a [u32; 4],
    index: usize,
}

impl core::iter::Iterator for PayloadIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        use crate::detail::BitOps;
        if self.index >= self.packet_size() {
            return None;
        }
        let v = self.data[(self.index + 3) / 4].octet((self.index + 3) % 4);
        self.index += 1;
        Some(v)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        use crate::detail::BitOps;
        if self.index + n >= self.packet_size() {
            self.index += n;
            return None;
        }
        let v = self.data[(self.index + n + 3) / 4].octet((self.index + n + 3) % 4);
        self.index = (self.index + n).min(13);
        Some(v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }
}

impl core::iter::FusedIterator for PayloadIterator<'_> {}

impl core::iter::ExactSizeIterator for PayloadIterator<'_> {
    fn len(&self) -> usize {
        self.packet_size() - self.index
    }
}

impl PayloadIterator<'_> {
    fn packet_size(&self) -> usize {
        use crate::detail::BitOps;
        let len = u8::from(self.data[0].nibble(3)) as usize - 1;
        debug_assert!(len <= 13);
        len
    }
}

impl Packet {
    pub fn status(&self) -> Status {
        status_from_data(&self.0[..]).unwrap()
    }
    pub fn stream_id(&self) -> u8 {
        sysex8::stream_id_from_packet(&self.0[..])
    }
    pub fn payload(&self) -> PayloadIterator {
        PayloadIterator {
            data: &self.0,
            index: 0,
        }
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

    #[test]
    fn payload_empty() {
        assert_eq!(
            Packet::try_from(&[0x5001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<u8>>(),
            std::vec::Vec::<u8>::new(),
        );
    }

    #[test]
    fn payload_one_item() {
        assert_eq!(
            Packet::try_from(&[0x5002_0001, 0x0000_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<u8>>(),
            std::vec![0x01,]
        );
    }

    #[test]
    fn payload_two_items() {
        assert_eq!(
            Packet::try_from(&[0x5003_0001, 0x0200_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<u8>>(),
            std::vec![0x01, 0x02,]
        );
    }

    #[test]
    fn payload_full() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<u8>>(),
            std::vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            ]
        );
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn payload_nth_0() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(0),
            Some(0x01),
        );
    }

    #[test]
    fn payload_nth_1() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(1),
            Some(0x02),
        );
    }

    #[test]
    fn payload_nth_5() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(5),
            Some(0x06),
        );
    }

    #[test]
    fn payload_nth_8() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(8),
            Some(0x09),
        );
    }

    #[test]
    fn payload_nth_11() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(11),
            Some(0x0C),
        );
    }

    #[test]
    fn payload_nth_12() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(12),
            Some(0x0D),
        );
    }

    #[test]
    fn payload_nth_13() {
        assert_eq!(
            Packet::try_from(&[0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D][..])
                .unwrap()
                .payload()
                .nth(13),
            None,
        );
    }

    #[test]
    fn payload_nth_13_followed_by_next_should_return_none() {
        let buffer = [0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.nth(13), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn payload_call_next_and_the_iterator_length_should_be_one_fewer() {
        let buffer = [0x500E_0001, 0x0203_0405, 0x0607_0809, 0x0A0B_0C0D];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.len(), 13);
        iter.next();
        assert_eq!(iter.len(), 12);
    }
}
