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

        if u8::from(data[0].nibble(3)) > 6 {
            return Err(error::InvalidData(sysex7::ERR_INVALID_PACKET_SIZE));
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

pub struct PayloadIterator<'a> {
    data: &'a [u32; 2],
    index: usize,
}

impl core::iter::Iterator for PayloadIterator<'_> {
    type Item = crate::ux::u7;
    fn next(&mut self) -> Option<Self::Item> {
        use crate::detail::BitOps;
        if self.index >= self.packet_size() {
            return None;
        }
        let v = self.data[(self.index + 2) / 4].septet((self.index + 2) % 4);
        self.index += 1;
        Some(v)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        use crate::detail::BitOps;
        if self.index + n >= self.packet_size() {
            self.index = (self.index + n).min(6);
            return None;
        }
        let v = self.data[(self.index + n + 2) / 4].septet((self.index + n + 2) % 4);
        self.index += n + 1;
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
        let len = u8::from(self.data[0].nibble(3)) as usize;
        debug_assert!(len <= 6);
        len
    }
}

impl Packet {
    pub fn status(&self) -> Status {
        status_from_data(&self.0[..]).unwrap()
    }

    pub fn payload(&self) -> PayloadIterator {
        PayloadIterator {
            data: &self.0,
            index: 0,
        }
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
    fn construction_invalid_payload_size() {
        assert_eq!(
            Packet::try_from(&[0x3007_0000, 0x0, 0x0, 0x0][..]),
            Err(error::InvalidData(sysex7::ERR_INVALID_PACKET_SIZE)),
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

    #[test]
    fn payload_empty() {
        assert_eq!(
            Packet::try_from(&[0x3000_0000, 0x0000_0000][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<ux::u7>>(),
            std::vec::Vec::<ux::u7>::new(),
        );
    }

    #[test]
    fn payload_one_item() {
        assert_eq!(
            Packet::try_from(&[0x3001_7F00, 0x0000_0000][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<ux::u7>>(),
            std::vec![ux::u7::new(0x7F),]
        );
    }

    #[test]
    fn payload_two_items() {
        assert_eq!(
            Packet::try_from(&[0x3002_0102, 0x0000_0000][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<ux::u7>>(),
            std::vec![ux::u7::new(0x01), ux::u7::new(0x02)],
        );
    }

    #[test]
    fn payload_full_payload() {
        assert_eq!(
            Packet::try_from(&[0x3006_0102, 0x0304_0506][..])
                .unwrap()
                .payload()
                .collect::<std::vec::Vec<ux::u7>>(),
            std::vec![
                ux::u7::new(0x01),
                ux::u7::new(0x02),
                ux::u7::new(0x03),
                ux::u7::new(0x04),
                ux::u7::new(0x05),
                ux::u7::new(0x06),
            ],
        );
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn payload_nth_0() {
        assert_eq!(
            Packet::try_from(&[0x3006_0102, 0x0304_0506][..])
                .unwrap()
                .payload()
                .nth(0),
            Some(ux::u7::new(0x01)),
        );
    }

    #[test]
    fn payload_nth_1() {
        assert_eq!(
            Packet::try_from(&[0x3006_0102, 0x0304_0506][..])
                .unwrap()
                .payload()
                .nth(1),
            Some(ux::u7::new(0x02)),
        );
    }

    #[test]
    fn payload_nth_5() {
        assert_eq!(
            Packet::try_from(&[0x3006_0102, 0x0304_0506][..])
                .unwrap()
                .payload()
                .nth(5),
            Some(ux::u7::new(0x06)),
        );
    }

    #[test]
    fn payload_nth_6() {
        assert_eq!(
            Packet::try_from(&[0x3006_0102, 0x0304_0506][..])
                .unwrap()
                .payload()
                .nth(6),
            None,
        );
    }

    #[test]
    fn payload_nth_6_followed_by_next_should_return_none() {
        let buffer = [0x3006_0102, 0x0304_0506];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.nth(6), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn payload_call_next_and_the_iterator_length_should_be_one_fewer() {
        let buffer = [0x3006_0102, 0x0304_0506];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.len(), 6);
        iter.next();
        assert_eq!(iter.len(), 5);
    }

    #[test]
    fn payload_exhaustive_nth_should_leave_iter_length_0() {
        let buffer = [0x3006_0102, 0x0304_0506];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.len(), 6);
        iter.nth(6);
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn payload_over_exhaustive_nth_should_leave_iter_length_0() {
        let buffer = [0x3006_0102, 0x0304_0506];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.len(), 6);
        iter.nth(7);
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn payload_nth_should_leave_iter_with_n_fewer_length() {
        let buffer = [0x3006_0102, 0x0304_0506];
        let message = Packet::try_from(&buffer[..]).unwrap();
        let mut iter = message.payload();
        assert_eq!(iter.len(), 6);
        iter.nth(2);
        assert_eq!(iter.len(), 3);
    }
}
