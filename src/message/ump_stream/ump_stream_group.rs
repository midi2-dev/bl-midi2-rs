#[cfg(feature = "std")]
use crate::IntoOwned;
use crate::{
    message::helpers as message_helpers,
    numeric_types::*,
    traits::{Data, FromData},
    util::BitOps,
    Error, Result,
};

#[derive(midi2_proc::UmpDebug, Clone, PartialEq, Eq)]
pub struct UmpStreamGroupBorrowed<'a>(&'a [u32]);

#[derive(midi2_proc::UmpDebug, Clone, PartialEq, Eq)]
#[cfg(feature = "std")]
pub struct UmpStreamGroupOwned(std::vec::Vec<u32>);

pub struct UmpStreamGroupBorrowedBuilder<'a> {
    pub buffer: &'a mut [u32],
    size: usize,
    error: Result<()>,
}

#[cfg(feature = "std")]
pub struct UmpStreamGroupBuilder<M: core::convert::From<UmpStreamGroupOwned>> {
    pub buffer: std::vec::Vec<u32>,
    phantom_message: core::marker::PhantomData<M>,
}

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

#[cfg(feature = "std")]
impl Data for UmpStreamGroupOwned {
    fn data(&self) -> &[u32] {
        &self.0
    }
}

impl<'a> FromData<'a> for UmpStreamGroupBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        UmpStreamGroupBorrowed(buffer)
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        // whole number of packets
        if buffer.len() % 4 != 0 && !buffer.is_empty() {
            return Err(Error::InvalidData);
        }

        // type code correct
        if !buffer
            .chunks_exact(4)
            .all(|packet| packet[0].nibble(0) == u4::new(0xF))
        {
            return Err(Error::InvalidData);
        }

        // consistent status
        let status = super::status_from_buffer(buffer);
        if !buffer
            .chunks_exact(4)
            .all(|packet| super::status_from_buffer(packet) == status)
        {
            return Err(Error::InvalidData);
        }

        message_helpers::check_flex_data_or_ump_stream_consistent_packet_formats(buffer, 2)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for UmpStreamGroupBorrowed<'a> {
    type Owned = UmpStreamGroupOwned;
    fn into_owned(self) -> Self::Owned {
        UmpStreamGroupOwned(self.0.to_vec())
    }
}

impl<'a> UmpStreamGroupBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        let error = if buffer.len() < 4 {
            Err(Error::BufferOverflow)
        } else {
            Ok(())
        };
        if buffer.len() >= 4 {
            // clear the first packet
            buffer[..4].copy_from_slice(&[0x0; 4]);
            // set type id
            buffer[0] |= 0xF000_0000;
        }
        UmpStreamGroupBorrowedBuilder {
            buffer,
            size: 1,
            error,
        }
    }
    pub fn build(self) -> Result<UmpStreamGroupBorrowed<'a>> {
        match self.error {
            Ok(_) => Ok(UmpStreamGroupBorrowed(&self.buffer[0..(self.size * 4)])),
            Err(e) => Err(e),
        }
    }
    pub fn status(self, v: u10) -> Self {
        for chunk in self.buffer[..self.size * 4].chunks_exact_mut(4) {
            chunk[0] &= !(0x3FF << 16);
            chunk[0] |= u32::from(v) << 16;
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u8>>(mut self, mut iter: I) -> Self {
        // paylod in batches is not yet supported
        // we reset here
        self.size = 1;

        self.buffer[0] &= 0xFFFF_0000;
        self.buffer[1..4].clone_from_slice(&[0x0; 3]);

        let mut packet_index = 2;
        loop {
            let Some(v) = iter.next() else {
                break;
            };
            if packet_index == 16 {
                self.grow();
                if self.error.is_err() {
                    break;
                }
                packet_index = 2;
            }

            let buffer_index = (self.size - 1) * 4 + packet_index / 4;
            let byte_index = packet_index % 4;
            self.buffer[buffer_index].set_octet(byte_index, v);

            packet_index += 1;
        }

        self
    }

    fn grow(&mut self) {
        if self.buffer.len() < 4 * (self.size + 1) {
            self.error = Err(Error::BufferOverflow);
            return;
        }
        grow_buffer(self.buffer, self.size);
        self.size += 1;
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<UmpStreamGroupOwned>> UmpStreamGroupBuilder<M> {
    pub fn new() -> Self {
        Self {
            buffer: std::vec![0xF000_0000, 0x0, 0x0, 0x0],
            phantom_message: Default::default(),
        }
    }
    pub fn build(&self) -> Result<M> {
        Ok(UmpStreamGroupOwned(self.buffer.clone()).into())
    }
    pub fn status(&mut self, v: u10) -> &mut Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0] &= !(0x3FF << 16);
            chunk[0] |= u32::from(v) << 16;
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u8>>(&mut self, mut iter: I) -> &mut Self {
        // paylod in batches is not yet supported
        // we reset here
        self.buffer.resize(4, 0x0);
        self.buffer[0] &= 0xFFFF_0000;
        self.buffer[1..4].clone_from_slice(&[0x0; 3]);

        let mut packet_index = 2;
        loop {
            let Some(v) = iter.next() else {
                break;
            };
            if packet_index == 16 {
                self.grow();
                packet_index = 2;
            }

            let buffer_index = (self.size() - 1) * 4 + packet_index / 4;
            let byte_index = packet_index % 4;
            self.buffer[buffer_index].set_octet(byte_index, v);

            packet_index += 1;
        }

        self
    }

    fn grow(&mut self) {
        let size = self.size();
        self.buffer.extend_from_slice(&[0x0; 4]);
        grow_buffer(&mut self.buffer, size);
    }

    fn size(&self) -> usize {
        self.buffer.len() / 4
    }
}

impl<'a, 'b> UmpStreamGroup<'a, 'b> for UmpStreamGroupBorrowed<'b>
where
    UmpStreamGroupBorrowed<'b>: 'a,
{
    fn status(&'a self) -> u10 {
        super::status_from_buffer(self.0)
    }
    fn payload(&'a self) -> PayloadIterator<'b> {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

#[cfg(feature = "std")]
impl<'b, 'a: 'b> UmpStreamGroup<'a, 'b> for UmpStreamGroupOwned
where
    UmpStreamGroupOwned: 'a,
{
    fn status(&'a self) -> u10 {
        super::status_from_buffer(&self.0)
    }
    fn payload(&'a self) -> PayloadIterator<'b> {
        PayloadIterator {
            data: &self.0,
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

fn grow_buffer(buffer: &mut [u32], size: usize) {
    if size == 1 {
        // set first packet to start
        buffer[0] |= 0b0000_0100_0000_0000_0000_0000_0000_0000;
    }
    if size > 1 {
        // set old last packet to continue
        let old_end_message_index = (size - 1) * 4;
        buffer[old_end_message_index] &= 0b1111_0011_1111_1111_1111_1111_1111_1111;
        buffer[old_end_message_index] |= 0b0000_1000_0000_0000_0000_0000_0000_0000;
    }
    let new_end_message_index = size * 4;
    // clear new packet
    buffer[new_end_message_index..(new_end_message_index + 4)].copy_from_slice(&[0x0; 4]);
    // set type id
    buffer[new_end_message_index] |= 0xF000_0000;
    // set new packet to end
    buffer[new_end_message_index] |= 0b0000_1100_0000_0000_0000_0000_0000_0000;
    // set status
    let status = super::status_from_buffer(buffer);
    buffer[new_end_message_index] |= u32::from(status) << 16;
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::{buffer::Ump, util::RandomBuffer};
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            UmpStreamGroupBuilder::new()
                .status(u10::new(0x185))
                .payload(0..50)
                .build(),
            Ok(UmpStreamGroupOwned(std::vec![
                0xF585_0001,
                0x0203_0405,
                0x0607_0809,
                0x0A0B_0C0D,
                0xF985_0E0F,
                0x1011_1213,
                0x1415_1617,
                0x1819_1A1B,
                0xF985_1C1D,
                0x1E1F_2021,
                0x2223_2425,
                0x2627_2829,
                0xFD85_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn borrowed_builder() {
        assert_eq!(
            UmpStreamGroupBorrowedBuilder::new(&mut Ump::random_buffer::<16>())
                .status(u10::new(0x185))
                .payload(0..50)
                .build(),
            Ok(UmpStreamGroupBorrowed(&[
                0xF585_0001,
                0x0203_0405,
                0x0607_0809,
                0x0A0B_0C0D,
                0xF985_0E0F,
                0x1011_1213,
                0x1415_1617,
                0x1819_1A1B,
                0xF985_1C1D,
                0x1E1F_2021,
                0x2223_2425,
                0x2627_2829,
                0xFD85_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn payload() {
        let actual: std::vec::Vec<u8> = UmpStreamGroupBorrowed::from_data(&[
            0xF585_0001,
            0x0203_0405,
            0x0607_0809,
            0x0A0B_0C0D,
            0xF985_0E0F,
            0x1011_1213,
            0x1415_1617,
            0x1819_1A1B,
            0xF985_1C1D,
            0x1E1F_2021,
            0x2223_2425,
            0x2627_2829,
            0xFD85_2A2B,
            0x2C2D_2E2F,
            0x3031_0000,
            0x0000_0000,
        ])
        .unwrap()
        .payload()
        .collect();
        let expected: std::vec::Vec<u8> =
            (0_u8..50_u8).chain([0x0_u8; 6].iter().cloned()).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn status() {
        assert_eq!(
            UmpStreamGroupBorrowed::from_data(&[
                0xF585_0001,
                0x0203_0405,
                0x0607_0809,
                0x0A0B_0C0D,
                0xF985_0E0F,
                0x1011_1213,
                0x1415_1617,
                0x1819_1A1B,
                0xF985_1C1D,
                0x1E1F_2021,
                0x2223_2425,
                0x2627_2829,
                0xFD85_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
                0x0000_0000,
            ])
            .unwrap()
            .status(),
            u10::new(0x185)
        );
    }
}
