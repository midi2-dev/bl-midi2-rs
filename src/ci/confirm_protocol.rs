use crate::{
    ci::{helpers as ci_helpers, DeviceId},
    error::Error,
    message::{sysex, system_exclusive_7bit as sysex7, system_exclusive_8bit as sysex8},
    result::Result,
    util::{Encode7Bit, Truncate},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfirmProtocolMessage<Repr: sysex::SysexMessages>(Repr);

const STATUS: u8 = 0x15;

impl<'a> ConfirmProtocolMessage<sysex8::Sysex8MessageGroup<'a>> {
    pub fn builder(
        buffer: &'a mut [u32],
    ) -> ConfirmProtocolBuilder<sysex8::Sysex8MessageGroup<'a>> {
        ConfirmProtocolBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        self.0.group()
    }
    pub fn source(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        ux::u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn destination(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(8);
        ux::u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn authority_level(&self) -> ux::u7 {
        let mut payload = self.0.payload();
        payload
            .nth(ci_helpers::STANDARD_DATA_SIZE)
            .unwrap()
            .truncate()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex8(data, STATUS)?;
        let mut payload = messages.payload();
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE) else {
            return Err(Error::InvalidData);
        };
        let None = payload.next() else {
            return Err(Error::InvalidData);
        };
        Ok(ConfirmProtocolMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> ConfirmProtocolMessage<sysex7::Sysex7MessageGroup<'a>> {
    pub fn builder(
        buffer: &'a mut [u32],
    ) -> ConfirmProtocolBuilder<sysex7::Sysex7MessageGroup<'a>> {
        ConfirmProtocolBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        self.0.group()
    }
    pub fn source(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        ux::u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn destination(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(8);
        ux::u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn authority_level(&self) -> ux::u7 {
        let mut payload = self.0.payload();
        payload.nth(ci_helpers::STANDARD_DATA_SIZE).unwrap()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex7(data, STATUS)?;
        let mut payload = messages.payload();
        let Some(_) = payload.nth(ci_helpers::STANDARD_DATA_SIZE) else {
            return Err(Error::InvalidData);
        };
        let None = payload.next() else {
            return Err(Error::InvalidData);
        };
        Ok(ConfirmProtocolMessage(messages))
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
}

pub struct ConfirmProtocolBuilder<Repr: sysex::SysexMessages> {
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
    builder: Repr::Builder,
}

impl<'a> ConfirmProtocolBuilder<sysex8::Sysex8MessageGroup<'a>> {
    pub fn group(&mut self, g: ux::u4) -> &mut Self {
        self.builder.group(g);
        self
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        self.builder.stream_id(id);
        self
    }
    pub fn source(&mut self, source: ux::u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn destination(&mut self, dest: ux::u28) -> &mut Self {
        self.destination = dest;
        self
    }
    pub fn authority_level(&mut self, auth: ux::u7) -> &mut Self {
        self.authority_level = auth;
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        ConfirmProtocolBuilder {
            builder: sysex8::Sysex8MessageGroupBuilder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            authority_level: Default::default(),
        }
    }
    pub fn build(&'a mut self) -> Result<ConfirmProtocolMessage<sysex8::Sysex8MessageGroup<'a>>> {
        match self
            .builder
            .payload(
                ci_helpers::StandardDataIterator::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    self.destination,
                )
                .chain([self.authority_level.into()].iter().copied()),
            )
            .build()
        {
            Ok(messages) => Ok(ConfirmProtocolMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> ConfirmProtocolBuilder<sysex7::Sysex7MessageGroup<'a>> {
    pub fn group(&mut self, g: ux::u4) -> &mut Self {
        self.builder.group(g);
        self
    }
    pub fn source(&mut self, source: ux::u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn destination(&mut self, dest: ux::u28) -> &mut Self {
        self.destination = dest;
        self
    }
    pub fn authority_level(&mut self, auth: ux::u7) -> &mut Self {
        self.authority_level = auth;
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        ConfirmProtocolBuilder {
            builder: sysex7::Sysex7MessageGroupBuilder::new(buffer),
            destination: Default::default(),
            source: Default::default(),
            authority_level: Default::default(),
        }
    }
    pub fn build(&'a mut self) -> Result<ConfirmProtocolMessage<sysex7::Sysex7MessageGroup<'a>>> {
        match self
            .builder
            .payload(
                ci_helpers::StandardDataIterator::new(
                    DeviceId::MidiPort,
                    STATUS,
                    self.source,
                    self.destination,
                )
                .map(ux::u7::new)
                .chain([self.authority_level].iter().copied()),
            )
            .build()
        {
            Ok(messages) => Ok(ConfirmProtocolMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::debug;

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(
                ConfirmProtocolMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 8])
                    .group(ux::u4::new(0xF))
                    .stream_id(0x76)
                    .source(ux::u28::new(11629215))
                    .destination(ux::u28::new(225493351))
                    .authority_level(ux::u7::new(0x27))
                    .build()
                    .unwrap()
                    .data(),
            ),
            debug::Data(&[
                0x5F1E_767E,
                0x7F0D_1501,
                0x1F65_4505,
                0x6702_436B,
                0x5F32_7627,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(
                ConfirmProtocolMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 8])
                    .group(ux::u4::new(0xF))
                    .source(ux::u28::new(11629215))
                    .destination(ux::u28::new(225493351))
                    .authority_level(ux::u7::new(0x27))
                    .build()
                    .unwrap()
                    .data(),
            ),
            debug::Data(&[
                0x3F16_7E7F,
                0x0D15_011F,
                0x3F26_6545,
                0x0567_0243,
                0x3F32_6B27,
                0x0000_0000
            ]),
        );
    }

    #[test]
    fn sysx8_from_data() {
        assert!(
            ConfirmProtocolMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x5F1E_767E,
                0x7F0D_1501,
                0x1F65_4505,
                0x6702_436B,
                0x5F32_7627,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .is_ok()
        )
    }

    #[test]
    fn sysex7_from_data() {
        assert!(
            ConfirmProtocolMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3F16_7E7F,
                0x0D15_011F,
                0x3F26_6545,
                0x0567_0243,
                0x3F32_6B27,
                0x0000_0000
            ])
            .is_ok()
        );
    }

    #[test]
    fn sysex8_group() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x5F1E_767E,
                0x7F0D_1501,
                0x1F65_4505,
                0x6702_436B,
                0x5F32_7627,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .group(),
            ux::u4::new(0xF),
        );
    }

    #[test]
    fn sysex8_source() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x5F1E_767E,
                0x7F0D_1501,
                0x1F65_4505,
                0x6702_436B,
                0x5F32_7627,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .source(),
            ux::u28::new(11629215),
        );
    }

    #[test]
    fn sysex8_destination() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x5F1E_767E,
                0x7F0D_1501,
                0x1F65_4505,
                0x6702_436B,
                0x5F32_7627,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .destination(),
            ux::u28::new(225493351),
        );
    }

    #[test]
    fn sysex8_authority_level() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x5F1E_767E,
                0x7F0D_1501,
                0x1F65_4505,
                0x6702_436B,
                0x5F32_7627,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .authority_level(),
            ux::u7::new(0x27),
        );
    }

    #[test]
    fn sysex7_group() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3F16_7E7F,
                0x0D15_011F,
                0x3F26_6545,
                0x0567_0243,
                0x3F32_6B27,
                0x0000_0000
            ])
            .unwrap()
            .group(),
            ux::u4::new(0xF),
        );
    }

    #[test]
    fn sysex7_source() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3F16_7E7F,
                0x0D15_011F,
                0x3F26_6545,
                0x0567_0243,
                0x3F32_6B27,
                0x0000_0000
            ])
            .unwrap()
            .source(),
            ux::u28::new(11629215),
        );
    }

    #[test]
    fn sysex7_destination() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3F16_7E7F,
                0x0D15_011F,
                0x3F26_6545,
                0x0567_0243,
                0x3F32_6B27,
                0x0000_0000
            ])
            .unwrap()
            .destination(),
            ux::u28::new(225493351),
        );
    }

    #[test]
    fn sysex7_authority_level() {
        assert_eq!(
            ConfirmProtocolMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3F16_7E7F,
                0x0D15_011F,
                0x3F26_6545,
                0x0567_0243,
                0x3F32_6B27,
                0x0000_0000
            ])
            .unwrap()
            .authority_level(),
            ux::u7::new(0x27),
        );
    }
}
