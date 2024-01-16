use crate::{
    error::Error,
    message::helpers as message_helpers,
    result::Result,
    util::{BitOps, Truncate},
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

#[derive(midi2_attr::UmpDebug, Clone, PartialEq, Eq)]
pub struct Sysex7PartialBorrowed<'a>(&'a [u32]);

#[derive(midi2_attr::UmpDebug, Clone, PartialEq, Eq)]
pub struct Sysex7PartialOwned([u32; 4]);

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
pub enum Sysex7PartialMessage<'a> {
    Owned(Sysex7PartialOwned),
    Borrowed(Sysex7PartialBorrowed<'a>),
}

pub trait Sysex7 {
    fn status(&self) -> Status;
}

impl<'a> IntoOwned for Sysex7PartialBorrowed<'a> {
    type Owned = Sysex7PartialOwned;
    fn into_owned(self) -> Self::Owned {
        let mut buffer: [u32; 4] = Default::default();
        buffer[..].copy_from_slice(self.0);
        Sysex7PartialOwned(buffer)
    }
}

impl<'a> IntoOwned for Sysex7PartialMessage<'a> {
    type Owned = Sysex7PartialOwned;
    fn into_owned(self) -> Sysex7PartialOwned {
        let mut buffer: [u32; 4] = Default::default();
        buffer[..].copy_from_slice(self.data());
        Sysex7PartialOwned(buffer)
    }
}

impl<'a> Data for Sysex7PartialBorrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0
    }
}

impl Data for Sysex7PartialOwned {
    fn data(&self) -> &[u32] {
        &self.0[..2]
    }
}

impl<'a> FromData<'a> for Sysex7PartialBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Sysex7PartialBorrowed(&data[..2])
    }
    fn validate_data(data: &[u32]) -> Result<()> {
        validate_buffer(data)?;
        validate_type(data)?;
        status_from_packet(data)?;
        validate_data(data)?;
        Ok(())
    }
}

impl<'a> FromData<'a> for Sysex7PartialMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        Sysex7PartialBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Sysex7PartialBorrowed::from_data_unchecked(buffer).into()
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

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7PartialOwned {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&self) -> PayloadIterator {
        PayloadIterator {
            data: &self.0[..],
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a> Sysex7PartialBorrowed<'a> {
    const OP_CODE: u4 = u4::new(0x3);
    pub fn status(&self) -> Status {
        status_from_packet(self.0).expect("valid status")
    }
    pub fn builder(buffer: &'a mut [u32]) -> Sysex7PartialBorrowedBuilder {
        Sysex7PartialBorrowedBuilder::new(buffer)
    }
}

impl Sysex7PartialOwned {
    pub fn status(&self) -> Status {
        status_from_packet(&self.0).expect("valid status")
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7PartialBorrowed<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&self) -> PayloadIterator {
        PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a> Grouped for Sysex7PartialBorrowed<'a> {}

impl Grouped for Sysex7PartialOwned {}

pub struct Sysex7PartialBorrowedBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> Sysex7PartialBorrowedBuilder<'a> {
    pub fn status(mut self, s: Status) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(
                2,
                match s {
                    Status::Complete => u4::new(0x0),
                    Status::Begin => u4::new(0x1),
                    Status::Continue => u4::new(0x2),
                    Status::End => u4::new(0x3),
                },
            );
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u7>>(mut self, mut data: I) -> Self {
        if let Ok(buffer) = &mut self.0 {
            let mut count = 0_u8;
            for i in 0_usize..6_usize {
                if let Some(v) = data.next() {
                    buffer[(i + 2) / 4].set_octet((i + 2) % 4, v.into());
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
    pub fn new(buffer: &'a mut [u32]) -> Self {
        if buffer.len() >= 2 {
            for b in buffer.iter_mut() {
                *b = 0x0;
            }
            message_helpers::write_type_to_packet(Sysex7PartialBorrowed::OP_CODE, buffer);
            Self(Ok(buffer))
        } else {
            Self(Err(Error::BufferOverflow))
        }
    }
    pub fn build(self) -> Result<Sysex7PartialBorrowed<'a>> {
        match self.0 {
            Ok(buffer) => Ok(Sysex7PartialBorrowed(buffer)),
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
}

fn validate_type(p: &[u32]) -> Result<()> {
    if p[0].nibble(0) != Sysex7PartialBorrowed::OP_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn status_from_packet(p: &[u32]) -> Result<Status> {
    status_from_nibble(p[0].nibble(2))
}

fn status_from_nibble(n: u4) -> Result<Status> {
    match u8::from(n) {
        0x0 => Ok(Status::Complete),
        0x1 => Ok(Status::Begin),
        0x2 => Ok(Status::Continue),
        0x3 => Ok(Status::End),
        _ => Err(Error::InvalidData),
    }
}

fn validate_buffer(buffer: &[u32]) -> Result<()> {
    if buffer.len() == 2 {
        Ok(())
    } else {
        Err(Error::BufferOverflow)
    }
}

fn validate_data(p: &[u32]) -> Result<()> {
    let n: usize = u8::from(p[0].nibble(3)).into();
    if n > 6 {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex7Borrowed<'a>(&'a [u32]);

#[derive(Clone, PartialEq, Eq)]
#[cfg(feature = "std")]
pub struct Sysex7Owned(std::vec::Vec<u32>);

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Debug, Clone, PartialEq, Eq)]
pub enum Sysex7Message<'a> {
    #[cfg(feature = "std")]
    Owned(Sysex7Owned),
    Borrowed(Sysex7Borrowed<'a>),
}

#[cfg(feature = "std")]
impl core::fmt::Debug for Sysex7Owned {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        format_message(f, self.0.iter(), "Sysex7Owned")
    }
}

impl<'a> core::fmt::Debug for Sysex7Borrowed<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        format_message(f, self.0.iter(), "Sysex7Owned")
    }
}

fn format_message<'a, I: core::iter::Iterator<Item = &'a u32>>(
    fmt: &mut core::fmt::Formatter,
    data: I,
    name: &str,
) -> core::fmt::Result {
    fmt.write_fmt(format_args!("{}(", name))?;
    let mut iter = data.peekable();
    while let Some(v) = iter.next() {
        fmt.write_fmt(format_args!("{v:#010X}"))?;
        if iter.peek().is_some() {
            fmt.write_str(",")?;
        }
    }
    fmt.write_str(")")
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
            Sysex7PartialBorrowed::validate_data(chunk)?;
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

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7Borrowed<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&self) -> Self::PayloadIterator {
        Self::PayloadIterator {
            data: self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

#[cfg(feature = "std")]
impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7Owned {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&'a self) -> Self::PayloadIterator {
        Self::PayloadIterator {
            data: &self.0,
            message_index: 0,
            payload_index: 0,
        }
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7Message<'a> {
    type PayloadIterator = PayloadIterator<'a>;
    fn payload(&'a self) -> Self::PayloadIterator {
        use Sysex7Message::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.payload(),
            Borrowed(m) => m.payload(),
        }
    }
}

pub struct Sysex7MessageGroupIterator<'a>(core::slice::ChunksExact<'a, u32>);

impl<'a> core::iter::Iterator for Sysex7MessageGroupIterator<'a> {
    type Item = Sysex7PartialBorrowed<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Sysex7PartialBorrowed)
    }
}

pub struct Sysex7BorrowedBuilder<'a> {
    buffer: &'a mut [u32],
    size: usize,
    error: Option<Error>,
    group: u4,
}

#[cfg(feature = "std")]
pub struct Sysex7Builder<M: core::convert::From<Sysex7Owned>> {
    buffer: std::vec::Vec<u32>,
    error: Option<Error>,
    group: u4,
    _phantom_message: core::marker::PhantomData<M>,
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7Owned>> core::default::Default for Sysex7Builder<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Sysex7BorrowedBuilder<'a> {
    // If there is no room in the buffer for an additional message
    // then the error field will be populated with a buffer
    // overflow error.
    fn grow(&mut self) {
        if self.buffer.len() < 2 * (self.size + 1) {
            self.error = Some(Error::BufferOverflow);
            return;
        }
        grow(self.buffer, self.size, self.group);
        self.size += 1;
    }
    pub fn build(self) -> Result<Sysex7Borrowed<'a>> {
        if let Some(e) = &self.error {
            return Err(e.clone());
        }
        Ok(Sysex7Borrowed(&self.buffer[..2 * self.size]))
    }

    pub fn new(buffer: &'a mut [u32]) -> Self {
        Sysex7BorrowedBuilder {
            buffer,
            size: 0,
            error: None,
            group: u4::new(0x0),
        }
    }
    pub fn group(mut self, g: u4) -> Self {
        if self.error.is_some() || self.group == g {
            return self;
        }
        self.group = g;
        for chunk in self.buffer[..self.size * 2].chunks_exact_mut(2) {
            chunk[0].set_nibble(1, g);
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u7>>(mut self, mut iter: I) -> Self {
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
            if current_size == u4::new(6) {
                self.grow();
                if self.error.is_some() {
                    return self;
                }
                0
            } else {
                u8::from(current_size) as usize
            }
        };

        self.buffer[message_index(self.size) + (data_start + 2) / 4]
            .set_octet((2 + data_start) % 4, first.into());
        increment_message_size(self.buffer, message_index(self.size));

        let mut stop = false;
        for i in (data_start + 1)..6 {
            match iter.next() {
                Some(v) => {
                    let index = message_index(self.size);
                    self.buffer[index + (i + 2) / 4].set_octet((2 + i) % 4, v.into());
                    increment_message_size(self.buffer, index);
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
        Sysex7Builder {
            buffer: std::vec::Vec::new(),
            error: None,
            group: u4::new(0x0),
            _phantom_message: Default::default(),
        }
    }
    pub fn group(&mut self, g: u4) -> &mut Self {
        if self.error.is_some() || self.group == g {
            return self;
        }
        self.group = g;
        let size = self.size();
        for chunk in self.buffer[..size * 2].chunks_exact_mut(2) {
            chunk[0].set_nibble(1, g);
        }
        self
    }
    pub fn payload<I: core::iter::Iterator<Item = u7>>(&mut self, mut iter: I) -> &mut Self {
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
            if current_size == u4::new(6) {
                self.grow();
                0
            } else {
                u8::from(current_size) as usize
            }
        };

        let size = self.size();
        self.buffer[message_index(size) + (data_start + 2) / 4]
            .set_octet((2 + data_start) % 4, first.into());
        increment_message_size(&mut self.buffer, message_index(size));

        let mut stop = false;
        for i in (data_start + 1)..6 {
            match iter.next() {
                Some(v) => {
                    let index = message_index(self.size());
                    self.buffer[index + (i + 2) / 4].set_octet((2 + i) % 4, v.into());
                    increment_message_size(&mut self.buffer, index);
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

    fn grow(&mut self) {
        let size = self.size();
        self.buffer.extend_from_slice(&[0x0; 2]);
        grow(&mut self.buffer, size, self.group)
    }
    fn size(&self) -> usize {
        self.buffer.len() / 2
    }
}

// The point in the buffer where the last most message begins.
fn message_index(size: usize) -> usize {
    2 * (size - 1)
}

// The size of the message in the group at the given index.
// N.B. it is up to the caller to ensure the index is valid.
fn message_size(buffer: &[u32], message_index: usize) -> u4 {
    buffer[message_index].nibble(3)
}

// Increment the size of the message at the given index.
// N.B. it is up to the caller to make sure the index is valid.
fn increment_message_size(buffer: &mut [u32], message_index: usize) {
    let new_value = buffer[message_index].nibble(3) + u4::new(1);
    buffer[message_index].set_nibble(3, new_value);
}

// Adds an additional sysex message to the end of the group.
// Updates the status of the previous message so that the
// group statuses together remain valid.
fn grow(buffer: &mut [u32], size: usize, group: u4) {
    {
        let mut builder = Sysex7PartialBorrowed::builder(&mut buffer[2 * size..2 * (size + 1)]);
        builder = builder.group(group);
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
        let mut prev_builder =
            Sysex7PartialBorrowedBuilder(Ok(&mut buffer[2 * (size - 1)..2 * size]));
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
    use crate::{buffer::Ump, util::RandomBuffer};

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Sysex7PartialBorrowed::from_data(&[0x2000_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Sysex7PartialBorrowed::from_data(&[0x30A0_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Sysex7PartialBorrowed::from_data(&[0x3009_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn builder() {
        let mut buffer = Ump::random_buffer::<2>();
        assert_eq!(
            Sysex7PartialBorrowed::builder(&mut buffer)
                .group(u4::new(0x1))
                .status(Status::Begin)
                .payload(
                    [u7::new(0x12), u7::new(0x34), u7::new(0x56),]
                        .iter()
                        .copied()
                )
                .build(),
            Ok(Sysex7PartialBorrowed(&[0x3113_1234, 0x5600_0000,])),
        );
    }

    #[test]
    fn builder_invalid_payload() {
        let mut buffer = Ump::random_buffer::<2>();
        assert_eq!(
            Sysex7PartialBorrowed::builder(&mut buffer)
                .payload([u7::default(); 7].iter().copied())
                .build(),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex7PartialBorrowed::from_data(&[0x3C00_0000, 0x0,])
                .unwrap()
                .group(),
            u4::new(0xC),
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            Sysex7PartialBorrowed::from_data(&[0x3020_0000, 0x0,])
                .unwrap()
                .status(),
            Status::Continue,
        );
    }

    #[test]
    fn data() {
        assert_eq!(
            Sysex7PartialBorrowed::from_data(&[0x3004_1234, 0x5678_0000,])
                .unwrap()
                .data(),
            &[0x30041234, 0x5678_0000]
        );
    }

    #[test]
    fn payload() {
        let message = Sysex7PartialBorrowed::from_data(&[0x3004_1234, 0x5678_0000]).unwrap();
        let mut buffer = [0x0; 4];
        for (i, v) in message.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn group_builder() {
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
    fn group_builder_group_after_payload() {
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
    fn group_builder_complete() {
        assert_eq!(
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<2>())
                .group(u4::new(0x4))
                .payload((0..4).map(u7::new))
                .build(),
            Ok(Sysex7Borrowed(&[0x3404_0001, 0x0203_0000,])),
        );
    }

    #[test]
    fn group_builder_payload_in_batches() {
        assert_eq!(
            Sysex7Borrowed::builder(&mut Ump::random_buffer::<4>())
                .payload((0..4).map(u7::new))
                .payload((4..8).map(u7::new))
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
    fn group_from_data_inconsistent_groups() {
        assert_eq!(
            Sysex7Borrowed::from_data(&[0x3010_0000, 0x0000_0000, 0x3130_0000, 0x0000_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group_from_data_incompatible_buffer_size() {
        assert_eq!(
            Sysex7Borrowed::from_data(&[0x3010_0000, 0x0000_0000, 0x3030_0000,]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group_from_data_complete() {
        assert!(Sysex7Borrowed::from_data(&[0x3000_0000, 0x0000_0000,]).is_ok());
    }

    #[test]
    fn group_from_data_invalid_message() {
        assert!(Sysex7Borrowed::from_data(&[0x1000_0000, 0x0000_0000,]).is_err());
    }

    #[test]
    fn group_payload() {
        let mut buffer = [0x0; 8];
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        for (i, v) in message_group.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[0, 1, 2, 3, 4, 5, 6, 7,])
    }

    #[test]
    fn group_payload_count() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        assert_eq!(message_group.payload().count(), 8);
    }

    #[test]
    fn group_payload_count_start_from_one() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.count(), 7);
    }

    #[test]
    fn group_payload_4th() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(4), Some(5));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn group_payload_0th() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(0), Some(1));
    }

    #[test]
    fn group_payload_nth_last() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(6), Some(7));
    }

    #[test]
    fn group_payload_nth_past_the_end() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.next();
        assert_eq!(payload.nth(7), None);
    }

    #[test]
    fn group_payload_nth_last_none_left() {
        let message_group = Sysex7Borrowed(&[0x3014_0001, 0x0203_0000, 0x3034_0405, 0x0607_0000]);
        let mut payload = message_group.payload();
        payload.nth(7);
        assert_eq!(payload.next(), None);
    }

    #[test]
    fn group_payload_from_sysex7_discovery() {
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
    use crate::util::debug;
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
