use crate::{
    *,
    error::Error,
    message::{helpers as message_helpers, sysex},
    result::Result,
    util::{debug, BitOps, Truncate},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadIterator<'a> {
    data: &'a [u32],
    message_index: usize,
    payload_index: usize,
}

impl<'a> PayloadIterator<'a> {
    fn value(&self) -> u8 {
        let buffer_index = self.message_index * 4 + (self.payload_index + 3) / 4;
        let octet_index = (self.payload_index + 3) % 4;
        self.data[buffer_index].octet(octet_index)
    }
    fn message_size(&self, message_index: usize) -> usize {
        u32::from(self.data[message_index * 4].nibble(3)) as usize - 1
    }
    fn finished(&self) -> bool {
        self.data.len() / 4 == self.message_index
    }
    fn len(&self) -> usize {
        let mut mi = self.message_index;
        let mut count = self.message_size(mi) - self.payload_index;
        mi += 1;
        loop {
            if mi == self.data.len() / 4 {
                break;
            }
            count += self.message_size(mi);
            mi += 1;
        }
        count
    }
    fn advance(&mut self) {
        self.payload_index += 1;
        if self.payload_index == self.message_size(self.message_index) {
            // end of message
            self.message_index += 1;
            self.payload_index = 0;
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

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        loop {
            if self.finished() {
                break;
            }
            if n == 0 {
                break;
            }
            let remaining = self.message_size(self.message_index) - self.payload_index;
            if n >= remaining {
                n -= remaining;
                self.message_index += 1;
                self.payload_index = 0;
            } else {
                self.payload_index += n;
                n = 0;
            }
        }
        if self.finished() {
            None
        } else {
            let ret = self.value();

            self.advance();

            Some(ret)
        }
    }

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex8Message<'a>(&'a [u32]);

impl<'a> Sysex8Message<'a> {
    const OP_CODE: u4 = u4::new(0x5);
    pub fn builder(buffer: &'a mut [u32]) -> Sysex8MessageBuilder<'a> {
        Sysex8MessageBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn status(&self) -> Status {
        try_status_from_packet(self.0).expect("Valid status")
    }
    pub fn stream_id(&self) -> u8 {
        self.0[0].octet(2)
    }
    pub fn payload(&self) -> PayloadIterator {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        validate_buffer(data)?;
        match try_status_from_packet(data) {
            Ok(status) => {
                validate_data(data, status)?;
                validate_packet(data)?;
                Ok(Sysex8Message(&data[..4]))
            }
            Err(e) => Err(e),
        }
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
}

debug::message_debug_impl!(Sysex8Message);

pub struct Sysex8MessageBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> Sysex8MessageBuilder<'a> {
    pub fn group(&mut self, g: u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
    /// When called with `Status::UnexpectedEnd(_)` the payload buffer
    /// will be filled with zeros accordingly.
    pub fn status(&mut self, s: Status) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(
                2,
                match s {
                    Status::Complete => u4::new(0x0),
                    Status::Begin => u4::new(0x1),
                    Status::Continue => u4::new(0x2),
                    Status::End => u4::new(0x3),
                    Status::UnexpectedEnd(_) => u4::new(0x3),
                },
            );
            if let Status::UnexpectedEnd(validity) = s {
                buffer[0] &= 0xFFFF_FF00;
                buffer[1..4].copy_from_slice(&[0x0, 0x0, 0x0]);
                match validity {
                    Validity::Valid => {
                        buffer[0].set_nibble(3, u4::new(0x1));
                    }
                    Validity::Invalid => {
                        buffer[0].set_nibble(3, u4::new(0xF));
                    }
                }
            }
        }
        self
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, id);
        }
        self
    }
    pub fn payload<'b, I: core::iter::Iterator<Item = &'b u8>>(
        &mut self,
        mut data: I,
    ) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            // start at one because we always have
            // a stream id
            let mut count = 1_u8;
            for i in 0_usize..13_usize {
                if let Some(&v) = data.next() {
                    buffer[(i + 3) / 4].set_octet((i + 3) % 4, v);
                    count += 1;
                } else {
                    break;
                }
            }
            if data.next().is_some() {
                self.0 = Err(Error::InvalidData);
            } else {
                buffer[0].set_nibble(3, count.truncate());
            }
        }
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        if buffer.len() >= 4 {
            let buffer = &mut buffer[..4];
            for v in buffer.iter_mut() {
                *v = 0;
            }
            message_helpers::write_type_to_packet(Sysex8Message::OP_CODE, buffer);
            buffer[0].set_nibble(3, u4::new(0x1)); // stream id
            Self(Ok(buffer))
        } else {
            Self(Err(Error::BufferOverflow))
        }
    }
    pub fn build(&'a self) -> Result<Sysex8Message<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(Sysex8Message(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Status {
    #[default]
    Complete,
    Begin,
    Continue,
    End,
    UnexpectedEnd(Validity),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Validity {
    Valid,
    Invalid,
}

fn validate_packet(p: &[u32]) -> Result<()> {
    if p[0].nibble(0) != Sysex8Message::OP_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn validate_buffer(buf: &[u32]) -> Result<()> {
    if buf.len() < 4 {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn try_status_from_packet(p: &[u32]) -> Result<Status> {
    match u8::from(p[0].nibble(2)) {
        0x0 => Ok(Status::Complete),
        0x1 => Ok(Status::Begin),
        0x2 => Ok(Status::Continue),
        0x3 => {
            let all_data_set_to_zero = {
                if p[0] & 0x0000_0011 != 0x0 {
                    false
                } else if p.len() > 1 {
                    p[1..].iter().all(|b| *b == 0)
                } else {
                    true
                }
            };
            if all_data_set_to_zero {
                if number_of_bytes(p) == u4::new(0x1) {
                    Ok(Status::UnexpectedEnd(Validity::Valid))
                } else if number_of_bytes(p) == u4::new(0xF) {
                    Ok(Status::UnexpectedEnd(Validity::Invalid))
                } else {
                    Ok(Status::End)
                }
            } else {
                Ok(Status::End)
            }
        }
        _ => Err(Error::InvalidData),
    }
}

fn number_of_bytes(p: &[u32]) -> u4 {
    p[0].nibble(3)
}

fn validate_data(p: &[u32], status: Status) -> Result<()> {
    let n: usize = u8::from(number_of_bytes(p)).into();
    let unexpected_end = matches!(status, Status::UnexpectedEnd(_));
    if n == 0 {
        // we expect a stream id
        Err(Error::InvalidData)
    } else if unexpected_end {
        // data should be set to zero
        // but we wont make it a hard requirement here
        Ok(())
    } else if n > 14 {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex8MessageGroup<'a>(&'a [u32]);

debug::message_debug_impl!(Sysex8MessageGroup);

impl<'a> Sysex8MessageGroup<'a> {
    pub fn builder(buffer: &'a mut [u32]) -> Sysex8MessageGroupBuilder<'a> {
        Sysex8MessageGroupBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn payload(&self) -> PayloadIterator {
        <Self as sysex::SysexMessages>::payload(self)
    }
    pub fn messages(&self) -> Sysex8MessageGroupIterator<'a> {
        Sysex8MessageGroupIterator(self.0.chunks_exact(4))
    }
    pub fn from_data(buffer: &'a [u32]) -> Result<Self> {
        if buffer.len() % 4 != 0 || buffer.is_empty() {
            return Err(Error::InvalidData);
        }
        for chunk in buffer.chunks(4) {
            Sysex8Message::from_data(chunk)?;
        }
        message_helpers::sysex_group_consistent_groups(buffer, 4)?;
        message_helpers::validate_sysex_group_statuses(
            buffer,
            |s| s == u4::new(0x0),
            |s| s == u4::new(0x1),
            |s| s == u4::new(0x2),
            |s| s == u4::new(0x3),
            4,
        )?;
        Ok(Sysex8MessageGroup(buffer))
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
}

pub struct Sysex8MessageGroupIterator<'a>(core::slice::ChunksExact<'a, u32>);

pub struct Sysex8MessageGroupBuilder<'a> {
    buffer: &'a mut [u32],
    size: usize,
    error: Option<Error>,
    group: u4,
    stream_id: u8,
}

impl<'a> Sysex8MessageGroupBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Sysex8MessageGroupBuilder {
            buffer,
            size: 0,
            error: None,
            group: u4::new(0x0),
            stream_id: 0x0,
        }
    }

    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        if self.error.is_some() || self.stream_id == id {
            return self;
        }
        self.stream_id = id;
        for chunk in self.buffer[..self.size * 4].chunks_exact_mut(4) {
            chunk[0].set_octet(2, id);
        }
        self
    }

    pub fn group(&mut self, g: u4) -> &mut Self {
        if self.error.is_some() || self.group == g {
            return self;
        }
        self.group = g;
        for chunk in self.buffer[..self.size * 4].chunks_exact_mut(4) {
            chunk[0].set_nibble(1, g);
        }
        self
    }

    pub fn payload<I: core::iter::Iterator<Item = u8>>(&mut self, mut iter: I) -> &mut Self {
        if self.error.is_some() {
            return self;
        }

        let first = iter.next();

        if first.is_some() {
            self.grow();
        } else {
            return self;
        }

        if self.error.is_some() {
            return self;
        }

        let message_index = 4 * (self.size - 1);
        let mut stop = false;

        self.buffer[message_index].set_octet(3, first.unwrap());
        self.increment_message_size(message_index);

        for i in 1..13 {
            match iter.next() {
                Some(v) => {
                    self.buffer[message_index + (i + 3) / 4].set_octet((3 + i) % 4, v);
                    self.increment_message_size(message_index);
                }
                None => {
                    stop = true;
                    break;
                }
            }
        }

        if stop {
            self
        } else {
            self.payload(iter)
        }
    }

    fn increment_message_size(&mut self, message_index: usize) {
        let new_value = self.buffer[message_index].nibble(3) + u4::new(1);
        self.buffer[message_index].set_nibble(3, new_value);
    }

    fn grow(&mut self) {
        if self.buffer.len() < 4 * (self.size + 1) {
            self.error = Some(Error::BufferOverflow);
            return;
        }

        {
            let mut builder =
                Sysex8Message::builder(&mut self.buffer[4 * self.size..4 * (self.size + 1)]);
            builder.group(self.group);
            builder.stream_id(self.stream_id);
            match self.size {
                0 => {
                    builder.status(Status::Complete);
                }
                _ => {
                    builder.status(Status::End);
                }
            }
            builder.build().expect("successful message build");
        }

        if self.size != 0 {
            let mut prev_builder =
                Sysex8MessageBuilder(Ok(&mut self.buffer[4 * (self.size - 1)..4 * self.size]));
            match self.size {
                1 => {
                    prev_builder.status(Status::Begin);
                }
                _ => {
                    prev_builder.status(Status::Continue);
                }
            }
            prev_builder.build().expect("successful message build");
        }
        self.size += 1;
    }

    pub fn build(&'a self) -> Result<Sysex8MessageGroup<'a>> {
        match &self.error {
            None => match self.size {
                0 => Err(Error::InvalidData),
                _ => Ok(Sysex8MessageGroup(&self.buffer[..4 * self.size])),
            },
            Some(e) => Err(e.clone()),
        }
    }
}

impl<'a> sysex::SysexMessages for Sysex8MessageGroup<'a> {
    type Builder = Sysex8MessageGroupBuilder<'a>;
    type Byte = u8;
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&self) -> Self::PayloadIterator {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        assert_eq!(
            Sysex8Message::builder(&mut [0x0; 4])
                .group(u4::new(0xA))
                .stream_id(0xC6)
                .status(Status::Continue)
                .payload([0x12, 0x34, 0x56, 0x78, 0x90].iter())
                .build(),
            Ok(Sysex8Message(&[0x5A26_C612, 0x3456_7890, 0x0, 0x0])),
        )
    }

    #[test]
    fn builder_large_payload() {
        assert_eq!(
            Sysex8Message::builder(&mut [0x0; 4])
                .payload([0x0; 14].iter())
                .build(),
            Err(Error::InvalidData),
        )
    }

    #[test]
    fn must_have_stream_id() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5000_0000, 0x0, 0x0, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5C01_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xC),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5001_9900, 0x0, 0x0, 0x0])
                .unwrap()
                .stream_id(),
            0x99,
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5021_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::Continue,
        );
    }

    #[test]
    fn status_end() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5032_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::End,
        );
    }

    #[test]
    fn status_unexpected_end_valid() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5031_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::UnexpectedEnd(Validity::Valid),
        );
    }

    #[test]
    fn status_unexpected_end_invalid() {
        assert_eq!(
            Sysex8Message::from_data(&[0x503F_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::UnexpectedEnd(Validity::Invalid),
        );
    }

    #[test]
    fn payload() {
        let message =
            Sysex8Message::from_data(&[0x5009_FF00, 0x1122_3344, 0x5566_7700, 0x0]).unwrap();
        let mut buffer = [0u8; 8];
        for (i, v) in message.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]);
    }

    #[test]
    fn group_builder() {
        assert_eq!(
            Sysex8MessageGroup::builder(&mut [0x0; 8])
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .payload(0..15)
                .build(),
            Ok(Sysex8MessageGroup(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5433_BB0D,
                0x0E00_0000,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn group_builder_metadata_after_payload() {
        assert_eq!(
            Sysex8MessageGroup::builder(&mut [0x0; 8])
                .payload(0..15)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8MessageGroup(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5433_BB0D,
                0x0E00_0000,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn group_builder_complete() {
        assert_eq!(
            Sysex8MessageGroup::builder(&mut [0x0; 4])
                .payload(0x0..0xA)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8MessageGroup(&[
                0x540B_BB00,
                0x0102_0304,
                0x0506_0708,
                0x0900_0000,
            ])),
        );
    }

    #[test]
    fn group_builder_dirty_buffer() {
        assert_eq!(
            Sysex8MessageGroup::builder(&mut [0xFF; 4])
                .payload(0x0..0xA)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8MessageGroup(&[
                0x540B_BB00,
                0x0102_0304,
                0x0506_0708,
                0x0900_0000,
            ])),
        );
    }

    #[test]
    fn group_builder_payload_in_batches() {
        assert_eq!(
            Sysex8MessageGroup::builder(&mut [0x0; 8])
                .payload(0x0..0xA)
                .payload(0x0..0x5)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8MessageGroup(&[
                0x541B_BB00,
                0x0102_0304,
                0x0506_0708,
                0x0900_0000,
                0x5436_BB00,
                0x0102_0304,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn group_from_data_inconsistent_groups() {
        assert_eq!(
            Sysex8MessageGroup::from_data(&[
                0x5011_0000,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5131_0000,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group_from_data_inconsistent_status() {
        assert_eq!(
            Sysex8MessageGroup::from_data(&[0x5011_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group_from_data_invalid_message() {
        assert_eq!(
            Sysex8MessageGroup::from_data(&[0x0001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
        assert_eq!(
            Sysex8MessageGroup::from_data(&[0x5000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group_payload() {
        let mut buffer = [0x0; 15];
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        for (i, v) in message_group.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(
            &buffer,
            &[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE,]
        )
    }

    #[test]
    fn group_payload_count() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        assert_eq!(message_group.payload().count(), 15);
    }

    #[test]
    fn group_payload_count_start_from_one() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.count(), 14);
    }

    #[test]
    fn group_payload_6th() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        assert_eq!(payload.nth(6), Some(0x6));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn group_payload_0th() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        assert_eq!(payload.nth(0), Some(0x0));
    }

    #[test]
    fn group_payload_nth_last() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        assert_eq!(payload.nth(14), Some(0xE));
    }

    #[test]
    fn group_payload_nth_past_the_end() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        assert_eq!(payload.nth(15), None);
    }

    #[test]
    fn group_payload_nth_consumes_nth() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        payload.nth(1);
        assert_eq!(payload.next(), Some(0x02));
    }

    #[test]
    fn group_payload_nth_last_none_left() {
        let message_group = Sysex8MessageGroup(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ]);
        let mut payload = message_group.payload();
        payload.nth(14);
        assert_eq!(payload.next(), None);
    }
}
