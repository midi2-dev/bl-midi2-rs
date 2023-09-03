use crate::{
    ci::{helpers as ci_helpers, DeviceId, SYSEX_END},
    error::Error,
    message::{sysex, system_exclusive_7bit as sysex7, system_exclusive_8bit as sysex8},
    result::Result,
    util::Encode7Bit,
    *,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NakMessage<Repr: sysex::SysexMessages>(Repr);

const STATUS: u8 = 0x7F;
const SIZE: usize = ci_helpers::STANDARD_DATA_SIZE + 1;

impl<'a> NakMessage<sysex8::Sysex8MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> NakBuilder<sysex8::Sysex8MessageGroup<'a>> {
        NakBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
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
    pub fn destination(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(8);
        u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn device_id(&self) -> DeviceId {
        let mut payload = self.0.payload();
        ci_helpers::device_id_from_u8(payload.nth(2).unwrap()).unwrap()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex8(data, STATUS)?;
        let mut payload = messages.payload();
        let Some(SYSEX_END) = payload.nth(SIZE - 1) else {
            return Err(Error::InvalidData);
        };
        let None = payload.next() else {
            return Err(Error::InvalidData);
        };
        Ok(NakMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> NakMessage<sysex7::Sysex7MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> NakBuilder<sysex7::Sysex7MessageGroup<'a>> {
        NakBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
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
    pub fn destination(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(8);
        u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn device_id(&self) -> DeviceId {
        let mut payload = self.0.payload();
        ci_helpers::device_id_from_u8(payload.nth(2).unwrap()).unwrap()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex7(data, STATUS)?;
        let mut payload = messages.payload();
        let Some(SYSEX_END) = payload.nth(SIZE - 1) else {
            return Err(Error::InvalidData);
        };
        let None = payload.next() else {
            return Err(Error::InvalidData);
        };
        Ok(NakMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

pub struct NakBuilder<Repr: sysex::SysexMessages> {
    device_id: DeviceId,
    source: u28,
    destination: u28,
    builder: Repr::Builder,
}

impl<'a> NakBuilder<sysex8::Sysex8MessageGroup<'a>> {
    pub fn group(mut self, g: u4) -> Self {
        self.builder = self.builder.group(g);
        self
    }
    pub fn stream_id(mut self, id: u8) -> Self {
        self.builder = self.builder.stream_id(id);
        self
    }
    pub fn device_id(mut self, id: DeviceId) -> Self {
        self.device_id = id;
        self
    }
    pub fn source(mut self, source: u28) -> Self {
        self.source = source;
        self
    }
    pub fn destination(mut self, dest: u28) -> Self {
        self.destination = dest;
        self
    }
    pub fn new(buffer: &'a mut [u32]) -> Self {
        NakBuilder {
            builder: sysex8::Sysex8MessageGroupBuilder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            device_id: DeviceId::MidiPort,
        }
    }
    pub fn build(self) -> Result<NakMessage<sysex8::Sysex8MessageGroup<'a>>> {
        match self
            .builder
            .payload(ci_helpers::StandardDataIterator::new(
                self.device_id,
                STATUS,
                self.source,
                self.destination,
            ))
            .build()
        {
            Ok(messages) => Ok(NakMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> NakBuilder<sysex7::Sysex7MessageGroup<'a>> {
    pub fn group(mut self, g: u4) -> Self {
        self.builder = self.builder.group(g);
        self
    }
    pub fn source(mut self, source: u28) -> Self {
        self.source = source;
        self
    }
    pub fn destination(mut self, dest: u28) -> Self {
        self.destination = dest;
        self
    }
    pub fn device_id(mut self, id: DeviceId) -> Self {
        self.device_id = id;
        self
    }
    pub fn new(buffer: &'a mut [u32]) -> Self {
        NakBuilder {
            builder: sysex7::Sysex7MessageGroupBuilder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            device_id: DeviceId::MidiPort,
        }
    }
    pub fn build(self) -> Result<NakMessage<sysex7::Sysex7MessageGroup<'a>>> {
        match self
            .builder
            .payload(
                ci_helpers::StandardDataIterator::new(
                    self.device_id,
                    STATUS,
                    self.source,
                    self.destination,
                )
                .map(u7::new),
            )
            .build()
        {
            Ok(messages) => Ok(NakMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{debug, random_buffer};

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(
                NakMessage::<sysex8::Sysex8MessageGroup>::builder(&mut random_buffer::<8>())
                    .group(u4::new(0x3))
                    .stream_id(0xB2)
                    .device_id(DeviceId::Channel(u4::new(0xD)))
                    .source(u28::new(92027634))
                    .destination(u28::new(139459637))
                    .build()
                    .unwrap()
                    .data(),
            ),
            debug::Data(&[
                0x531E_B2F0,
                0x7E0D_0D7F,
                0x0172_7570,
                0x2B35_783F,
                0x5333_B242,
                0xF700_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(
                NakMessage::<sysex7::Sysex7MessageGroup>::builder(&mut random_buffer::<6>())
                    .group(u4::new(0x3))
                    .device_id(DeviceId::Channel(u4::new(0xD)))
                    .source(u28::new(92027634))
                    .destination(u28::new(139459637))
                    .build()
                    .unwrap()
                    .data(),
            ),
            debug::Data(&[
                0x3316_F07E,
                0x0D0D_7F01,
                0x3326_7275,
                0x702B_3578,
                0x3333_3F42,
                0xF700_0000,
            ]),
        );
    }

    #[test]
    fn sysex8_from_data() {
        assert!(NakMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
            0x531E_B2F0,
            0x7E0D_0D7F,
            0x0172_7570,
            0x2B35_783F,
            0x5333_B242,
            0xF700_0000,
            0x0000_0000,
            0x0000_0000,
        ])
        .is_ok());
    }

    #[test]
    fn sysex7_from_data() {
        assert!(NakMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
            0x3316_F07E,
            0x0D0D_7F01,
            0x3326_7275,
            0x702B_3578,
            0x3333_3F42,
            0xF700_0000,
        ])
        .is_ok());
    }

    #[test]
    fn device_id_sysex7() {
        assert_eq!(
            NakMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3316_F07E,
                0x0D0D_7F01,
                0x3326_7275,
                0x702B_3578,
                0x3333_3F42,
                0xF700_0000,
            ])
            .unwrap()
            .device_id(),
            DeviceId::Channel(u4::new(0xD))
        );
    }

    #[test]
    fn device_id_sysex8() {
        assert_eq!(
            NakMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x531E_B2F0,
                0x7E0D_0D7F,
                0x0172_7570,
                0x2B35_783F,
                0x5333_B242,
                0xF700_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .device_id(),
            DeviceId::Channel(u4::new(0xD))
        );
    }
}
