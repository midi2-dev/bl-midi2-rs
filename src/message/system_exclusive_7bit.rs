use crate::{error::Error, message::helpers as message_helpers, result::Result, util::BitOps, *};

const MESSAGE_TYPE: u4 = u4::new(0x3);
const STATUS_COMPLETE: u4 = u4::new(0x0);
const STATUS_START: u4 = u4::new(0x1);
const STATUS_CONTINUE: u4 = u4::new(0x2);
const STATUS_END: u4 = u4::new(0x3);

trait Sysex7BuilderInternal {
    fn buffer(&self) -> &[u32];
    fn buffer_mut(&mut self) -> &mut [u32];
    fn resize_buffer(&mut self, sz: usize);
    fn is_error(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadIterator<'a> {
    data: &'a [u32],
    message_index: usize,
    payload_index: usize,
}

impl<'a> PayloadIterator<'a> {
    fn value(&self) -> u8 {
        let buffer_index = self.message_index * 2 + (self.payload_index + 2) / 4;
        let octet_index = (self.payload_index + 2) % 4;
        self.data[buffer_index].octet(octet_index)
    }
    fn message_size(&self, message_index: usize) -> usize {
        u32::from(self.data[message_index * 2].nibble(3)) as usize
    }
    fn finished(&self) -> bool {
        self.data.len() / 2 == self.message_index
    }
    fn len(&self) -> usize {
        let mut mi = self.message_index;
        let mut count = self.message_size(mi) - self.payload_index;
        mi += 1;
        loop {
            if mi == self.data.len() / 2 {
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
            if n == 0 || self.finished() {
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
            let ret = Some(self.value());
            self.advance();
            ret
        }
    }

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Sysex7Borrowed<'a> {
    type Owned = Sysex7Owned;
    fn into_owned(self) -> Self::Owned {
        Sysex7Owned(self.0.to_vec())
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Sysex7Message<'a> {
    type Owned = Sysex7Owned;
    fn into_owned(self) -> Sysex7Owned {
        use Sysex7Message::*;
        match self {
            Owned(m) => m,
            Borrowed(m) => m.into_owned(),
        }
    }
}

#[derive(midi2_proc::UmpDebug, Clone, PartialEq, Eq)]
pub struct Sysex7Borrowed<'a>(&'a [u32]);

#[derive(midi2_proc::UmpDebug, Clone, PartialEq, Eq)]
#[cfg(feature = "std")]
pub struct Sysex7Owned(std::vec::Vec<u32>);

#[derive(derive_more::From, midi2_proc::Data, midi2_proc::Grouped, Debug, Clone, PartialEq, Eq)]
pub enum Sysex7Message<'a> {
    #[cfg(feature = "std")]
    Owned(Sysex7Owned),
    Borrowed(Sysex7Borrowed<'a>),
}

impl<'a> Sysex7Borrowed<'a> {
    pub fn _messages(&self) -> Sysex7MessageGroupIterator<'a> {
        Sysex7MessageGroupIterator(self.0.chunks_exact(2))
    }
    pub fn builder(buffer: &'a mut [u32]) -> Sysex7BorrowedBuilder<'a> {
        Sysex7BorrowedBuilder::new(buffer)
    }
}

#[cfg(feature = "std")]
impl Sysex7Owned {
    pub fn _messages(&self) -> Sysex7MessageGroupIterator {
        Sysex7MessageGroupIterator(self.0.chunks_exact(2))
    }
    pub fn builder() -> Sysex7Builder<Self> {
        Sysex7Builder::new()
    }
}

#[cfg(feature = "std")]
impl<'a> Sysex7Message<'a> {
    pub fn builder() -> Sysex7Builder<Self> {
        Sysex7Builder::new()
    }
}

impl<'a> Data for Sysex7Borrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0
    }
}

#[cfg(feature = "std")]
impl Data for Sysex7Owned {
    fn data(&self) -> &[u32] {
        &self.0
    }
}

impl<'a> FromData<'a> for Sysex7Borrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Sysex7Borrowed(buffer)
    }
    fn validate_data(buffer: &[u32]) -> Result<()> {
        if buffer.len() % 2 != 0 || buffer.is_empty() {
            return Err(Error::InvalidData);
        }
        for chunk in buffer.chunks(2) {
            if chunk[0].nibble(0) != MESSAGE_TYPE {
                return Err(Error::InvalidData);
            }
            if u8::from(chunk[0].nibble(3)) > 6 {
                return Err(Error::InvalidData);
            }
        }
        message_helpers::validate_sysex_group_statuses(
            buffer,
            |s| s == u4::new(0x0),
            |s| s == u4::new(0x1),
            |s| s == u4::new(0x2),
            |s| s == u4::new(0x3),
            2,
        )?;
        message_helpers::sysex_group_consistent_groups(buffer, 2)?;
        Ok(())
    }
}

impl<'a> FromData<'a> for Sysex7Message<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Sysex7Message::Borrowed(Sysex7Borrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        Sysex7Borrowed::validate_data(buffer)
    }
}

impl<'a> Grouped for Sysex7Borrowed<'a> {}

impl<'a> Sysex<'a> for Sysex7Borrowed<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload<'b: 'a>(&'b self) -> Self::PayloadIterator {
        Self::PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

#[cfg(feature = "std")]
impl<'a> Sysex<'a> for Sysex7Owned {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload<'b: 'a>(&'b self) -> Self::PayloadIterator {
        Self::PayloadIterator {
            data: &self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a> Sysex<'a> for Sysex7Message<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload<'b: 'a>(&'b self) -> Self::PayloadIterator {
        use Sysex7Message::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.payload(),
            Borrowed(m) => m.payload(),
        }
    }
}

pub struct Sysex7MessageGroupIterator<'a>(core::slice::ChunksExact<'a, u32>);

pub struct Sysex7BorrowedBuilder<'a> {
    buffer: &'a mut [u32],
    size: usize,
    error: Option<Error>,
}

#[cfg(feature = "std")]
pub struct Sysex7Builder<M: core::convert::From<Sysex7Owned>> {
    buffer: std::vec::Vec<u32>,
    error: Option<Error>,
    _phantom_message: core::marker::PhantomData<M>,
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7Owned>> core::default::Default for Sysex7Builder<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Sysex7BuilderInternal for Sysex7BorrowedBuilder<'a> {
    fn buffer(&self) -> &[u32] {
        &self.buffer[..self.size]
    }
    fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer[..self.size]
    }
    fn resize_buffer(&mut self, sz: usize) {
        if self.error.is_some() {
            return;
        }
        if sz > self.buffer.len() {
            self.error = Some(Error::BufferOverflow);
        } else {
            if self.size < sz {
                for d in &mut self.buffer[self.size..sz] {
                    *d = 0x0;
                }
            }
            self.size = sz;
        }
    }
    fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

impl<'a> SysexBuilderInternal for Sysex7BorrowedBuilder<'a> {
    type ByteType = u7;
    fn resize(&mut self, payload_size: usize) {
        resize(self, payload_size);
    }
    fn payload_size(&self) -> usize {
        payload_size(self)
    }
    fn write_datum(&mut self, datum: Self::ByteType, payload_index: usize) {
        if self.error.is_some() {
            return;
        }
        write_datum(self, datum, payload_index);
    }
    fn shift_tail_forward(&mut self, payload_index_tail_start: usize, distance: usize) {
        if self.error.is_some() {
            return;
        }
        shift_tail_forward(self, payload_index_tail_start, distance);
    }
    fn shift_tail_backward(&mut self, payload_index_tail_start: usize, distance: usize) {
        if self.error.is_some() {
            return;
        }
        shift_tail_backward(self, payload_index_tail_start, distance);
    }
}

impl<'a> SysexBorrowedBuilder for Sysex7BorrowedBuilder<'a> {
    type ByteType = u7;
    fn append_payload<I: core::iter::Iterator<Item = u7>>(mut self, data: I) -> Self {
        if self.error.is_some() {
            return self;
        }
        let sz = self.payload_size();
        message_helpers::replace_sysex_payload_range(&mut self, data, sz..);
        self
    }
    fn insert_payload<D>(mut self, data: D, before: usize) -> Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
    {
        if self.error.is_some() {
            return self;
        }
        message_helpers::replace_sysex_payload_range(&mut self, data, before..before);
        self
    }
    fn replace_payload_range<D, R>(mut self, data: D, range: R) -> Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
        R: core::ops::RangeBounds<usize> + core::iter::Iterator<Item = usize>,
    {
        if self.error.is_some() {
            return self;
        }
        message_helpers::replace_sysex_payload_range(&mut self, data, range);
        self
    }
    fn payload<I: core::iter::Iterator<Item = u7>>(mut self, data: I) -> Self {
        if self.error.is_some() {
            return self;
        }
        message_helpers::replace_sysex_payload_range(&mut self, data, 0..);
        self
    }
}

impl<'a> Sysex7BorrowedBuilder<'a> {
    pub fn build(self) -> Result<Sysex7Borrowed<'a>> {
        if let Some(e) = &self.error {
            return Err(e.clone());
        }
        Ok(Sysex7Borrowed(&self.buffer[..self.size]))
    }
    pub fn new(buffer: &'a mut [u32]) -> Self {
        let mut ret = Sysex7BorrowedBuilder {
            buffer,
            size: 0,
            error: None,
        };
        ret.resize(0);
        ret
    }
    pub fn group(mut self, g: u4) -> Self {
        set_group(&mut self, g);
        self
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7Owned>> Sysex7BuilderInternal for Sysex7Builder<M> {
    fn buffer(&self) -> &[u32] {
        &self.buffer[..]
    }
    fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer[..]
    }
    fn resize_buffer(&mut self, sz: usize) {
        self.buffer.resize(sz, 0x0);
    }
    fn is_error(&self) -> bool {
        false
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7Owned>> SysexBuilderInternal for Sysex7Builder<M> {
    type ByteType = u7;
    fn resize(&mut self, payload_size: usize) {
        resize(self, payload_size);
    }
    fn payload_size(&self) -> usize {
        payload_size(self)
    }
    fn write_datum(&mut self, datum: Self::ByteType, payload_index: usize) {
        write_datum(self, datum, payload_index);
    }
    fn shift_tail_forward(&mut self, payload_index_tail_start: usize, distance: usize) {
        shift_tail_forward(self, payload_index_tail_start, distance);
    }
    fn shift_tail_backward(&mut self, payload_index_tail_start: usize, distance: usize) {
        shift_tail_backward(self, payload_index_tail_start, distance);
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7Owned>> SysexBuilder for Sysex7Builder<M> {
    type ByteType = u7;
    fn append_payload<I: core::iter::Iterator<Item = u7>>(&mut self, data: I) -> &mut Self {
        message_helpers::replace_sysex_payload_range(self, data, self.payload_size()..);
        self
    }
    fn insert_payload<D>(&mut self, data: D, before: usize) -> &mut Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
    {
        message_helpers::replace_sysex_payload_range(self, data, before..before);
        self
    }
    fn replace_payload_range<D, R>(&mut self, data: D, range: R) -> &mut Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
        R: core::ops::RangeBounds<usize> + core::iter::Iterator<Item = usize>,
    {
        message_helpers::replace_sysex_payload_range(self, data, range);
        self
    }
    fn payload<I: core::iter::Iterator<Item = u7>>(&mut self, data: I) -> &mut Self {
        message_helpers::replace_sysex_payload_range(self, data, 0..);
        self
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7Owned>> Sysex7Builder<M> {
    pub fn build(&self) -> Result<M> {
        if let Some(e) = &self.error {
            return Err(e.clone());
        }
        Ok(Sysex7Owned(self.buffer.clone()).into())
    }

    pub fn new() -> Self {
        let mut ret = Sysex7Builder {
            buffer: std::vec::Vec::new(),
            error: None,
            _phantom_message: Default::default(),
        };
        ret.resize(0);
        ret
    }
    pub fn group(&mut self, g: u4) -> &mut Self {
        set_group(self, g);
        self
    }
}

fn resize<B: Sysex7BuilderInternal>(builder: &mut B, payload_size: usize) {
    let new_size = if payload_size % 6 == 0 {
        if payload_size == 0 {
            2
        } else {
            payload_size / 3
        }
    } else {
        2 * (payload_size / 6 + 1)
    };
    builder.resize_buffer(new_size);

    let mut iter = builder.buffer_mut().chunks_exact_mut(2).peekable();
    let mut group = None;

    // first packet
    if let Some(first_packet) = iter.next() {
        first_packet[0].set_nibble(0, MESSAGE_TYPE);
        group = Some(first_packet[0].nibble(1));
        if iter.peek().is_some() {
            // start packet
            first_packet[0].set_nibble(2, STATUS_START);
            first_packet[0].set_nibble(3, u4::new(6));
        } else {
            // complete packet
            first_packet[0].set_nibble(2, STATUS_COMPLETE);
            first_packet[0].set_nibble(3, u4::new(payload_size as u8));
        }
    }

    while let Some(chunk) = iter.next() {
        chunk[0].set_nibble(0, MESSAGE_TYPE);
        chunk[0].set_nibble(1, group.unwrap());
        if iter.peek().is_some() {
            // middle packet
            chunk[0].set_nibble(2, STATUS_CONTINUE);
            chunk[0].set_nibble(3, u4::new(6));
        } else {
            // last packet
            chunk[0].set_nibble(2, STATUS_END);
            match payload_size % 6 {
                0 => {
                    chunk[0].set_nibble(3, u4::new(6));
                }
                r => {
                    chunk[0].set_nibble(3, u4::new(r as u8));
                    // zero off the end of the packet
                    for i in r..6 {
                        chunk[(i + 2) / 4].set_septet((i + 2) % 4, u7::new(0x0));
                    }
                }
            };
        }
    }
}

fn payload_size<B: Sysex7BuilderInternal>(builder: &B) -> usize {
    // because the builder always packs the data tightly
    // we can tell the size of the payload more efficiently
    // than the iterator type, which must also be correct
    // for messages not built by this crate
    debug_assert!(builder.buffer().len() % 2 == 0);
    debug_assert!(!builder.buffer().is_empty());
    let size_of_body = 6 * (builder.buffer().len() - 2) / 2;
    let size_of_tail = u8::from(builder.buffer()[builder.buffer().len() - 2].nibble(3)) as usize;
    size_of_body + size_of_tail
}

fn write_datum<B: Sysex7BuilderInternal>(builder: &mut B, datum: u7, payload_index: usize) {
    let buffer_index = 2 * (payload_index / 6);
    let byte_index = payload_index % 6;
    builder.buffer_mut()[buffer_index + (byte_index + 2) / 4]
        .set_septet((byte_index + 2) % 4, datum);
}

fn shift_tail_forward<B: Sysex7BuilderInternal>(
    builder: &mut B,
    payload_index_tail_start: usize,
    distance: usize,
) {
    let tail_end = payload_size(builder);
    resize(builder, tail_end + distance);
    if builder.is_error() {
        return;
    }
    for i in (payload_index_tail_start..tail_end).rev() {
        let buffer_index = 2 * (i / 6);
        let byte_index = i % 6;
        write_datum(
            builder,
            builder.buffer()[buffer_index + (byte_index + 2) / 4].septet((byte_index + 2) % 4),
            i + distance,
        );
    }
}
fn shift_tail_backward<B: Sysex7BuilderInternal>(
    builder: &mut B,
    payload_index_tail_start: usize,
    distance: usize,
) {
    let tail_end = payload_size(builder);
    for i in payload_index_tail_start..tail_end {
        let buffer_index = 2 * (i / 6);
        let byte_index = i % 6;
        write_datum(
            builder,
            builder.buffer()[buffer_index + (byte_index + 2) / 4].septet((byte_index + 2) % 4),
            i - distance,
        );
    }
    resize(builder, payload_size(builder) - distance);
}

fn set_group<B: Sysex7BuilderInternal>(builder: &mut B, g: u4) {
    for chunk in builder.buffer_mut().chunks_exact_mut(2) {
        chunk[0].set_nibble(1, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{buffer::Ump, test_support::random_buffer::RandomBuffer};
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut buffer = Ump::random_buffer::<6>();
        assert_eq!(
            Sysex7Borrowed::builder(&mut buffer)
                .group(u4::new(0x4))
                .payload((0..15).map(u7::new))
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3433_0C0D,
                0x0E00_0000,
            ])),
        );
    }

    #[test]
    fn builder_group_after_payload() {
        assert_eq!(
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<6>())
                .payload((0..15).map(u7::new))
                .group(u4::new(0x4))
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3433_0C0D,
                0x0E00_0000,
            ])),
        );
    }

    #[test]
    fn builder_complete() {
        assert_eq!(
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<2>())
                .group(u4::new(0x4))
                .payload((0..4).map(u7::new))
                .build(),
            Ok(Sysex7Borrowed(&[0x3404_0001, 0x0203_0000,])),
        );
    }

    #[test]
    fn builder_append_payload() {
        assert_eq!(
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<4>())
                .payload((0..4).map(u7::new))
                .append_payload((4..8).map(u7::new))
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3016_0001,
                0x0203_0405,
                0x3032_0607,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn builder_payload_with_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            // N.B. we need a larger than necessary buffer to account for the
            // lack of size_hint implementation from the rubbish iterator.
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<30>())
                .payload(RubbishPayloadIterator::new().map(u7::new))
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3016_0001,
                0x0203_0405,
                0x3026_0607,
                0x0809_0A0B,
                0x3026_0C0D,
                0x0E0F_1011,
                0x3026_1213,
                0x1415_1617,
                0x3026_1819,
                0x1A1B_1C1D,
                0x3026_1E1F,
                0x2021_2223,
                0x3026_2425,
                0x2627_2829,
                0x3026_2A2B,
                0x2C2D_2E2F,
                0x3032_3031,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn builder_replace_range_with_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            // N.B. we need a larger than necessary buffer to account for the
            // lack of size_hint implementation from the rubbish iterator.
            Sysex7Borrowed::builder(&mut [0x0; 50])
                .payload((0..30).map(u7::new))
                .replace_payload_range(RubbishPayloadIterator::new().map(u7::new), 10..20)
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3016_0001,
                0x0203_0405,
                0x3026_0607,
                0x0809_0001,
                0x3026_0203,
                0x0405_0607,
                0x3026_0809,
                0x0A0B_0C0D,
                0x3026_0E0F,
                0x1011_1213,
                0x3026_1415,
                0x1617_1819,
                0x3026_1A1B,
                0x1C1D_1E1F,
                0x3026_2021,
                0x2223_2425,
                0x3026_2627,
                0x2829_2A2B,
                0x3026_2C2D,
                0x2E2F_3031,
                0x3026_1415,
                0x1617_1819,
                0x3034_1A1B,
                0x1C1D_0000,
            ])),
        );
    }

    #[test]
    fn builder_insert_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            // N.B. we need a larger than necessary buffer to account for the
            // lack of size_hint implementation from the rubbish iterator.
            Sysex7Borrowed::builder(&mut [0x0; 50])
                .payload((0..20).map(u7::new))
                .insert_payload(RubbishPayloadIterator::new().map(u7::new), 10)
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3016_0001,
                0x0203_0405,
                0x3026_0607,
                0x0809_0001,
                0x3026_0203,
                0x0405_0607,
                0x3026_0809,
                0x0A0B_0C0D,
                0x3026_0E0F,
                0x1011_1213,
                0x3026_1415,
                0x1617_1819,
                0x3026_1A1B,
                0x1C1D_1E1F,
                0x3026_2021,
                0x2223_2425,
                0x3026_2627,
                0x2829_2A2B,
                0x3026_2C2D,
                0x2E2F_3031,
                0x3026_0A0B,
                0x0C0D_0E0F,
                0x3034_1011,
                0x1213_0000,
            ])),
        );
    }

    #[test]
    fn builder_insert_payload_into_empty_builder() {
        assert_eq!(
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<8>())
                .insert_payload((0..10).map(u7::new), 10)
                .build(),
            Ok(Sysex7Borrowed(&[
                0x3016_0000,
                0x0000_0000,
                0x3026_0000,
                0x0000_0001,
                0x3026_0203,
                0x0405_0607,
                0x3032_0809,
                0x0000_0000,
            ])),
        );
    }
    #[test]
    fn from_data_inconsistent_groups() {
        assert_eq!(
            Sysex7Borrowed::from_data(&[0x3010_0000, 0x0000_0000, 0x3130_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn from_data_incompatible_buffer_size() {
        assert_eq!(
            Sysex7Borrowed::from_data(&[0x3010_0000, 0x0000_0000, 0x3030_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn from_data_complete() {
        assert!(Sysex7Borrowed::from_data(&[0x3000_0000, 0x0000_0000,]).is_ok());
    }

    #[test]
    fn from_data_invalid_message() {
        assert!(Sysex7Borrowed::from_data(&[0x1000_0000, 0x0000_0000,]).is_err());
    }

    #[test]
    fn payload() {
        let mut buffer = [0x0; 8];
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        for (i, v) in message_group.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[0, 1, 2, 3, 4, 5, 6, 7,])
    }

    #[test]
    fn payload_count() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        assert_eq!(message_group.payload().count(), 8);
    }

    #[test]
    fn payload_count_start_from_one() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.count(), 7);
    }

    #[test]
    fn payload_4th() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(4), Some(5));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn payload_0th() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(0), Some(1));
    }

    #[test]
    fn payload_nth_last() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(6), Some(7));
    }

    #[test]
    fn payload_nth_past_the_end() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(7), None);
    }

    #[test]
    fn payload_nth_last_none_left() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.nth(7);
        assert_eq!(payload.next(), None);
    }

    #[test]
    fn payload_from_sysex7_discovery() {
        let group = Sysex7Borrowed(&[
            0x3816_F07E,
            0x7F0D_7101,
            0x3826_007A,
            0x405D_094A,
            0x3826_451E,
            0x2D36_7D7C,
            0x3826_0374,
            0x3701_0605,
            0x3826_310E,
            0x6639_0954,
            0x3831_F700,
            0x0000_0000,
        ]);
        let expected_payload = [
            0xF0, 0x7E, 0x7F, 0x0D, 0x71, 0x01, 0x00, 0x7A, 0x40, 0x5D, 0x09, 0x4A, 0x45, 0x1E,
            0x2D, 0x36, 0x7D, 0x7C, 0x03, 0x74, 0x37, 0x01, 0x06, 0x05, 0x31, 0x0E, 0x66, 0x39,
            0x09, 0x54, 0xF7,
        ];

        assert_eq!(group.payload().count(), expected_payload.len());

        for (actual, expected) in group.payload().zip(expected_payload) {
            assert_eq!(actual, expected);
        }
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod std_tests {
    use super::*;
    use crate::test_support::debug;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            Sysex7Message::builder()
                .group(u4::new(0x4))
                .payload((0..15).map(u7::new))
                .build(),
            Ok(Sysex7Message::Owned(Sysex7Owned(std::vec![
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3433_0C0D,
                0x0E00_0000,
            ]))),
        );
    }

    #[test]
    fn builder_append_payload() {
        assert_eq!(
            Sysex7Message::builder()
                .group(u4::new(0x4))
                .payload((0..15).map(u7::new))
                .append_payload((0..15).map(u7::new))
                .build(),
            Ok(Sysex7Message::Owned(Sysex7Owned(std::vec![
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3426_0C0D,
                0x0E00_0102,
                0x3426_0304,
                0x0506_0708,
                0x3436_090A,
                0x0B0C_0D0E,
            ]))),
        );
    }

    #[test]
    fn builder_replace_payload_range() {
        assert_eq!(
            Sysex7Message::builder()
                .group(u4::new(0x4))
                .payload((0..15).map(u7::new))
                .replace_payload_range((0..15).map(u7::new), 5..10)
                .build(),
            Ok(Sysex7Message::Owned(Sysex7Owned(std::vec![
                0x3416_0001,
                0x0203_0400,
                0x3426_0102,
                0x0304_0506,
                0x3426_0708,
                0x090A_0B0C,
                0x3426_0D0E,
                0x0A0B_0C0D,
                0x3431_0E00,
                0x0000_0000,
            ]))),
        );
    }

    #[test]
    fn builder_payload_with_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            Sysex7Message::builder()
                .group(u4::new(0x4))
                .payload(RubbishPayloadIterator::new().map(u7::new))
                .build(),
            Ok(Sysex7Message::Owned(Sysex7Owned(std::vec![
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3426_0C0D,
                0x0E0F_1011,
                0x3426_1213,
                0x1415_1617,
                0x3426_1819,
                0x1A1B_1C1D,
                0x3426_1E1F,
                0x2021_2223,
                0x3426_2425,
                0x2627_2829,
                0x3426_2A2B,
                0x2C2D_2E2F,
                0x3432_3031,
                0x0000_0000,
            ]))),
        );
    }

    #[test]
    fn payload() {
        let actual: std::vec::Vec<u8> = Sysex7Message::from_data(&[
            0x3416_0001,
            0x0203_0405,
            0x3426_0607,
            0x0809_0A0B,
            0x3433_0C0D,
            0x0E00_0000,
        ])
        .unwrap()
        .payload()
        .collect();
        let expected =
            std::vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE,];
        assert_eq!(debug::ByteData(&actual), debug::ByteData(&expected));
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex7Message::from_data(&[
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3433_0C0D,
                0x0E00_0000,
            ])
            .unwrap()
            .group(),
            u4::new(0x4),
        );
    }

    #[test]
    fn into_owned() {
        assert_eq!(
            Sysex7Message::from_data(&[
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3433_0C0D,
                0x0E00_0000,
            ])
            .unwrap()
            .into_owned(),
            Sysex7Owned::builder()
                .group(u4::new(0x4))
                .payload((0..15).map(u7::new))
                .build()
                .unwrap(),
        );
    }
}
