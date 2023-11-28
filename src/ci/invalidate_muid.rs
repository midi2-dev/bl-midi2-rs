use crate::{
    ci::{helpers, Ci, CiStandardData, DeviceId},
    message::sysex_bytes::{Sysex7BytesBorrowed, Sysex7BytesBorrowedBuilder},
    util::Encode7Bit,
    *,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidateMuidBorrowed<'a>(Sysex7BytesBorrowed<'a>);

pub struct InvalidateMuidBorrowedBuilder<'a> {
    sysex_builder: Sysex7BytesBorrowedBuilder<'a>,
    target_muid: Option<u28>,
    standard_data: CiStandardData,
}

pub trait InvalidateMuid: ByteData {
    fn builder(buffer: &mut [u8]) -> InvalidateMuidBorrowedBuilder {
        InvalidateMuidBorrowedBuilder::new(buffer)
    }

    fn target_muid(&self) -> u28 {
        u28::from_u7s(&self.byte_data()[14..18])
    }
}

impl<'a> ByteData for InvalidateMuidBorrowed<'a> {
    fn byte_data(&self) -> &[u8] {
        self.0.byte_data()
    }
}

impl<'a> InvalidateMuid for InvalidateMuidBorrowed<'a> {}

impl<'a> Ci for InvalidateMuidBorrowed<'a> {}

impl<'a> FromByteData<'a> for InvalidateMuidBorrowed<'a> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        Sysex7BytesBorrowed::validate_byte_data(buffer)?;
        helpers::validate_ci_standard_bytes(buffer)?;
        if buffer.len() < 19 {
            return Err(Error::InvalidData);
        }
        Ok(())
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        Self(Sysex7BytesBorrowed::from_byte_data_unchecked(buffer))
    }
}

impl<'a> InvalidateMuidBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            sysex_builder: Sysex7BytesBorrowedBuilder::new(buffer),
            standard_data: CiStandardData {
                sysex_sub_id2: Some(u7::new(0x7E)),
                device_id: DeviceId::FunctionBlock,
                destination: Some(u28::from_u7s(&[0x7F_u8; 4])),
                ..Default::default()
            },
            target_muid: None,
        }
    }

    pub fn target_muid(mut self, muid: u28) -> Self {
        self.target_muid = Some(muid);
        self
    }

    pub fn source(mut self, v: u28) -> Self {
        self.standard_data.source = Some(v);
        self
    }

    pub fn build(mut self) -> Result<InvalidateMuidBorrowed<'a>> {
        let Some(target) = self.target_muid else {
            return Err(Error::InvalidData);
        };

        self.sysex_builder = self.sysex_builder.payload(self.standard_data.payload()?);

        let mut target_muid_data = [u7::default(); 4];
        target.to_u7s(&mut target_muid_data[..]);
        self.sysex_builder = self.sysex_builder.payload(target_muid_data.iter().cloned());

        Ok(InvalidateMuidBorrowed(self.sysex_builder.build()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::VERSION,
        util::{debug, RandomBuffer},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            debug::ByteData(
                InvalidateMuidBorrowed::builder(&mut Bytes::random_buffer::<25>())
                    .source(u28::new(0xDAA877))
                    .target_muid(u28::new(0xABC4B9F))
                    .build()
                    .unwrap()
                    .byte_data()
            ),
            debug::ByteData(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x7E,
                VERSION.into(),
                0x77,
                0x50,
                0x6A,
                0x06,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x1F,
                0x17,
                0x71,
                0x55,
                0xF7,
            ]),
        );
    }

    #[test]
    fn target_muid() {
        assert_eq!(
            InvalidateMuidBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x7E,
                VERSION.into(),
                0x77,
                0x50,
                0x6A,
                0x06,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x1F,
                0x17,
                0x71,
                0x55,
                0xF7,
            ])
            .unwrap()
            .target_muid(),
            u28::new(0xABC4B9F)
        );
    }
}
