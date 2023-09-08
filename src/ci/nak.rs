use crate::{
    ci::{helpers as ci_helpers, DeviceId, SYSEX_END},
    error::Error,
    message::system_exclusive_8bit as sysex8,
    result::Result,
    *,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NakMessage<'a, Repr: 'a>(Repr, core::marker::PhantomData<&'a u8>)
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>;

const STATUS: u8 = 0x7F;
const SIZE: usize = ci_helpers::STANDARD_DATA_SIZE + 1;

impl<'a, Repr> NakMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    pub fn source(&self) -> u28 {
        ci_helpers::source_from_payload(self.0.payload())
    }
    pub fn destination(&self) -> u28 {
        ci_helpers::destination_from_payload(self.0.payload())
    }
    pub fn device_id(&self) -> DeviceId {
        let mut payload = self.0.payload();
        ci_helpers::device_id_from_u8(payload.nth(2).unwrap()).unwrap()
    }
}

impl<'a, Repr> Message<'a> for NakMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    fn data(&self) -> &'a [u32] {
        self.0.data()
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        NakMessage(Repr::from_data_unchecked(data), Default::default())
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        let messages = ci_helpers::validate_sysex::<Repr>(buffer, STATUS)?;
        let mut payload = messages.payload();
        let Some(SYSEX_END) = payload.nth(SIZE - 1) else {
            return Err(Error::InvalidData);
        };
        let None = payload.next() else {
            return Err(Error::InvalidData);
        };
        Ok(())
    }
}

impl<'a, Repr> Buildable<'a> for NakMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    type Builder = NakBuilder<'a, Repr>;
}

impl<'a, Repr> GroupedMessage<'a> for NakMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    fn group(&self) -> u4 {
        self.0.group()
    }
}

impl<'a> StreamedMessage<'a> for NakMessage<'a, sysex8::Sysex8MessageGroup<'a>> {
    fn stream_id(&self) -> u8 {
        self.0.stream_id()
    }
}

pub struct NakBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    device_id: DeviceId,
    source: u28,
    destination: u28,
    builder: Repr::Builder,
}

impl<'a, Repr> NakBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
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
}

impl<'a, Repr> Builder<'a> for NakBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    type Message = NakMessage<'a, Repr>;
    fn new(buffer: &'a mut [u32]) -> Self {
        NakBuilder {
            builder: Repr::Builder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            device_id: DeviceId::MidiPort,
        }
    }
    fn build(self) -> Result<Self::Message> {
        match self
            .builder
            .payload(ci_helpers::StandardDataIterator::<'a, Repr>::new(
                self.device_id,
                STATUS,
                self.source,
                self.destination,
            ))
            .build()
        {
            Ok(messages) => Ok(NakMessage(messages, Default::default())),
            Err(e) => Err(e),
        }
    }
}

impl<'a, Repr> GroupedBuilder<'a> for NakBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    fn group(mut self, g: u4) -> Self {
        self.builder = self.builder.group(g);
        self
    }
}

impl<'a> StreamedBuilder<'a> for NakBuilder<'a, sysex8::Sysex8MessageGroup<'a>> {
    fn stream_id(mut self, id: u8) -> Self {
        self.builder = self.builder.stream_id(id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        message::{system_exclusive_7bit as sysex7, system_exclusive_8bit as sysex8},
        util::{debug, random_buffer},
    };

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
