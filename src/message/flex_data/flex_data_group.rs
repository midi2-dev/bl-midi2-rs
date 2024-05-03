pub struct PayloadIterator<'a> {
    data: &'a [u32],
    message_index: usize,
    payload_index: usize,
}

impl<'a> FromData<'a> for FlexDataGroupBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        FlexDataGroupBorrowed(buffer)
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        // whole number of packets
        if buffer.len() % 4 != 0 && !buffer.is_empty() {
            return Err(Error::InvalidData);
        }

        // type code correct
        if !buffer
            .chunks_exact(4)
            .all(|packet| packet[0].nibble(0) == u4::new(0xD))
        {
            return Err(Error::InvalidData);
        }

        // consistent bank
        let bank = super::bank_from_buffer(buffer);
        if !buffer
            .chunks_exact(4)
            .all(|packet| super::bank_from_buffer(packet) == bank)
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

        // consistent channel
        let channel = super::channel_from_buffer(buffer);
        if !buffer
            .chunks_exact(4)
            .all(|packet| super::channel_from_buffer(packet) == channel)
        {
            return Err(Error::InvalidData);
        }

        message_helpers::check_flex_data_or_ump_stream_consistent_packet_formats(buffer, 4)?;

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for FlexDataGroupBorrowed<'a> {
    type Owned = FlexDataGroupOwned;
    fn into_owned(self) -> Self::Owned {
        FlexDataGroupOwned(self.0.to_vec())
    }
}

impl<'a> FlexDataGroupBorrowedBuilder<'a> {
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
            buffer[0] |= 0xD000_0000;
            // set address to default
            buffer[0] |= 0x0010_0000;
        }
        FlexDataGroupBorrowedBuilder {
            buffer,
            size: 1,
            error,
        }
    }
    pub fn build(self) -> Result<FlexDataGroupBorrowed<'a>> {
        match self.error {
            Ok(_) => Ok(FlexDataGroupBorrowed(&self.buffer[0..(self.size * 4)])),
            Err(e) => Err(e),
        }
    }
    pub fn channel(self, channel: Option<u4>) -> Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            if let Some(v) = channel {
                chunk[0].set_crumb(5, u2::new(0x0));
                chunk[0].set_nibble(3, v);
            } else {
                chunk[0].set_crumb(5, u2::new(0x1));
                chunk[0].set_nibble(3, u4::new(0x0));
            }
        }
        self
    }
    pub fn group(self, group: u4) -> Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0].set_nibble(1, group);
        }
        self
    }
    pub fn status(self, v: u8) -> Self {
        for chunk in self.buffer[..self.size * 4].chunks_exact_mut(4) {
            chunk[0] &= !0x0000_00FF;
            chunk[0] |= u32::from(v);
        }
        self
    }
    pub fn bank(self, v: u8) -> Self {
        for chunk in self.buffer[..self.size * 4].chunks_exact_mut(4) {
            chunk[0] &= !0x0000_FF00;
            chunk[0] |= u32::from(v) << 8;
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u8>>(mut self, mut iter: I) -> Self {
        // paylod in batches is not yet supported
        // we reset here
        self.size = 1;
        self.buffer[1..4].clone_from_slice(&[0x0; 3]);

        let mut packet_index = 4;
        loop {
            let Some(v) = iter.next() else {
                break;
            };
            if packet_index == 16 {
                self.grow();
                if self.error.is_err() {
                    break;
                }
                packet_index = 4;
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
impl FlexDataGroupBuilder {
    pub fn new() -> Self {
        Self {
            buffer: std::vec![0xD010_0000, 0x0, 0x0, 0x0],
        }
    }
    pub fn build(&self) -> Result<FlexDataGroupOwned> {
        Ok(FlexDataGroupOwned(self.buffer.clone()))
    }
    pub fn status(&mut self, v: u8) -> &mut Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0].set_octet(3, v);
        }
        self
    }
    pub fn bank(&mut self, v: u8) -> &mut Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0].set_octet(2, v);
        }
        self
    }
    pub fn channel(&mut self, channel: Option<u4>) -> &mut Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            if let Some(v) = channel {
                chunk[0].set_crumb(5, u2::new(0x0));
                chunk[0].set_nibble(3, v);
            } else {
                chunk[0].set_crumb(5, u2::new(0x1));
                chunk[0].set_nibble(3, u4::new(0x0));
            }
        }
        self
    }
    pub fn group(&mut self, group: u4) -> &mut Self {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0].set_nibble(1, group);
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u8>>(&mut self, mut iter: I) -> &mut Self {
        // paylod in batches is not yet supported
        // we reset here
        self.buffer.resize(4, 0x0);
        self.buffer[1..4].clone_from_slice(&[0x0; 3]);

        let mut packet_index = 4;
        loop {
            let Some(v) = iter.next() else {
                break;
            };
            if packet_index == 16 {
                self.grow();
                packet_index = 4;
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

#[cfg(feature = "std")]
impl core::default::Default for FlexDataGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Grouped for FlexDataGroupBorrowed<'a> {}

#[cfg(feature = "std")]
impl Grouped for FlexDataGroupOwned {}

impl<'a, 'b> FlexDataGroup<'a, 'b> for FlexDataGroupBorrowed<'b>
where
    FlexDataGroupBorrowed<'b>: 'a,
{
    fn channel(&'a self) -> Option<u4> {
        super::channel_from_buffer(self.0)
    }
    fn status(&'a self) -> u8 {
        super::status_from_buffer(self.0)
    }
    fn bank(&'a self) -> u8 {
        super::bank_from_buffer(self.0)
    }
    fn payload(&'a self) -> PayloadIterator<'b> {
        PayloadIterator::new(self.0)
    }
}

#[cfg(feature = "std")]
impl<'b, 'a: 'b> FlexDataGroup<'a, 'b> for FlexDataGroupOwned
where
    FlexDataGroupOwned: 'a,
{
    fn channel(&'a self) -> Option<u4> {
        super::channel_from_buffer(&self.0)
    }
    fn status(&'a self) -> u8 {
        super::status_from_buffer(&self.0)
    }
    fn bank(&'a self) -> u8 {
        super::status_from_buffer(&self.0)
    }
    fn payload(&'a self) -> PayloadIterator<'b> {
        PayloadIterator::new(&self.0)
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
        if self.payload_index == 16 {
            // end of message
            self.message_index += 1;
            self.payload_index = 4;
        }
    }
    fn value(&mut self) -> u8 {
        let buffer_index = self.message_index * 4 + self.payload_index / 4;
        let byte_index = self.payload_index % 4;
        self.data[buffer_index].octet(byte_index)
    }
    fn new(data: &'a [u32]) -> Self {
        Self {
            data,
            message_index: 0,
            payload_index: 4,
        }
    }
}

fn grow_buffer(buffer: &mut [u32], size: usize) {
    if size == 1 {
        // set first packet to start
        buffer[0] |= 0b0000_0000_0100_0000_0000_0000_0000_0000;
    }
    if size > 1 {
        // set old last packet to continue
        let old_end_message_index = (size - 1) * 4;
        buffer[old_end_message_index] &= 0b1111_1111_0011_1111_1111_1111_1111_1111;
        buffer[old_end_message_index] |= 0b0000_0000_1000_0000_0000_0000_0000_0000;
    }
    let new_end_message_index = size * 4;
    // clear new packet
    buffer[new_end_message_index..(new_end_message_index + 4)].copy_from_slice(&[0x0; 4]);
    // set type id
    buffer[new_end_message_index] |= 0xD000_0000;
    // set new packet to end
    buffer[new_end_message_index] |= 0b0000_0000_1100_0000_0000_0000_0000_0000;

    // set header data
    buffer[new_end_message_index].set_octet(2, super::bank_from_buffer(buffer));
    buffer[new_end_message_index].set_octet(3, super::status_from_buffer(buffer));
    buffer[new_end_message_index].set_nibble(3, buffer[0].nibble(3)); // group
    let channel = super::channel_from_buffer(buffer);
    channel_to_buffer(
        &mut buffer[new_end_message_index..(new_end_message_index + 4)],
        channel,
    );
}

fn channel_to_buffer(buffer: &mut [u32], channel: Option<u4>) {
    <Ump as Property<Option<u4>, UmpSchema<0x003F_0000, 0x0, 0x0, 0x0>, ()>>::write(
        buffer, channel,
    );
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            FlexDataGroupBuilder::new()
                .payload(0..50)
                .group(u4::new(0x2))
                .status(0x54)
                .bank(0xF2)
                .build(),
            Ok(FlexDataGroupOwned(std::vec![
                0xD250_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0A0B,
                0xD290_F254,
                0x0C0D_0E0F,
                0x1011_1213,
                0x1415_1617,
                0xD290_F254,
                0x1819_1A1B,
                0x1C1D_1E1F,
                0x2021_2223,
                0xD290_F254,
                0x2425_2627,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0xD2D0_F254,
                0x3031_0000,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn builder_complete() {
        assert_eq!(
            FlexDataGroupBuilder::new()
                .payload(0..10)
                .group(u4::new(0x2))
                .status(0x54)
                .bank(0xF2)
                .build(),
            Ok(FlexDataGroupOwned(std::vec![
                0xD210_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0000,
            ])),
        );
    }

    #[test]
    fn builder_channel() {
        assert_eq!(
            FlexDataGroupBuilder::new()
                .payload(0..50)
                .channel(Some(u4::new(0x1)))
                .group(u4::new(0x2))
                .status(0x54)
                .bank(0xF2)
                .build(),
            Ok(FlexDataGroupOwned(std::vec![
                0xD241_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0A0B,
                0xD281_F254,
                0x0C0D_0E0F,
                0x1011_1213,
                0x1415_1617,
                0xD281_F254,
                0x1819_1A1B,
                0x1C1D_1E1F,
                0x2021_2223,
                0xD281_F254,
                0x2425_2627,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0xD2C1_F254,
                0x3031_0000,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            FlexDataGroupBorrowed::from_data(&[
                0xD241_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0A0B,
                0xD281_F254,
                0x0C0D_0E0F,
                0x1011_1213,
                0x1415_1617,
                0xD281_F254,
                0x1819_1A1B,
                0x1C1D_1E1F,
                0x2021_2223,
                0xD281_F254,
                0x2425_2627,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0xD2C1_F254,
                0x3031_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .channel(),
            Some(u4::new(0x1))
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            FlexDataGroupBorrowed::from_data(&[
                0xD241_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0A0B,
                0xD281_F254,
                0x0C0D_0E0F,
                0x1011_1213,
                0x1415_1617,
                0xD281_F254,
                0x1819_1A1B,
                0x1C1D_1E1F,
                0x2021_2223,
                0xD281_F254,
                0x2425_2627,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0xD2C1_F254,
                0x3031_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .status(),
            0x54,
        );
    }

    #[test]
    fn bank() {
        assert_eq!(
            FlexDataGroupBorrowed::from_data(&[
                0xD241_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0A0B,
                0xD281_F254,
                0x0C0D_0E0F,
                0x1011_1213,
                0x1415_1617,
                0xD281_F254,
                0x1819_1A1B,
                0x1C1D_1E1F,
                0x2021_2223,
                0xD281_F254,
                0x2425_2627,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0xD2C1_F254,
                0x3031_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .bank(),
            0xF2,
        );
    }

    #[test]
    fn payload() {
        assert_eq!(
            FlexDataGroupBorrowed::from_data(&[
                0xD241_F254,
                0x0001_0203,
                0x0405_0607,
                0x0809_0A0B,
                0xD281_F254,
                0x0C0D_0E0F,
                0x1011_1213,
                0x1415_1617,
                0xD281_F254,
                0x1819_1A1B,
                0x1C1D_1E1F,
                0x2021_2223,
                0xD281_F254,
                0x2425_2627,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0xD2C1_F254,
                0x3031_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .payload()
            .collect::<std::vec::Vec<u8>>(),
            (0..50)
                .chain(core::iter::repeat(0x0_u8).take(10))
                .collect::<std::vec::Vec<u8>>(),
        );
    }

    #[test]
    fn payload_complete() {
        assert_eq!(
            FlexDataGroupBorrowed::from_data(
                &[0xD210_F254, 0x0001_0203, 0x0405_0607, 0x0809_0000,]
            )
            .unwrap()
            .payload()
            .collect::<std::vec::Vec<u8>>(),
            (0..10)
                .chain(core::iter::repeat(0x0_u8).take(2))
                .collect::<std::vec::Vec<u8>>(),
        );
    }
}
