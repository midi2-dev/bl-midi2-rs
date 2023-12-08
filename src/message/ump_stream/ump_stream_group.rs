use crate::{numeric_types::*, traits::Data, util::BitOps};

pub struct UmpStreamGroupBorrowed<'a>(&'a [u32]);
pub struct PayloadIterator<'a> {
    data: &'a [u32],
    message_index: usize,
    payload_index: usize,
}

pub trait UmpStreamGroup<'a, 'b>: Data {
    fn status(&'a self) -> u10;
    fn payload(&'a self) -> PayloadIterator<'b>;
}

impl<'a> Data for UmpStreamGroupBorrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0
    }
}

impl<'a, 'b> UmpStreamGroup<'a, 'b> for UmpStreamGroupBorrowed<'b>
where
    UmpStreamGroupBorrowed<'b>: 'a,
{
    fn status(&'a self) -> u10 {
        todo!();
    }
    fn payload(&'a self) -> PayloadIterator<'b> {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a> core::iter::Iterator for PayloadIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished() {
            return None;
        }
        let ret = Some(self.value());
        self.advance();
        ret
    }
}

impl<'a> PayloadIterator<'a> {
    fn finished(&self) -> bool {
        self.data.len() / 4 == self.message_index
    }
    fn advance(&mut self) {
        self.payload_index += 1;
        if self.payload_index == 14 {
            // end of message
            self.message_index += 1;
            self.payload_index = 0;
        }
    }
    fn value(&mut self) -> u8 {
        let buffer_index = self.message_index * 4 + (self.payload_index + 2) / 4;
        let byte_index = (self.payload_index + 2) % 4;
        self.data[buffer_index].octet(byte_index)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn payload() {
        let actual: std::vec::Vec<u8> = UmpStreamGroupBorrowed(&[
            0x0000_0001,
            0x0203_0405,
            0x0607_0809,
            0x0A0B_0C0D,
            0x0000_0E0F,
            0x1011_1213,
            0x1415_1617,
            0x1819_1A1B,
            0x0000_1C1D,
            0x1E1F_2021,
            0x2223_2425,
            0x2627_2829,
            0x0000_2A2B,
            0x2C2D_2E2F,
            0x3031_0000,
            0x0000_0000,
        ])
        .payload()
        .collect();
        let expected: std::vec::Vec<u8> = (0_u8..50_u8)
            .into_iter()
            .chain([0x0_u8; 6].iter().cloned())
            .collect();
        assert_eq!(actual, expected);
    }
}
