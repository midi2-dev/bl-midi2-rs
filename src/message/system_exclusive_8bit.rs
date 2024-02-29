use crate::{error::Error, message::helpers as message_helpers, result::Result, util::BitOps, *};

const MESSAGE_TYPE: u4 = u4::new(0x5);
const STATUS_COMPLETE: u4 = u4::new(0x0);
const STATUS_START: u4 = u4::new(0x1);
const STATUS_CONTINUE: u4 = u4::new(0x2);
const STATUS_END: u4 = u4::new(0x3);
const PACKET_CAPACITY_U8: u8 = 14;
const PACKET_CAPACITY: usize = 14;
const PACKET_SIZE: usize = 4;

const fn capacity_without_stream_id() -> usize {
    PACKET_CAPACITY - 1
}

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

trait Sysex8BuilderInternal {
    fn buffer(&self) -> &[u32];
    fn is_error(&self) -> bool;
    fn buffer_mut(&mut self) -> &mut [u32];
    fn resize_buffer(&mut self, sz: usize);
}

pub trait Sysex8 {
    fn status(&self) -> Status;
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Sysex8Borrowed<'a> {
    type Owned = Sysex8Owned;
    fn into_owned(self) -> Self::Owned {
        Sysex8Owned(self.0.to_vec())
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Sysex8Message<'a> {
    type Owned = Sysex8Owned;
    fn into_owned(self) -> Sysex8Owned {
        use Sysex8Message::*;
        match self {
            Owned(m) => m,
            Borrowed(m) => m.into_owned(),
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
    if p[0].nibble(0) != MESSAGE_TYPE {
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

#[derive(midi2_proc::UmpDebug, Clone, PartialEq, Eq)]
pub struct Sysex8Borrowed<'a>(&'a [u32]);

#[cfg(feature = "std")]
#[derive(midi2_proc::UmpDebug, Clone, PartialEq, Eq)]
pub struct Sysex8Owned(std::vec::Vec<u32>);

#[derive(derive_more::From, midi2_proc::Data, midi2_proc::Grouped, Debug, Clone, PartialEq, Eq)]
pub enum Sysex8Message<'a> {
    #[cfg(feature = "std")]
    Owned(Sysex8Owned),
    Borrowed(Sysex8Borrowed<'a>),
}

impl<'a> Sysex8Borrowed<'a> {
    pub fn builder(buffer: &'a mut [u32]) -> Sysex8BorrowedBuilder<'a> {
        Sysex8BorrowedBuilder::new(buffer)
    }
}

#[cfg(feature = "std")]
impl Sysex8Owned {
    pub fn builder() -> Sysex8Builder<Self> {
        Sysex8Builder::new()
    }
}

#[cfg(feature = "std")]
impl<'a> Sysex8Message<'a> {
    pub fn builder() -> Sysex8Builder<Self> {
        Sysex8Builder::new()
    }
}

impl<'a> Data for Sysex8Borrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0
    }
}

#[cfg(feature = "std")]
impl Data for Sysex8Owned {
    fn data(&self) -> &[u32] {
        &self.0
    }
}

impl<'a> FromData<'a> for Sysex8Borrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Sysex8Borrowed(buffer)
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        if buffer.len() % 4 != 0 || buffer.is_empty() {
            return Err(Error::InvalidData);
        }
        for chunk in buffer.chunks(4) {
            validate_packet(chunk)?;
            let Ok(status) = try_status_from_packet(chunk) else {
                return Err(Error::InvalidData);
            };
            validate_data(chunk, status)?;
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
        Ok(())
    }
}

impl<'a> FromData<'a> for Sysex8Message<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Sysex8Message::Borrowed(Sysex8Borrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        Sysex8Borrowed::validate_data(buffer)
    }
}

impl<'a> Grouped for Sysex8Borrowed<'a> {}

#[cfg(feature = "std")]
impl Grouped for Sysex8Owned {}

impl<'a> Streamed for Sysex8Borrowed<'a> {}

#[cfg(feature = "std")]
impl Streamed for Sysex8Owned {}

impl<'a> Streamed for Sysex8Message<'a> {}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex8Borrowed<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&self) -> Self::PayloadIterator {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

#[cfg(feature = "std")]
impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex8Owned {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&'a self) -> Self::PayloadIterator {
        PayloadIterator {
            data: &self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex8Message<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&'a self) -> Self::PayloadIterator {
        use Sysex8Message::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.payload(),
            Borrowed(m) => m.payload(),
        }
    }
}

#[cfg(feature = "std")]
pub struct Sysex8Builder<M: core::convert::From<Sysex8Owned>> {
    buffer: std::vec::Vec<u32>,
    error: Option<Error>,
    _phantom_message: core::marker::PhantomData<M>,
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> Sysex8BuilderInternal for Sysex8Builder<M> {
    fn is_error(&self) -> bool {
        false
    }
    fn buffer(&self) -> &[u32] {
        &self.buffer[..]
    }
    fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer[..]
    }
    fn resize_buffer(&mut self, sz: usize) {
        self.buffer.resize(sz, 0x0);
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> SysexBuilderInternal for Sysex8Builder<M> {
    type ByteType = u8;
    fn resize(&mut self, payload_size: usize) {
        resize(self, payload_size);
    }
    fn write_datum(&mut self, datum: Self::ByteType, payload_index: usize) {
        write_datum(self, datum, payload_index);
    }
    fn payload_size(&self) -> usize {
        payload_size(self)
    }
    fn shift_tail_forward(&mut self, payload_index_tail_start: usize, distance: usize) {
        shift_tail_forward(self, payload_index_tail_start, distance)
    }
    fn shift_tail_backward(&mut self, payload_index_tail_start: usize, distance: usize) {
        shift_tail_backward(self, payload_index_tail_start, distance)
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> SysexBuilder for Sysex8Builder<M> {
    type ByteType = u8;
    fn payload<I: core::iter::Iterator<Item = Self::ByteType>>(&mut self, data: I) -> &mut Self {
        message_helpers::replace_sysex_payload_range(self, data, 0..);
        self
    }
    fn insert_payload<D>(&mut self, data: D, before: usize) -> &mut Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
    {
        message_helpers::replace_sysex_payload_range(self, data, before..before);
        self
    }
    fn append_payload<I: core::iter::Iterator<Item = u8>>(&mut self, data: I) -> &mut Self {
        message_helpers::replace_sysex_payload_range(self, data, self.payload_size()..);
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
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> Sysex8Builder<M> {
    pub fn group(&mut self, g: u4) -> &mut Self {
        set_group(self.buffer_mut(), g);
        self
    }

    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        set_stream_id(self.buffer_mut(), id);
        self
    }

    pub fn new() -> Self {
        let mut ret = Sysex8Builder {
            buffer: std::vec::Vec::new(),
            error: None,
            _phantom_message: Default::default(),
        };
        ret.resize(0);
        ret
    }

    pub fn build(&self) -> Result<M> {
        let None = &self.error else {
            return Err(Error::InvalidData);
        };
        Ok(Sysex8Owned(self.buffer.clone()).into())
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> core::default::Default for Sysex8Builder<M> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Sysex8BorrowedBuilder<'a> {
    buffer: &'a mut [u32],
    size: usize,
    error: Option<Error>,
}

impl<'a> Sysex8BuilderInternal for Sysex8BorrowedBuilder<'a> {
    fn is_error(&self) -> bool {
        self.error.is_some()
    }
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
            return;
        }
        if self.size < sz {
            for d in &mut self.buffer[self.size..sz] {
                *d = 0x0;
            }
        }
        self.size = sz;
    }
}

impl<'a> SysexBuilderInternal for Sysex8BorrowedBuilder<'a> {
    type ByteType = u8;
    fn resize(&mut self, payload_size: usize) {
        resize(self, payload_size);
    }
    fn write_datum(&mut self, datum: Self::ByteType, payload_index: usize) {
        if self.error.is_some() {
            return;
        }
        write_datum(self, datum, payload_index);
    }
    fn payload_size(&self) -> usize {
        payload_size(self)
    }
    fn shift_tail_forward(&mut self, payload_index_tail_start: usize, distance: usize) {
        if self.error.is_some() {
            return;
        }
        shift_tail_forward(self, payload_index_tail_start, distance)
    }
    fn shift_tail_backward(&mut self, payload_index_tail_start: usize, distance: usize) {
        if self.error.is_some() {
            return;
        }
        shift_tail_backward(self, payload_index_tail_start, distance)
    }
}

impl<'a> SysexBorrowedBuilder for Sysex8BorrowedBuilder<'a> {
    type ByteType = u8;
    fn payload<I: core::iter::Iterator<Item = Self::ByteType>>(mut self, data: I) -> Self {
        message_helpers::replace_sysex_payload_range(&mut self, data, 0..);
        self
    }
    fn insert_payload<D>(mut self, data: D, before: usize) -> Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
    {
        message_helpers::replace_sysex_payload_range(&mut self, data, before..before);
        self
    }
    fn append_payload<I: core::iter::Iterator<Item = u8>>(mut self, data: I) -> Self {
        let sz = self.payload_size();
        message_helpers::replace_sysex_payload_range(&mut self, data, sz..);
        self
    }
    fn replace_payload_range<D, R>(mut self, data: D, range: R) -> Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
        R: core::ops::RangeBounds<usize> + core::iter::Iterator<Item = usize>,
    {
        message_helpers::replace_sysex_payload_range(&mut self, data, range);
        self
    }
}

impl<'a> Sysex8BorrowedBuilder<'a> {
    pub fn group(mut self, g: u4) -> Self {
        if self.error.is_some() {
            return self;
        }
        set_group(self.buffer_mut(), g);
        self
    }

    pub fn stream_id(mut self, id: u8) -> Self {
        if self.error.is_some() {
            return self;
        }
        set_stream_id(self.buffer_mut(), id);
        self
    }

    pub fn new(buffer: &'a mut [u32]) -> Self {
        let mut ret = Self {
            buffer,
            size: 0,
            error: None,
        };
        ret.resize(0);
        ret
    }

    pub fn build(self) -> Result<Sysex8Borrowed<'a>> {
        let None = &self.error else {
            return Err(Error::InvalidData);
        };
        Ok(Sysex8Borrowed(&self.buffer[..self.size]))
    }
}

fn set_group(buffer: &mut [u32], g: u4) {
    for chunk in buffer.chunks_exact_mut(4) {
        chunk[0].set_nibble(1, g);
    }
}

fn set_stream_id(buffer: &mut [u32], stream_id: u8) {
    for chunk in buffer.chunks_exact_mut(4) {
        chunk[0].set_octet(2, stream_id);
    }
}

fn resize<B: Sysex8BuilderInternal>(builder: &mut B, payload_size: usize) {
    let new_size = if payload_size % capacity_without_stream_id() == 0 {
        if payload_size == 0 {
            4
        } else {
            4 * payload_size / capacity_without_stream_id()
        }
    } else {
        4 * (payload_size / capacity_without_stream_id() + 1)
    };
    builder.resize_buffer(new_size);

    let mut iter = builder.buffer_mut().chunks_exact_mut(4).peekable();
    let mut group = None;
    let mut stream_id = None;

    // first packet
    if let Some(first_packet) = iter.next() {
        first_packet[0].set_nibble(0, MESSAGE_TYPE);
        group = Some(first_packet[0].nibble(1));
        stream_id = Some(first_packet[0].octet(2));
        if iter.peek().is_some() {
            // start packet
            first_packet[0].set_nibble(2, STATUS_START);
            first_packet[0].set_nibble(3, u4::new(PACKET_CAPACITY_U8));
        } else {
            // complete packet
            first_packet[0].set_nibble(2, STATUS_COMPLETE);
            first_packet[0].set_nibble(3, u4::new(payload_size as u8 + 1));
        }
    }

    while let Some(chunk) = iter.next() {
        chunk[0].set_nibble(0, MESSAGE_TYPE);
        chunk[0].set_nibble(1, group.unwrap());
        chunk[0].set_octet(2, stream_id.unwrap());
        if iter.peek().is_some() {
            // middle packet
            chunk[0].set_nibble(2, STATUS_CONTINUE);
            chunk[0].set_nibble(3, u4::new(PACKET_CAPACITY_U8));
        } else {
            // last packet
            chunk[0].set_nibble(2, STATUS_END);
            match payload_size % capacity_without_stream_id() {
                0 => {
                    chunk[0].set_nibble(3, u4::new(PACKET_CAPACITY_U8));
                }
                r => {
                    chunk[0].set_nibble(3, u4::new(r as u8 + 1));
                    // zero off the end of the packet
                    for i in r..capacity_without_stream_id() {
                        chunk[(i + 3) / 4].set_octet((i + 3) % 4, 0x0);
                    }
                }
            };
        }
    }
}

fn write_datum<B: Sysex8BuilderInternal>(builder: &mut B, datum: u8, payload_index: usize) {
    let buffer_index = PACKET_SIZE * (payload_index / capacity_without_stream_id());
    let byte_index = payload_index % capacity_without_stream_id();
    builder.buffer_mut()[buffer_index + (byte_index + 3) / 4]
        .set_octet((byte_index + 3) % 4, datum);
}

fn payload_size<B: Sysex8BuilderInternal>(builder: &B) -> usize {
    // because the builder always packs the data tightly
    // we can tell the size of the payload more efficiently
    // than the iterator type, which must also be correct
    // for messages not built by this crate
    debug_assert!(builder.buffer().len() % 4 == 0);
    debug_assert!(!builder.buffer().is_empty());
    let size_of_body = capacity_without_stream_id() * (builder.buffer().len() - 4) / 4;
    let size_of_tail =
        u8::from(builder.buffer()[builder.buffer().len() - 4].nibble(3)) as usize - 1;
    size_of_body + size_of_tail
}

fn shift_tail_forward<B: Sysex8BuilderInternal>(
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
        let buffer_index = 4 * (i / capacity_without_stream_id());
        let byte_index = i % capacity_without_stream_id();
        write_datum(
            builder,
            builder.buffer()[buffer_index + (byte_index + 3) / 4].octet((byte_index + 3) % 4),
            i + distance,
        );
    }
}

fn shift_tail_backward<B: Sysex8BuilderInternal>(
    builder: &mut B,
    payload_index_tail_start: usize,
    distance: usize,
) {
    let tail_end = payload_size(builder);
    for i in payload_index_tail_start..tail_end {
        let buffer_index = 4 * (i / capacity_without_stream_id());
        let byte_index = i % capacity_without_stream_id();
        write_datum(
            builder,
            builder.buffer()[buffer_index + (byte_index + 3) / 4].octet((byte_index + 3) % 4),
            i - distance,
        );
    }
    resize(builder, payload_size(builder) - distance);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{buffer::Ump, test_support::random_buffer::RandomBuffer};
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            Sysex8Borrowed::builder(&mut Ump::random_buffer::<8>())
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .payload(0..15)
                .build(),
            Ok(Sysex8Borrowed(&[
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
    fn builder_payload_with_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            // N.B. we need a larger than necessary buffer to account for the
            // lack of size_hint implementation from the rubbish iterator.
            Sysex8Borrowed::builder(&mut Ump::random_buffer::<30>())
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .payload(RubbishPayloadIterator::new())
                .build(),
            Ok(Sysex8Borrowed(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x542E_BB0D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x542E_BB1A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x543C_BB27,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ])),
        );
    }

    #[test]
    fn builder_replace_range_with_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            // N.B. we need a larger than necessary buffer to account for the
            // lack of size_hint implementation from the rubbish iterator.
            Sysex8Borrowed::builder(&mut [0x0; 50])
                .payload(0..30)
                .replace_payload_range(RubbishPayloadIterator::new(), 10..20)
                .build(),
            Ok(Sysex8Borrowed(&[
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x0900_0102,
                0x502E_0003,
                0x0405_0607,
                0x0809_0A0B,
                0x0C0D_0E0F,
                0x502E_0010,
                0x1112_1314,
                0x1516_1718,
                0x191A_1B1C,
                0x502E_001D,
                0x1E1F_2021,
                0x2223_2425,
                0x2627_2829,
                0x502E_002A,
                0x2B2C_2D2E,
                0x2F30_3114,
                0x1516_1718,
                0x5036_0019,
                0x1A1B_1C1D,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn builder_insert_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            // N.B. we need a larger than necessary buffer to account for the
            // lack of size_hint implementation from the rubbish iterator.
            Sysex8Borrowed::builder(&mut [0x0; 50])
                .payload(0..20)
                .insert_payload(RubbishPayloadIterator::new(), 10)
                .build(),
            Ok(Sysex8Borrowed(&[
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x0900_0102,
                0x502E_0003,
                0x0405_0607,
                0x0809_0A0B,
                0x0C0D_0E0F,
                0x502E_0010,
                0x1112_1314,
                0x1516_1718,
                0x191A_1B1C,
                0x502E_001D,
                0x1E1F_2021,
                0x2223_2425,
                0x2627_2829,
                0x502E_002A,
                0x2B2C_2D2E,
                0x2F30_310A,
                0x0B0C_0D0E,
                0x5036_000F,
                0x1011_1213,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn builder_metadata_after_payload() {
        let mut buffer = Ump::random_buffer::<8>();
        assert_eq!(
            Sysex8Borrowed::builder(&mut buffer)
                .payload(0..15)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8Borrowed(&[
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
    fn builder_complete() {
        let mut buffer = Ump::random_buffer::<4>();
        assert_eq!(
            Sysex8Borrowed::builder(&mut buffer)
                .payload(0x0..0xA)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8Borrowed(&[
                0x540B_BB00,
                0x0102_0304,
                0x0506_0708,
                0x0900_0000,
            ])),
        );
    }

    #[test]
    fn builder_payload_in_batches() {
        let mut buffer = Ump::random_buffer::<8>();
        assert_eq!(
            Sysex8Borrowed::builder(&mut buffer)
                .payload(0x0..0xA)
                .append_payload(0x0..0x5)
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .build(),
            Ok(Sysex8Borrowed(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x0900_0102,
                0x5433_BB03,
                0x0400_0000,
                0x0000_0000,
                0x0000_0000,
            ])),
        );
    }

    #[test]
    fn from_data_inconsistent_groups() {
        assert_eq!(
            Sysex8Borrowed::from_data(&[
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
    fn from_data_inconsistent_status() {
        assert_eq!(
            Sysex8Borrowed::from_data(&[0x5011_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn from_data_invalid_message() {
        assert_eq!(
            Sysex8Borrowed::from_data(&[0x0001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
        assert_eq!(
            Sysex8Borrowed::from_data(&[0x5000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn payload() {
        let mut buffer = [0x0; 15];
        let message_group = Sysex8Borrowed(&[
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
    fn payload_count() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_count_start_from_one() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_6th() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_0th() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_nth_last() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_nth_past_the_end() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_nth_consumes_nth() {
        let message_group = Sysex8Borrowed(&[
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
    fn payload_nth_last_none_left() {
        let message_group = Sysex8Borrowed(&[
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

#[cfg(feature = "std")]
#[cfg(test)]
mod std_tests {
    use super::*;
    use crate::test_support::debug;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            Sysex8Message::builder()
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .payload(0..15)
                .build(),
            Ok(Sysex8Message::Owned(Sysex8Owned(std::vec![
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5433_BB0D,
                0x0E00_0000,
                0x0000_0000,
                0x0000_0000,
            ]))),
        );
    }

    #[test]
    fn builder_payload_with_rubbish_payload_iterator() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        assert_eq!(
            Sysex8Message::builder()
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .payload(RubbishPayloadIterator::new())
                .build(),
            Ok(Sysex8Message::Owned(Sysex8Owned(std::vec![
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x542E_BB0D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x542E_BB1A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x543C_BB27,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ]))),
        );
    }

    #[test]
    fn payload() {
        let actual: std::vec::Vec<u8> = Sysex8Message::from_data(&[
            0x541E_BB00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5433_BB0D,
            0x0E00_0000,
            0x0000_0000,
            0x0000_0000,
        ])
        .unwrap()
        .payload()
        .collect();
        let expected: std::vec::Vec<u8> = (0..15).into_iter().collect();
        assert_eq!(debug::ByteData(&actual), debug::ByteData(&expected));
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex8Message::from_data(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5433_BB0D,
                0x0E00_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .group(),
            u4::new(0x4),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Sysex8Message::from_data(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5433_BB0D,
                0x0E00_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .stream_id(),
            0xBB,
        );
    }

    #[test]
    fn into_owned() {
        assert_eq!(
            Sysex8Message::from_data(&[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5433_BB0D,
                0x0E00_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .into_owned(),
            Sysex8Owned::builder()
                .group(u4::new(0x4))
                .stream_id(0xBB)
                .payload(0..15)
                .build()
                .unwrap(),
        );
    }
}
