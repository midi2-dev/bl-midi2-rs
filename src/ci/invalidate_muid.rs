use crate::{
    ci::{helpers as ci_helpers, DeviceId},
    error::Error,
    message::system_exclusive_8bit as sysex8,
    result::Result,
    util::Encode7Bit,
    *,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidateMuidMessage<'a, Repr>(Repr, core::marker::PhantomData<&'a u8>)
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>;

const STATUS: u8 = 0x7E;

impl<'a, Repr> InvalidateMuidMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    pub fn builder(buffer: &'a mut [u32]) -> InvalidateMuidBuilder<'a, Repr> {
        InvalidateMuidBuilder::<'a, Repr>::new(buffer)
    }
    pub fn source(&self) -> u28 {
        ci_helpers::source_from_payload(self.0.payload())
    }
    pub fn target_muid(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(13);
        u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
}

impl<'a, Repr> Message<'a> for InvalidateMuidMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    fn data(&self) -> &'a [u32] {
        self.0.data()
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        InvalidateMuidMessage(
            <Repr as Message>::from_data_unchecked(data),
            Default::default(),
        )
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        let messages = ci_helpers::validate_sysex::<Repr>(buffer, STATUS)?;
        let mut payload = messages.payload();
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE + 3) else {
            return Err(Error::InvalidData);
        };
        Ok(())
    }
}

impl<'a, Repr> Buildable<'a> for InvalidateMuidMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    type Builder = InvalidateMuidBuilder<'a, Repr>;
}

impl<'a, Repr> GroupedMessage<'a> for InvalidateMuidMessage<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    fn group(&self) -> u4 {
        self.0.group()
    }
}

impl<'a> StreamedMessage<'a> for InvalidateMuidMessage<'a, sysex8::Sysex8MessageGroup<'a>> {
    fn stream_id(&self) -> u8 {
        self.0.stream_id()
    }
}

pub struct InvalidateMuidBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    source: u28,
    target_muid: u28,
    builder: Repr::Builder,
}

impl<'a, Repr> InvalidateMuidBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    pub fn source(mut self, source: u28) -> Self {
        self.source = source;
        self
    }
    pub fn target_muid(mut self, muid: u28) -> Self {
        self.target_muid = muid;
        self
    }
}

impl<'a, Repr> Builder<'a> for InvalidateMuidBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    type Message = InvalidateMuidMessage<'a, Repr>;
    fn new(buffer: &'a mut [u32]) -> Self {
        InvalidateMuidBuilder {
            builder: <Repr as Buildable<'a>>::Builder::new(buffer),
            source: Default::default(),
            target_muid: Default::default(),
        }
    }
    fn build(self) -> Result<InvalidateMuidMessage<'a, Repr>> {
        match self
            .builder
            .payload(
                ci_helpers::StandardDataIterator::<'a, Repr>::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    u28::new(0xFFFFFFF),
                )
                .chain(
                    self.target_muid
                        .to_u7s()
                        .map(u8::from)
                        .map(<<Repr as Buildable<'a>>::Builder as SysexBuilder<'a>>::Byte::from_u8),
                ),
            )
            .build()
        {
            Ok(messages) => Ok(InvalidateMuidMessage(messages, Default::default())),
            Err(e) => Err(e),
        }
    }
}

impl<'a, Repr> GroupedBuilder<'a> for InvalidateMuidBuilder<'a, Repr>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    fn group(mut self, g: u4) -> Self {
        self.builder = self.builder.group(g);
        self
    }
}

impl<'a> StreamedBuilder<'a> for InvalidateMuidBuilder<'a, sysex8::Sysex8MessageGroup<'a>> {
    fn stream_id(mut self, id: u8) -> Self {
        self.builder = self.builder.stream_id(id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        message::system_exclusive_7bit as sysex7,
        message::system_exclusive_8bit as sysex8,
        util::{debug, random_buffer},
    };

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(
                InvalidateMuidMessage::<sysex8::Sysex8MessageGroup>::builder(
                    &mut random_buffer::<8>()
                )
                .group(u4::new(0x7))
                .stream_id(0x4A)
                .source(u28::new(3767028))
                .target_muid(u28::new(226028650))
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x571E_4AF0,
                0x7E7F_0D7E,
                0x0174_7565,
                0x017F_7F7F,
                0x5737_4A7F,
                0x6A58_636B,
                0xF700_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(
                InvalidateMuidMessage::<sysex7::Sysex7MessageGroup>::builder(&mut random_buffer::<
                    10,
                >(
                ))
                .group(u4::new(0x7))
                .source(u28::new(3767028))
                .target_muid(u28::new(226028650))
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x3716_F07E,
                0x7F0D_7E01,
                0x3726_7475,
                0x6501_7F7F,
                0x3726_7F7F,
                0x6A58_636B,
                0x3731_F700,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn target_muid_sysex8() {
        assert_eq!(
            InvalidateMuidMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x571E_4AF0,
                0x7E7F_0D7E,
                0x0174_7565,
                0x017F_7F7F,
                0x5737_4A7F,
                0x6A58_636B,
                0xF700_0000,
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
                0x3716_F07E,
                0x7F0D_7E01,
                0x3726_7475,
                0x6501_7F7F,
                0x3726_7F7F,
                0x6A58_636B,
                0x3731_F700,
                0x0000_0000,
            ])
            .unwrap()
            .target_muid(),
            u28::new(226028650),
        )
    }
}
