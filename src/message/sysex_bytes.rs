use crate::*;

#[derive(midi2_proc::BytesDebug, Clone, PartialEq, Eq)]
pub struct Sysex7BytesBorrowed<'a>(&'a [u8]);

pub struct Sysex7BytesBorrowedBuilder<'a>(Result<&'a mut [u8]>, usize);

#[cfg(feature = "std")]
#[derive(midi2_proc::BytesDebug, Clone, PartialEq, Eq)]
pub struct Sysex7BytesOwned(std::vec::Vec<u8>);

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Eq)]
pub struct Sysex7BytesBuilder<M: core::convert::From<Sysex7BytesOwned>>(
    Result<std::vec::Vec<u8>>,
    core::marker::PhantomData<M>,
);

#[derive(derive_more::From, Debug, Clone, PartialEq, Eq)]
pub enum Sysex7BytesMessage<'a> {
    #[cfg(feature = "std")]
    Owned(Sysex7BytesOwned),
    Borrowed(Sysex7BytesBorrowed<'a>),
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Sysex7BytesBorrowed<'a> {
    type Owned = Sysex7BytesOwned;
    fn into_owned(self) -> Self::Owned {
        Sysex7BytesOwned(self.0.to_vec())
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Sysex7BytesMessage<'a> {
    type Owned = Sysex7BytesOwned;
    fn into_owned(self) -> Sysex7BytesOwned {
        use Sysex7BytesMessage::*;
        match self {
            Owned(m) => m,
            Borrowed(m) => m.into_owned(),
        }
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7BytesBorrowed<'a> {
    type PayloadIterator = core::iter::Cloned<core::slice::Iter<'a, u8>>;
    fn payload(&self) -> Self::PayloadIterator {
        self.0[1..self.0.len() - 1].iter().cloned()
    }
}

#[cfg(feature = "std")]
impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7BytesOwned {
    type PayloadIterator = core::iter::Cloned<core::slice::Iter<'a, u8>>;
    fn payload(&'b self) -> Self::PayloadIterator {
        self.0[1..self.0.len() - 1].iter().cloned()
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7BytesMessage<'a> {
    type PayloadIterator = core::iter::Cloned<core::slice::Iter<'a, u8>>;
    fn payload(&'b self) -> Self::PayloadIterator {
        use Sysex7BytesMessage::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.payload(),
            Borrowed(m) => m.payload(),
        }
    }
}

impl<'a> Sysex7BytesBorrowed<'a> {
    pub fn builder(buffer: &'a mut [u8]) -> Sysex7BytesBorrowedBuilder<'a> {
        Sysex7BytesBorrowedBuilder::new(buffer)
    }
}

#[cfg(feature = "std")]
impl Sysex7BytesOwned {
    pub fn builder() -> Sysex7BytesBuilder<Self> {
        Sysex7BytesBuilder::new()
    }
}

#[cfg(feature = "std")]
impl<'a> Sysex7BytesMessage<'a> {
    pub fn builder() -> Sysex7BytesBuilder<Self> {
        Sysex7BytesBuilder::new()
    }
}

impl<'a> FromByteData<'a> for Sysex7BytesBorrowed<'a> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        if buffer.len() < 2 || buffer[0] != 0xF0 || buffer[buffer.len() - 1] != 0xF7 {
            Err(Error::InvalidData)
        } else {
            Ok(())
        }
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }
}

impl<'a> FromByteData<'a> for Sysex7BytesMessage<'a> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        Sysex7BytesBorrowed::validate_byte_data(buffer)
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self {
        Self::Borrowed(Sysex7BytesBorrowed::from_byte_data_unchecked(buffer))
    }
}

impl<'a> ByteData for Sysex7BytesBorrowed<'a> {
    fn byte_data(&self) -> &[u8] {
        self.0
    }
}

#[cfg(feature = "std")]
impl ByteData for Sysex7BytesOwned {
    fn byte_data(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> ByteData for Sysex7BytesMessage<'a> {
    fn byte_data(&self) -> &[u8] {
        use Sysex7BytesMessage::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.byte_data(),
            Borrowed(m) => m.byte_data(),
        }
    }
}

impl<'a> Sysex7BytesBorrowedBuilder<'a> {
    fn grow(&mut self) {
        if let Ok(buffer) = &self.0 {
            if buffer.len() < self.1 + 1 {
                self.0 = Err(Error::BufferOverflow);
            } else {
                self.1 += 1;
            }
        }
    }
    pub fn new(buffer: &'a mut [u8]) -> Self {
        if buffer.len() < 2 {
            Self(Err(Error::BufferOverflow), 0)
        } else {
            buffer[0] = 0xF0;
            Self(Ok(buffer), 1)
        }
    }
    pub fn build(mut self) -> Result<Sysex7BytesBorrowed<'a>> {
        if self.0.is_ok() {
            self.grow();
        }
        match self.0 {
            Ok(buffer) => {
                buffer[self.1 - 1] = 0xF7;
                Ok(Sysex7BytesBorrowed(&buffer[..self.1]))
            }
            Err(e) => Err(e.clone()),
        }
    }
    pub fn payload<I: core::iter::Iterator<Item = u7>>(mut self, data: I) -> Self {
        for d in data {
            self.grow();
            match &mut self.0 {
                Ok(buffer) => {
                    buffer[self.1 - 1] = d.into();
                }
                Err(_) => {
                    break;
                }
            }
        }
        self
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7BytesOwned>> core::default::Default for Sysex7BytesBuilder<M> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<Sysex7BytesOwned>> Sysex7BytesBuilder<M> {
    pub fn new() -> Self {
        Self(Ok(std::vec![0xF0_u8]), Default::default())
    }
    pub fn build(&self) -> Result<M> {
        match self.0.clone() {
            Ok(mut cache) => {
                cache.push(0xF7);
                Ok(Sysex7BytesOwned(cache).into())
            }
            Err(e) => Err(e),
        }
    }
    pub fn payload<I: core::iter::Iterator<Item = u7>>(&mut self, data: I) -> &mut Self {
        if let Ok(cache) = &mut self.0 {
            for d in data {
                cache.push(d.into());
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::Bytes,
        util::{RandomBuffer, Truncate},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            Sysex7BytesBorrowed::builder(&mut Bytes::random_buffer::<22>())
                .payload((0u8..20u8).map(|v| v.truncate()))
                .build(),
            Ok(Sysex7BytesBorrowed(&[
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ])),
        );
    }

    #[test]
    fn builder_buffer_overflow() {
        assert_eq!(
            Sysex7BytesBorrowed::builder(&mut Bytes::random_buffer::<21>())
                .payload((0u8..20u8).map(|v| v.truncate()))
                .build(),
            Err(Error::BufferOverflow),
        );
    }

    #[test]
    fn from_data_missing_start() {
        assert_eq!(
            Sysex7BytesBorrowed::from_byte_data(&[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn from_data_missing_end() {
        assert_eq!(
            Sysex7BytesBorrowed::from_byte_data(&[
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
            ]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn payload() {
        let actual: [u8; 20] = {
            let data = [
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ];
            let message = Sysex7BytesBorrowed::from_byte_data(&data).unwrap();
            let payload = message.payload();
            let mut buffer: [u8; 20] = Default::default();
            for (i, d) in payload.enumerate() {
                buffer[i] = d;
            }
            buffer
        };
        let expected: [u8; 20] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
        ];
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod std_tests {
    use super::*;
    use crate::util::{debug, Truncate};
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            Sysex7BytesMessage::builder()
                .payload((0u8..20u8).map(|v| v.truncate()))
                .build(),
            Ok(Sysex7BytesMessage::Owned(Sysex7BytesOwned(std::vec![
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ]))),
        );
    }

    #[test]
    fn payload() {
        let expected: std::vec::Vec<u8> = std::vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
        ];
        let actual: std::vec::Vec<u8> = Sysex7BytesMessage::from_byte_data(&[
            0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
            0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
        ])
        .unwrap()
        .payload()
        .collect();
        assert_eq!(debug::ByteData(&expected), debug::ByteData(&actual));
    }

    #[test]
    fn into_owned() {
        assert_eq!(
            Sysex7BytesMessage::from_byte_data(&[
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ])
            .unwrap()
            .into_owned(),
            Sysex7BytesOwned::builder()
                .payload((0u8..20u8).map(|v| v.truncate()))
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn byte_data() {
        assert_eq!(
            Sysex7BytesMessage::builder()
                .payload((0u8..20u8).map(|v| v.truncate()))
                .build()
                .unwrap()
                .byte_data(),
            &[
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ],
        );
    }
}
