use crate::{
    ci::{helpers as ci_helpers, DeviceId},
    error::Error,
    message::{sysex, system_exclusive_7bit as sysex7, system_exclusive_8bit as sysex8},
    result::Result,
    util::Encode7Bit,
    *,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidateMuidMessage<Repr: sysex::SysexMessages>(Repr);

const STATUS: u8 = 0x7E;

impl<'a> InvalidateMuidMessage<sysex8::Sysex8MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> InvalidateMuidBuilder<sysex8::Sysex8MessageGroup<'a>> {
        InvalidateMuidBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0.group()
    }
    pub fn source(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn target_muid(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(12);
        u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex8(data, STATUS)?;

        let mut payload = messages.payload();
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE + 3) else {
            return Err(Error::InvalidData);
        };

        Ok(InvalidateMuidMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> InvalidateMuidMessage<sysex7::Sysex7MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> InvalidateMuidBuilder<sysex7::Sysex7MessageGroup<'a>> {
        InvalidateMuidBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0.group()
    }
    pub fn source(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn target_muid(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(12);
        u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex7(data, STATUS)?;

        let mut payload = messages.payload();
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE + 3) else {
            return Err(Error::InvalidData);
        };

        Ok(InvalidateMuidMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

pub struct InvalidateMuidBuilder<Repr: sysex::SysexMessages> {
    source: u28,
    target_muid: u28,
    builder: Repr::Builder,
}

impl<'a> InvalidateMuidBuilder<sysex8::Sysex8MessageGroup<'a>> {
    pub fn group(&mut self, g: u4) -> &mut Self {
        self.builder.group(g);
        self
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        self.builder.stream_id(id);
        self
    }
    pub fn source(&mut self, source: u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn target_muid(&mut self, muid: u28) -> &mut Self {
        self.target_muid = muid;
        self
    }
    pub fn new(buffer: &'a mut [u32]) -> Self {
        InvalidateMuidBuilder {
            builder: sysex8::Sysex8MessageGroupBuilder::new(buffer),
            source: Default::default(),
            target_muid: Default::default(),
        }
    }
    pub fn build(&'a mut self) -> Result<InvalidateMuidMessage<sysex8::Sysex8MessageGroup<'a>>> {
        match self
            .builder
            .payload(
                ci_helpers::StandardDataIterator::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    u28::new(0xFFFFFFF),
                )
                .chain(self.target_muid.to_u7s().map(u8::from)),
            )
            .build()
        {
            Ok(messages) => Ok(InvalidateMuidMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> InvalidateMuidBuilder<sysex7::Sysex7MessageGroup<'a>> {
    pub fn group(&mut self, g: u4) -> &mut Self {
        self.builder.group(g);
        self
    }
    pub fn source(&mut self, source: u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn target_muid(&mut self, muid: u28) -> &mut Self {
        self.target_muid = muid;
        self
    }
    pub fn new(buffer: &'a mut [u32]) -> Self {
        InvalidateMuidBuilder {
            builder: sysex7::Sysex7MessageGroupBuilder::new(buffer),
            source: Default::default(),
            target_muid: Default::default(),
        }
    }
    pub fn build(&'a mut self) -> Result<InvalidateMuidMessage<sysex7::Sysex7MessageGroup<'a>>> {
        match self
            .builder
            .payload(
                ci_helpers::StandardDataIterator::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    u28::new(0xFFFFFFF),
                )
                .map(u7::new)
                .chain(self.target_muid.to_u7s()),
            )
            .build()
        {
            Ok(messages) => Ok(InvalidateMuidMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        message::system_exclusive_7bit as sysex7, message::system_exclusive_8bit as sysex8,
        util::debug,
    };

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(
                InvalidateMuidMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                    .group(u4::new(0x7))
                    .stream_id(0x4A)
                    .source(u28::new(3767028))
                    .target_muid(u28::new(226028650))
                    .build()
                    .unwrap()
                    .data(),
            ),
            debug::Data(&[
                0x571E_4A7E,
                0x7F0D_7E01,
                0x7475_6501,
                0x7F7F_7F7F,
                0x5735_4A6A,
                0x5863_6B00,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(
                InvalidateMuidMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                    .group(u4::new(0x7))
                    .source(u28::new(3767028))
                    .target_muid(u28::new(226028650))
                    .build()
                    .unwrap()
                    .data(),
            ),
            debug::Data(&[
                0x3716_7E7F,
                0x0D7E_0174,
                0x3726_7565,
                0x017F_7F7F,
                0x3735_7F6A,
                0x5863_6B00,
            ]),
        );
    }

    #[test]
    fn target_muid_sysex8() {
        assert_eq!(
            InvalidateMuidMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x571E_4A7E,
                0x7F0D_7E01,
                0x7475_6501,
                0x7F7F_7F7F,
                0x5735_4A6A,
                0x5863_6B00,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .target_muid(),
            u28::new(226028650),
        )
    }

    #[test]
    fn target_muid_sysex7() {
        assert_eq!(
            InvalidateMuidMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3716_7E7F,
                0x0D7E_0174,
                0x3726_7565,
                0x017F_7F7F,
                0x3735_7F6A,
                0x5863_6B00,
            ])
            .unwrap()
            .target_muid(),
            u28::new(226028650),
        )
    }
}
