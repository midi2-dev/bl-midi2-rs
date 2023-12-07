use crate::{
    error::Error,
    message::helpers,
    result::Result,
    util::{debug, BitOps, Truncate},
    *,
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
pub struct Sysex8PartialBorrowed<'a>(&'a [u32]);

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex8PartialOwned([u32; 4]);

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
pub enum Sysex8PartialMessage<'a> {
    Owned(Sysex8PartialOwned),
    Borrowed(Sysex8PartialBorrowed<'a>),
}

impl<'a> IntoOwned for Sysex8PartialBorrowed<'a> {
    type Owned = Sysex8PartialOwned;
    fn into_owned(self) -> Self::Owned {
        let mut buffer: [u32; 4] = Default::default();
        buffer[..].copy_from_slice(self.0);
        Sysex8PartialOwned(buffer)
    }
}

impl<'a> IntoOwned for Sysex8PartialMessage<'a> {
    type Owned = Sysex8PartialOwned;
    fn into_owned(self) -> Sysex8PartialOwned {
        let mut buffer: [u32; 4] = Default::default();
        buffer[..].copy_from_slice(self.data());
        Sysex8PartialOwned(buffer)
    }
}

pub trait Sysex8 {
    fn status(&self) -> Status;
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex8PartialBorrowed<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&self) -> Self::PayloadIterator {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex8PartialOwned
where
    Self: 'b,
{
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&'b self) -> Self::PayloadIterator {
        PayloadIterator {
            data: &self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a> Sysex8 for Sysex8PartialBorrowed<'a> {
    fn status(&self) -> Status {
        try_status_from_packet(self.0).expect("Valid status")
    }
}

impl Sysex8 for Sysex8PartialOwned {
    fn status(&self) -> Status {
        try_status_from_packet(&self.0).expect("Valid status")
    }
}

impl<'a> Sysex8PartialBorrowed<'a> {
    const OP_CODE: u4 = u4::new(0x5);
    pub fn builder(buffer: &'a mut [u32]) -> Sysex8BuilderBorrowed<'a> {
        Sysex8BuilderBorrowed::new(buffer)
    }
}

impl<'a> Data for Sysex8PartialBorrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0
    }
}

impl Data for Sysex8PartialOwned {
    fn data(&self) -> &[u32] {
        &self.0
    }
}

impl<'a> FromData<'a> for Sysex8PartialBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Sysex8PartialBorrowed(&data[..4])
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        validate_buffer(buffer)?;
        let Ok(status) = try_status_from_packet(buffer) else {
            return Err(Error::InvalidData);
        };
        validate_data(buffer, status)?;
        validate_packet(buffer)?;
        Ok(())
    }
}

impl<'a> FromData<'a> for Sysex8PartialMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        Sysex8PartialBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Sysex8PartialBorrowed::from_data_unchecked(buffer).into()
    }
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

impl<'a> Grouped for Sysex8PartialBorrowed<'a> {}

impl Grouped for Sysex8PartialOwned {}

impl<'a> Streamed for Sysex8PartialBorrowed<'a> {}
impl Streamed for Sysex8PartialOwned {}

debug::message_debug_impl!(Sysex8PartialBorrowed);
debug::message_debug_impl_owned!(Sysex8PartialOwned);

pub struct Sysex8BuilderBorrowed<'a>(Result<&'a mut [u32]>);

impl<'a> Sysex8BuilderBorrowed<'a> {
    /// When called with `Status::UnexpectedEnd(_)` the payload buffer
    /// will be filled with zeros accordingly.
    pub fn status(mut self, s: Status) -> Self {
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
    pub fn payload<'b, I: core::iter::Iterator<Item = &'b u8>>(mut self, mut data: I) -> Self {
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
    pub fn group(mut self, g: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
    pub fn stream_id(mut self, id: u8) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, id);
        }
        self
    }
    fn build(self) -> Result<Sysex8PartialBorrowed<'a>> {
        match self.0 {
            Ok(buffer) => Ok(Sysex8PartialBorrowed(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        if buffer.len() >= 4 {
            buffer[..4].copy_from_slice(&[0x0; 4]);
            helpers::write_type_to_packet(Sysex8PartialBorrowed::OP_CODE, buffer);
            buffer[0].set_nibble(3, u4::new(0x1)); // stream id
            Self(Ok(&mut buffer[..4]))
        } else {
            Self(Err(Error::BufferOverflow))
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
    if p[0].nibble(0) != Sysex8PartialBorrowed::OP_CODE {
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
pub struct Sysex8Borrowed<'a>(&'a [u32]);

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Eq)]
pub struct Sysex8Owned(std::vec::Vec<u32>);

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Debug, Clone, PartialEq, Eq)]
pub enum Sysex8Message<'a> {
    #[cfg(feature = "std")]
    Owned(Sysex8Owned),
    Borrowed(Sysex8Borrowed<'a>),
}

debug::message_debug_impl!(Sysex8Borrowed);

#[cfg(feature = "std")]
debug::message_debug_impl_owned!(Sysex8Owned);

impl<'a> Sysex8Borrowed<'a> {
    pub fn messages(&self) -> Sysex8MessageGroupIterator {
        Sysex8MessageGroupIterator(self.0.chunks_exact(4))
    }
    pub fn builder(buffer: &'a mut [u32]) -> Sysex8MessageGroupBuilderBorrowed<'a> {
        Sysex8MessageGroupBuilderBorrowed::new(buffer)
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
            Sysex8PartialBorrowed::validate_data(chunk)?;
        }
        helpers::sysex_group_consistent_groups(buffer, 4)?;
        helpers::validate_sysex_group_statuses(
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

pub struct Sysex8MessageGroupIterator<'a>(core::slice::ChunksExact<'a, u32>);

impl<'a> core::iter::Iterator for Sysex8MessageGroupIterator<'a> {
    type Item = Sysex8PartialBorrowed<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Sysex8PartialBorrowed)
    }
}

#[cfg(feature = "std")]
pub struct Sysex8Builder<M: core::convert::From<Sysex8Owned>> {
    buffer: std::vec::Vec<u32>,
    error: Option<Error>,
    group: u4,
    stream_id: u8,
    _phantom_message: core::marker::PhantomData<M>,
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> Sysex8Builder<M> {
    pub fn group(mut self, g: u4) -> Self {
        if self.error.is_some() || self.group == g {
            return self;
        }
        self.group = g;
        let sz = self.size();
        set_group(&mut self.buffer, sz, g);
        self
    }

    pub fn stream_id(mut self, id: u8) -> Self {
        if self.error.is_some() || self.stream_id == id {
            return self;
        }
        self.stream_id = id;
        let sz = self.size();
        for chunk in self.buffer[..sz * 4].chunks_exact_mut(4) {
            chunk[0].set_octet(2, id);
        }
        self
    }

    pub fn payload<I: core::iter::Iterator<Item = u8>>(mut self, mut iter: I) -> Self {
        if self.error.is_some() {
            return self;
        }

        let Some(first) = iter.next() else {
            return self;
        };

        if self.size() == 0 {
            self.grow();
        }

        let data_start: usize = {
            let current_size = message_size(&self.buffer, message_index(self.size()));
            if current_size == u4::new(14) {
                self.grow();
                0
            } else {
                u8::from(current_size) as usize - 1
            }
        };

        let message_index = message_index(self.size());
        let mut stop = false;

        set_datum(&mut self.buffer, message_index, data_start, first);

        for i in (data_start + 1)..13 {
            match iter.next() {
                Some(v) => {
                    set_datum(&mut self.buffer, message_index, i, v);
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

    pub fn new() -> Self {
        Sysex8Builder {
            buffer: std::vec::Vec::new(),
            error: None,
            group: u4::new(0x0),
            stream_id: 0x0,
            _phantom_message: Default::default(),
        }
    }

    pub fn build(self) -> Result<M> {
        let None = &self.error else {
            return Err(Error::InvalidData);
        };
        Ok(Sysex8Owned(self.buffer).into())
    }

    fn grow(&mut self) {
        let size = self.size();
        self.buffer.extend_from_slice(&[0x0; 4]);
        grow(&mut self.buffer, size, self.group, self.stream_id);
    }

    fn size(&self) -> usize {
        self.buffer.len() / 4
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex8Owned>> core::default::Default for Sysex8Builder<M> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Sysex8MessageGroupBuilderBorrowed<'a> {
    buffer: &'a mut [u32],
    size: usize,
    error: Option<Error>,
    group: u4,
    stream_id: u8,
}

impl<'a> Sysex8MessageGroupBuilderBorrowed<'a> {
    fn grow(&mut self) {
        if self.buffer.len() < 4 * (self.size + 1) {
            self.error = Some(Error::BufferOverflow);
            return;
        }

        grow(self.buffer, self.size, self.group, self.stream_id);

        self.size += 1;
    }

    pub fn group(mut self, g: u4) -> Self {
        if self.error.is_some() || self.group == g {
            return self;
        }
        self.group = g;
        set_group(self.buffer, self.size, g);
        self
    }

    pub fn stream_id(mut self, id: u8) -> Self {
        if self.error.is_some() || self.stream_id == id {
            return self;
        }
        self.stream_id = id;
        for chunk in self.buffer[..self.size * 4].chunks_exact_mut(4) {
            chunk[0].set_octet(2, id);
        }
        self
    }

    pub fn payload<I: core::iter::Iterator<Item = u8>>(mut self, mut iter: I) -> Self {
        if self.error.is_some() {
            return self;
        }

        let Some(first) = iter.next() else {
            return self;
        };

        if self.size == 0 {
            self.grow();
            if self.error.is_some() {
                return self;
            }
        }

        let data_start: usize = {
            let current_size = message_size(self.buffer, message_index(self.size));
            if current_size == u4::new(14) {
                self.grow();
                if self.error.is_some() {
                    return self;
                }
                0
            } else {
                u8::from(current_size) as usize - 1
            }
        };

        let message_index = message_index(self.size);
        let mut stop = false;

        set_datum(self.buffer, message_index, data_start, first);

        for i in (data_start + 1)..13 {
            match iter.next() {
                Some(v) => {
                    set_datum(self.buffer, message_index, i, v);
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

    pub fn new(buffer: &'a mut [u32]) -> Self {
        Sysex8MessageGroupBuilderBorrowed {
            buffer,
            size: 0,
            error: None,
            group: u4::new(0x0),
            stream_id: 0x0,
        }
    }

    pub fn build(self) -> Result<Sysex8Borrowed<'a>> {
        let None = &self.error else {
            return Err(Error::InvalidData);
        };
        Ok(Sysex8Borrowed(&self.buffer[..4 * self.size]))
    }
}

fn set_group(buffer: &mut [u32], size: usize, g: u4) {
    for chunk in buffer[..size * 4].chunks_exact_mut(4) {
        chunk[0].set_nibble(1, g);
    }
}

// set the ith datum value in the message at position m in the buffer
fn set_datum(buffer: &mut [u32], message_index: usize, data_index: usize, v: u8) {
    buffer[message_index + (data_index + 3) / 4].set_octet((3 + data_index) % 4, v);
    increment_message_size(buffer, message_index);
}

// the size of the sysex message beginning at index in the buffer.
fn message_size(buffer: &[u32], message_index: usize) -> u4 {
    buffer[message_index].nibble(3)
}

// The buffer index into the last message in the group.
fn message_index(size: usize) -> usize {
    4 * (size - 1)
}

fn increment_message_size(buffer: &mut [u32], message_index: usize) {
    let new_value = message_size(buffer, message_index) + u4::new(1);
    buffer[message_index].set_nibble(3, new_value);
}

fn grow(buffer: &mut [u32], size: usize, group: u4, stream_id: u8) {
    {
        let mut builder = Sysex8PartialBorrowed::builder(&mut buffer[4 * size..4 * (size + 1)]);
        builder = builder.group(group);
        builder = builder.stream_id(stream_id);
        match size {
            0 => {
                builder = builder.status(Status::Complete);
            }
            _ => {
                builder = builder.status(Status::End);
            }
        }
        builder.build().expect("successful message build");
    }

    if size != 0 {
        let mut prev_builder = Sysex8BuilderBorrowed(Ok(&mut buffer[4 * (size - 1)..4 * size]));
        match size {
            1 => {
                prev_builder = prev_builder.status(Status::Begin);
            }
            _ => {
                prev_builder = prev_builder.status(Status::Continue);
            }
        }
        prev_builder.build().expect("successful message build");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        let mut buffer = Ump::random_buffer::<4>();
        assert_eq!(
            Sysex8PartialBorrowed::builder(&mut buffer)
                .group(u4::new(0xA))
                .stream_id(0xC6)
                .status(Status::Continue)
                .payload([0x12, 0x34, 0x56, 0x78, 0x90].iter())
                .build(),
            Ok(Sysex8PartialBorrowed(&[0x5A26_C612, 0x3456_7890, 0x0, 0x0])),
        )
    }

    #[test]
    fn builder_large_payload() {
        let mut buffer = Ump::random_buffer::<4>();
        assert_eq!(
            Sysex8PartialBorrowed::builder(&mut buffer)
                .payload([0x0; 14].iter())
                .build(),
            Err(Error::InvalidData),
        )
    }

    #[test]
    fn must_have_stream_id() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x5000_0000, 0x0, 0x0, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x5C01_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xC),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x5001_9900, 0x0, 0x0, 0x0])
                .unwrap()
                .stream_id(),
            0x99,
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x5021_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::Continue,
        );
    }

    #[test]
    fn status_end() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x5032_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::End,
        );
    }

    #[test]
    fn status_unexpected_end_valid() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x5031_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::UnexpectedEnd(Validity::Valid),
        );
    }

    #[test]
    fn status_unexpected_end_invalid() {
        assert_eq!(
            Sysex8PartialBorrowed::from_data(&[0x503F_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .status(),
            Status::UnexpectedEnd(Validity::Invalid),
        );
    }

    #[test]
    fn payload() {
        let message =
            Sysex8PartialBorrowed::from_data(&[0x5009_FF00, 0x1122_3344, 0x5566_7700, 0x0])
                .unwrap();
        let mut buffer = [0u8; 8];
        for (i, v) in message.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]);
    }

    #[test]
    fn group_builder() {
        let mut buffer = Ump::random_buffer::<8>();
        assert_eq!(
            Sysex8Borrowed::builder(&mut buffer)
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
    fn group_builder_metadata_after_payload() {
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
    fn group_builder_complete() {
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
    fn group_builder_payload_in_batches() {
        let mut buffer = Ump::random_buffer::<8>();
        assert_eq!(
            Sysex8Borrowed::builder(&mut buffer)
                .payload(0x0..0xA)
                .payload(0x0..0x5)
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
    fn group_from_data_inconsistent_groups() {
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
    fn group_from_data_inconsistent_status() {
        assert_eq!(
            Sysex8Borrowed::from_data(&[0x5011_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group_from_data_invalid_message() {
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
    fn group_payload() {
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
    fn group_payload_count() {
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
    fn group_payload_count_start_from_one() {
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
    fn group_payload_6th() {
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
    fn group_payload_0th() {
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
    fn group_payload_nth_last() {
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
    fn group_payload_nth_past_the_end() {
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
    fn group_payload_nth_consumes_nth() {
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
    fn group_payload_nth_last_none_left() {
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
