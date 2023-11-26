use crate::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex7BytesBorrowed<'a>(&'a [u8]);

pub struct Sysex7BytesBuilder<'a>(Result<&'a mut [u8]>, usize);

impl<'a> core::fmt::Debug for Sysex7BytesBorrowed<'a> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.write_fmt(format_args!("Sysex7BytesBorrowed("))?;
        let mut iter = self.0.iter().peekable();
        while let Some(v) = iter.next() {
            fmt.write_fmt(format_args!("{v:#010X}"))?;
            if iter.peek().is_some() {
                fmt.write_str(",")?;
            }
        }
        fmt.write_str(")")
    }
}

impl<'a, 'b: 'a> Sysex<'a, 'b> for Sysex7BytesBorrowed<'a> {
    type PayloadIterator = core::iter::Cloned<core::slice::Iter<'a, u8>>;
    fn payload(&self) -> Self::PayloadIterator {
        self.0[1..self.0.len() - 1].iter().cloned()
    }
}

impl<'a> Sysex7BytesBorrowed<'a> {
    pub fn builder(buffer: &'a mut [u8]) -> Sysex7BytesBuilder<'a> {
        Sysex7BytesBuilder::new(buffer)
    }
    pub fn validate_data(buffer: &'a [u8]) -> Result<()> {
        if buffer.len() < 2 || buffer[0] != 0xF0 || buffer[buffer.len() - 1] != 0xF7 {
            Err(Error::InvalidData)
        } else {
            Ok(())
        }
    }
    pub fn from_data_unchecked(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }
    pub fn from_data(buffer: &'a [u8]) -> Result<Self> {
        match Self::validate_data(buffer) {
            Ok(()) => Ok(Self::from_data_unchecked(buffer)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> Sysex7BytesBuilder<'a> {
    fn grow(&mut self) {
        if let Ok(buffer) = &self.0 {
            if buffer.len() < self.1 + 1 {
                self.0 = Err(Error::BufferOverflow);
            } else {
                self.1 += 1;
            }
        }
    }
    fn new(buffer: &'a mut [u8]) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{RandomBuffer, Truncate};
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
            Sysex7BytesBorrowed::from_data(&[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn from_data_missing_end() {
        assert_eq!(
            Sysex7BytesBorrowed::from_data(&[
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
            let message = Sysex7BytesBorrowed::from_data(&data).unwrap();
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
